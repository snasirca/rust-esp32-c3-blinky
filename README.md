# Rust ESP32-C3 Blinky

An example Rust project that uses pin GPIO4 on an ESP32-C3 board to blink an LED and print some text to the serial bus.

To flash this project, run this command:

    cargo espflash $(ls /dev/cu.usbserial-* | head -n 1) --monitor

Or

    cargo run --release

## Additional Examples

* Demonstrates how to connect to a Wifi network
* Demonstrates how to publish an MQTT message
* Demonstrates how to go to deep sleep

## Espressif docs

* [ESP32-C3-DevKitC-02](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c3/esp32-c3-devkitc-02/user_guide.html#hardware-reference)

## Credits

This project has been heavily inspired by these projects:
* [ESP32-C3 Embassy](https://github.com/claudiomattera/esp32c3-embassy)
* [Rust on ESP32 STD demo app](https://github.com/ivmarkov/rust-esp32-std-demo#rust-on-esp32-std-demo-app)
