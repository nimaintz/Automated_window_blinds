#![no_std]
#![no_main]
#![allow(unused_imports)]

//receives the information
use core::panic::PanicInfo;
use core::str::from_utf8;
use byte_slice_cast::AsByteSlice;
use cyw43_pio::PioSpi;
use embassy_executor::Spawner;
use embassy_futures::select;
use embassy_net::tcp::TcpSocket;
use embassy_net::udp::{PacketMetadata, UdpSocket};
use embassy_net::{Config, IpAddress, IpEndpoint, Ipv4Address, Ipv4Cidr, Stack, StackResources};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::{Duration, Timer, Delay};
use embedded_hal_1::delay::DelayNs;
use embedded_hal_1::digital::OutputPin;
use embedded_io_async::Write;
use heapless::Vec;
use log::{info, warn};
use static_cell::StaticCell;
use uln2003::{ULN2003, Direction, StepperMotor, StepError};

// USB driver
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, Endpoint, InterruptHandler as USBInterruptHandler};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => USBInterruptHandler<USB>;
    // PIO interrupt for CYW SPI communication
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

const WIFI_NETWORK: &str = "heh";
const WIFI_PASSWORD: &str = "abcd1234";

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<cyw43::NetDriver<'static>>) -> ! {
    stack.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    // Start the USB logger driver
    let driver = Driver::new(peripherals.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    let mut led = Output::new(peripherals.PIN_1, Level::Low);

    // Set up GPIO pins for the 28BYJ-48
    let mut pin2 = Output::new(peripherals.PIN_2, Level::Low);
    let mut pin3 = Output::new(peripherals.PIN_3, Level::Low);
    let mut pin4 = Output::new(peripherals.PIN_4, Level::Low);
    let mut pin5 = Output::new(peripherals.PIN_5, Level::Low);

    // Initialize motor with delay
    let mut motor = ULN2003::<_,_,_,_,u32,_>::new(
        pin2,
        pin3,
        pin4,
        pin5,
        Some(Delay),
    );

    // Link CYW43 firmware
    let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");

    // Init SPI for communication with CYW43
    let pwr = Output::new(peripherals.PIN_23, Level::Low);
    let cs = Output::new(peripherals.PIN_25, Level::High);
    let mut pio = Pio::new(peripherals.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        pio.irq0,
        cs,
        peripherals.PIN_24,
        peripherals.PIN_29,
        peripherals.DMA_CH0,
    );

    // Start Wi-Fi task
    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    spawner.spawn(wifi_task(runner)).unwrap();

    // Init the device
    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

        let config = Config::dhcpv4(Default::default());
    // Generate random seed
    let seed = 0x0123_4567_89ab_cdef;

    // Init network stack
    static STACK: StaticCell<Stack<cyw43::NetDriver<'static>>> = StaticCell::new();
    static RESOURCES: StaticCell<StackResources<2>> = StaticCell::new();
    let stack = &*STACK.init(Stack::new(
        net_device,
        config,
        RESOURCES.init(StackResources::<2>::new()),
        seed,
    ));

    // Start network stack task
    spawner.spawn(net_task(stack)).unwrap();

    loop {
        // Join WPA2 access point
        // Modify WIFI_NETWORK and WIFI_PASSWORD if you're connecting to a WPA AP
        // Use `join_open` instead if you're connecting to an open AP
        match control.join_wpa2(WIFI_NETWORK, WIFI_PASSWORD).await {
            Ok(_) => break,
            Err(err) => {
                info!("join failed with status {}", err.status);
            }
        }
    }

    // Wait for DHCP (not necessary when using static IP)
    info!("waiting for DHCP...");
    while !stack.is_config_up() {
        Timer::after_millis(100).await;
    }
    info!("DHCP is now up {:?}!", stack.config_v4());

    // And now we can use it!

    // Buffers
    let mut rx_buffer = [0; 4096];
    let mut rx_metadata_buffer = [PacketMetadata::EMPTY; 3];
    let mut tx_buffer = [0; 4096];
    let mut tx_metadata_buffer = [PacketMetadata::EMPTY; 3];

    let mut buf = [0u8; 4096];

    loop {
        // Initialize UDP socket
        let mut socket = UdpSocket::new(
            stack,
            &mut rx_metadata_buffer,
            &mut rx_buffer,
            &mut tx_metadata_buffer,
            &mut tx_buffer,
        );

        info!("Starting server on UDP:1234...");

        // Bind socket to port
        if let Err(e) = socket.bind(1234) {
            warn!("accept error: {:?}", e);
            continue;
        }

        
        loop {
            match socket.recv_from(&mut buf).await {
                Ok((n, endpoint)) => {
                    
                    info!("Received from {:?}: {:?}", endpoint, from_utf8(&buf[..n]).unwrap());
                    let recv = from_utf8(&buf[..n]).unwrap().trim();
                    match recv{
                        // Rotate the motor clockwise with the button = downward
                        "roll:down" => {
                            motor.set_direction(Direction::Normal);
                                for _ in 0..1000 {
                                    motor.step().unwrap();
                                    Timer::after(Duration::from_millis(5)).await;
                                    }
                            info!("Rolled down");
                            led.toggle(); //on
                            Timer::after_millis(500).await;
                            led.toggle(); //off
                            Timer::after_millis(500).await;
                            led.toggle(); //on
                            Timer::after_millis(500).await;
                            led.toggle(); //off
                        },
                        // Rotate the motor counter-clockwise with the button = upward
                        "roll:up" => {
                            motor.set_direction(Direction::Reverse);
                                for _ in 0..1000 {
                                    motor.step().unwrap();
                                    Timer::after(Duration::from_millis(5)).await;
                                    }
                            info!("Rolled up");
                            led.toggle();
                            Timer::after_millis(500).await;
                            led.toggle();
                            Timer::after_millis(500).await;
                            led.toggle();
                            Timer::after_millis(500).await;
                            led.toggle();
                            },
                        _ => {}
                    }
                },
                Err(_) => {
                    info!("An error occurred when receiving the packet!");
                }
            }
        }    
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
