use std::thread;
use std::time::Duration;

use embedded_hal::digital::blocking::OutputPin;
use esp_idf_hal::peripherals::Peripherals;

fn main() {
    let peripherals = Peripherals::take().unwrap();
    let mut led = peripherals.pins.gpio4.into_output().unwrap();

    loop {
        led.set_high().unwrap();
        thread::sleep(Duration::from_millis(1000));

        led.set_low().unwrap();
        thread::sleep(Duration::from_millis(1000));

        println!("HELLO WORLD!")
    }
}
