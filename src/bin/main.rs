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
    gpio::{Input, InputConfig, Level, Output, OutputConfig},
    main,
};
use esp_hal::time::{Duration, Instant};
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
    // let peripherals = esp_hal::init(config);
    let peripherals = esp_hal::init(esp_hal::Config::default());

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


    let mut led = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());
    let button = Input::new(peripherals.GPIO0, InputConfig::default());

    // led.set_high();
    //
    // let delay = Delay::new();

    loop {
        if button.is_high() {
            led.set_high();
        } else {
            led.set_low();
            info!("Роберт любит лизу!");
            let delay_start = Instant::now();
            while delay_start.elapsed() < Duration::from_millis(1500) {}
        }

        // led.toggle();
        // delay.delay_millis(500);
        // info!("Роберт любит лизу!");
        // let delay_start = Instant::now();
        // while delay_start.elapsed() < Duration::from_millis(500) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.2.2/examples
}
