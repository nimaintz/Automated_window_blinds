[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-24ddc0f5d75046c5622901739e7c5dd533143b0c8e959d652212380cedb1ea36.svg)](https://classroom.github.com/a/l2EJ6P3t)
# Automated Window Blinds

## Description: 
My project aims to automate window roller blinds. The blind will be controlled through wifi by another raspberry pi pico done by my colleague [Rebeca Chiorean](https://github.com/UPB-FILS-MA/project-ChioreanRebeca/blob/main/README.md). The implementation works as follows: the other pico sends a message through wifi that in turn rolls up or down the blinds and blinks a led as a confirmation that it rolled.</br>
My project uses 3D printed parts and a custom built prototype of a window to demonstrate how this would work in real life.

<!-- Describe in a few words your project idea. -->

## Hardware

<!-- Fill out this table with all the hardware components that you mght need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [Rapspberry Pi Pico WH](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | the microcontroller | [56,23 RON](https://ardushop.ro/ro/home/2819-raspberry-pi-pico-wh.html?search_query=Raspberry+Pi+Pico+WH%2C+Wireless+Headers&results=1031) |
| [Stepper Motor 28BYJ-48 5V and Driver ULN2003](https://www.hwlibre.com/en/28bj-48/)| the motor to rotate the blinds and the driver to connect it to the microcontroller | [16,97 RON](https://www.optimusdigital.ro/ro/motoare-motoare-pas-cu-pas/101-driver-uln2003-motor-pas-cu-pas-de-5-v-.html) |
| Breadboard | to place the components | [24,61 RON](https://ardushop.ro/ro/electronica/163-kit-breadboard830-65xfire-jumper-sursa-alimentare-335v.html?search_query=KIT+Breadboard830+++65xfire+jumper+++sursa+alimentare+3%2C3%2F5V&results=694) |
| Jumper wires | to conect the components | [24,61 RON](https://ardushop.ro/ro/electronica/163-kit-breadboard830-65xfire-jumper-sursa-alimentare-335v.html?search_query=KIT+Breadboard830+++65xfire+jumper+++sursa+alimentare+3%2C3%2F5V&results=694) |
| Power source | to give the motor 5V and pico 3V3 | [24,61 RON](https://ardushop.ro/ro/electronica/163-kit-breadboard830-65xfire-jumper-sursa-alimentare-335v.html?search_query=KIT+Breadboard830+++65xfire+jumper+++sursa+alimentare+3%2C3%2F5V&results=694) |
| Resistors | regulate the power supply | [12,29 RON](https://ardushop.ro/ro/electronica/212-set-rezistente-14w-600buc30-valori-10r-1m.html?search_query=SET+rezistori+&results=429) |
| Red LED | blink | [0,69 RON](https://www.optimusdigital.ro/ro/optoelectronice-led-uri/696-led-rou-de-3-mm-cu-lentile-difuze.html?search_query=led&results=819) |
| 3D printed toothed disks | connect the motor to the blinds |
| 9V adaptor | power supply |

## Links

<!-- Add a few links that got you the idea and that you think you will use for your project -->

1. [3D printed and fully automated Roller Blind Motor](https://imgur.com/a/xuQjH3z)
2. [Pico Project: Automated window blinds that open at sunrise and close at sunset](https://www.reddit.com/r/raspberrypipico/comments/wbdsz1/pico_project_automated_window_blinds_that_open_at/)
3. [28BYJ-48 Stepper Motor with Raspberry PI Pico](https://www.youtube.com/watch?v=VM3S9CiyPzY&t=2s)
4. [3D printed toothed disks](https://www.instructables.com/DIY-Motorized-WiFi-Roller-Blind-ESP8266-Blynk/)

