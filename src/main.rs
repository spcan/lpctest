

#![no_std]
#![no_main]



use defmt_rtt as _;
//use lpc5500 as _;

use lpc55_hal as hal;



#[cortex_m_rt::entry]
fn main_() -> ! {
    use lpc5500::gpio::Output;

    defmt::error!("Hello from main");

    // Initialize the device.
    let hal = unsafe { lpc5500::init() };

    // Get the user interface.
    let mut user = hal.user;

    // Get the clock state.
    let (frequency, source) = user.main.getclock();

    defmt::debug!("Device is using {} as main clock [{} Hz]", source, frequency);

    // Configure Flash acceleration.
    user.flash.acceleration( Some( accelcfg() ) );

    let fmccr = unsafe { core::ptr::read_volatile( (0x50000000 + 0x400) as *mut u32 ) };
    defmt::info!("FMCCR {:b}", fmccr);

    // Get the Port 1 pins.
    let pins = hal.pins.PORT1;

    // Blue LED is PIO1_6.
    let mut blue = Output::new( pins.PIN6 );

    // Green LED is PIO1_7.
    let mut green = Output::new( pins.PIN7.anon() );

    // Red LED is PIO1_4.
    let mut red = Output::new( pins.PIN4 );

    defmt::info!("Created RGB LED pins");



    // Configure this set of frequencies and repeats.
    let sequence = [
        (lpc5500::system::user::ClockSource::FRO96Mhz, 100000, 5),
        (lpc5500::system::user::ClockSource::FRO12Mhz, 100000, 5),
        (lpc5500::system::user::ClockSource::FRO1Mhz , 100000, 5),
    ];

    loop {
        // Loop over all states.
        for (source, delay, reps) in sequence.iter() {
            // Set the frequency of the device.
            user.setclock(*source);

            // Read the MUX values and AHBDIV.
            /*
            let a = unsafe { core::ptr::read_volatile( (0x50000000 + 0x280) as *const u32 ) };
            let b = unsafe { core::ptr::read_volatile( (0x50000000 + 0x284) as *const u32 ) };
            let d = unsafe { core::ptr::read_volatile( (0x50000000 + 0x380) as *const u32 ) };

            defmt::info!("Clock {} > MAINSELA : {} | MAINSELB : {} [divisor {}]", *source, a, b, d & 0xFF);
            */

            // Number of repetitions.
            for _ in 0..*reps {
                // Toggle the lights.
                for _ in 0..*delay {
                    unsafe { core::arch::asm!("nop", options(nostack, nomem)); }
                }
        
                blue.toggle();
        
                for _ in 0..*delay {
                    unsafe { core::arch::asm!("nop", options(nostack, nomem)); }
                }
        
                green.toggle();
        
                for _ in 0..*delay {
                    unsafe { core::arch::asm!("nop", options(nostack, nomem)); }
                }
        
                red.toggle();
            }
        }
    }
}



fn accelcfg() -> lpc5500::system::user::Acceleration {
    use lpc5500::system::user::{
        Acceleration, BufferUsage,
    };

    Acceleration {
        dbuf: BufferUsage::All,
        ibuf: BufferUsage::All,
        prefetch: true,
    }
}



#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop { unsafe { core::arch::asm!("wfi") } }
}
