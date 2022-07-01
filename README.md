# Rust ESP32-C3 Blinky

An example Rust project that uses pin GPIO4 on an ESP32-C3 board to blink an LED and print some text to the serial bus.

To flash this project, run this command:

    cargo espflash /dev/cu.usbserial-220 --monitor --speed 921600

Substitute with the device name of your board.