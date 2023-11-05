

#![no_std]
#![no_main]



use defmt_rtt as _;
//use lpc5500 as _;

use lpc55_hal as hal;


use embedded_hal::digital::v2::OutputPin;


#[cortex_m_rt::entry]
fn main_() -> ! {
    use lpc5500::gpio::Output;

    defmt::error!("Hello from main");

    // Initialize the device.
    let hal = unsafe { lpc5500::init() };

    // Get the Port 1 pins.
    let pins = hal.pins.PORT1;

    // Blue LED is PIO1_6.
    let mut blue = Output::new( pins.PIN6 );

    // Green LED is PIO1_7.
    let mut green = Output::new( pins.PIN7.anon() );

    // Red LED is PIO1_4.
    let mut red = Output::new( pins.PIN4 );

    defmt::info!("Created RGB LED pins");

    // Configure this delay.
    const DELAY: usize = 1000000;

    loop {
        for _ in 0..DELAY {
            unsafe { core::arch::asm!("nop"); }
        }

        blue.toggle();

        for _ in 0..DELAY {
            unsafe { core::arch::asm!("nop"); }
        }

        green.toggle();

        for _ in 0..DELAY {
            unsafe { core::arch::asm!("nop"); }
        }

        red.toggle();
    }
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop { unsafe { core::arch::asm!("wfi") } }
}
