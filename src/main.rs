#![no_std]
#![no_main]

use core::convert::Infallible;

use log::error;
use log::info;

use embassy_executor::Spawner;
use embassy_time::Timer;

use esp_backtrace as _;

use esp_hal::{
    gpio::{Io, Level, Output},
    prelude::*,
    timer::timg::TimerGroup,
};

mod logging;
use self::logging::setup as setup_logging;

#[embassy_executor::task]
async fn one_second_task() {
    let mut count = 0;
    loop {
        info!("Spawn Task Count: {}", count);
        count += 1;
        Timer::after_millis(1_000).await;
    }
}

#[main]
async fn main(spawner: Spawner) {
    setup_logging();

    if let Err(error) = main_fallible(&spawner).await {
        error!("Error while running firmware: {error:?}");
    }
}

/// Main task that can return an error
async fn main_fallible(spawner: &Spawner) -> Result<(), Error> {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    spawner.spawn(one_second_task()).unwrap();

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = Output::new(io.pins.gpio4, Level::High);

    let mut count = 0;
    loop {
        info!("Main Task Count: {}", count);
        led.toggle();
        count += 1;
        Timer::after_millis(5_000).await;
    }
}

#[derive(Debug)]
enum Error {
    /// An impossible error existing only to satisfy the type system
    Impossible(Infallible),
}
