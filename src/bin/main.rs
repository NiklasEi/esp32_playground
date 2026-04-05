#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use core::panic::PanicInfo;
use embedded_hal::delay::DelayNs;
use esp_hal::delay::Delay;
use esp_hal::gpio::{DriveMode, Flex, InputConfig, OutputConfig, Pull};
use esp_hal::main;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    esp_println::println!("{}", info);
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let mut dht_pin = Flex::new(peripherals.GPIO4);
    dht_pin.apply_input_config(&InputConfig::default().with_pull(Pull::Up));
    dht_pin.apply_output_config(&OutputConfig::default().with_drive_mode(DriveMode::OpenDrain));
    dht_pin.set_input_enable(true);
    dht_pin.set_output_enable(true);
    dht_pin.set_high();

    let mut delay = Delay::new();

    // Let the sensor settle after power-on
    delay.delay_ms(1000);

    loop {
        match dht_sensor::dht11::blocking::read(&mut delay, &mut dht_pin) {
            Ok(reading) => {
                esp_println::println!(
                    "Temperature: {}°C, Humidity: {}%",
                    reading.temperature,
                    reading.relative_humidity
                );
            }
            Err(e) => {
                esp_println::println!("DHT11 read error: {:?}", e);
            }
        }

        delay.delay_ms(2000);
    }
}
