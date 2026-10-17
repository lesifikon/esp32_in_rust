#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::clock::CpuClock;
use esp_hal::{
    delay::Delay,
    gpio::{Io, Level, Output, OutputConfig, Input, InputConfig},
    time::{Duration, Instant},
    main,
};
use log::{error, info};

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    error!("{}", panic_info);
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.4.0
    // generator parameters: -o esp32 -o esp32-wroom-32e -o unstable-hal -o log -o neovim-rustaceanvim

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // The following pins are used to bootstrap the chip. They are available
    // for use, but check the datasheet of the module for more information on them.
    // - GPIO0
    // - GPIO2
    // - GPIO5
    // - GPIO12
    // - GPIO15
    // These GPIO pins are in use by some feature of the module and should not be used.
    let _gpio6 = peripherals.GPIO6;
    let _gpio7 = peripherals.GPIO7;
    let _gpio8 = peripherals.GPIO8;
    let _gpio9 = peripherals.GPIO9;
    let _gpio10 = peripherals.GPIO10;
    let _gpio11 = peripherals.GPIO11;
    let _gpio16 = peripherals.GPIO16;
    let _gpio20 = peripherals.GPIO20;

    let mut red_led = Output::new(peripherals.GPIO26, Level::Low, OutputConfig::default());
    let mut green_led = Output::new(peripherals.GPIO27, Level::Low, OutputConfig::default());
    let button = Input::new(peripherals.GPIO21, InputConfig::default());

    let delay = Delay::new();

    loop {
        if button.is_high() {
            // нажата
            red_led.set_high();
            green_led_pin(&red_led, &mut green_led, delay);
            // delay.delay_millis(500);
        } else {
            red_led.set_low();
            // не нажата
            green_led_pin(&red_led, &mut green_led, delay);
            // delay.delay_millis(500);
        }
    }
}

fn green_led_pin(red_led: &Output<'_>, green_led: &mut Output<'_>, delay: Delay) {
    delay.delay_millis(500);
    if red_led.is_set_high() {
        green_led.set_high();
    } else {
        green_led.set_low();
    }
    delay.delay_millis(500);
}
