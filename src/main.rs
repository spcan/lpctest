

#![no_std]
#![no_main]



use defmt_rtt as _;
//use lpc5500 as _;

use lpc55_hal as _;



#[cortex_m_rt::entry]
fn main_() -> ! {
    use lpc5500::gpio::Output;
    use lpc5500::security::random;

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

    defmt::info!("Testing random numbers {:X} | {:X} | {:X}", random(), random(), random());

    coprocessor( hal.powerquad );


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


#[inline(never)]
fn coprocessor(pq: lpc5500::powerquad::PowerQuad) {
    use lpc5500::powerquad::coprocessor::traits::*;

    // Initialize the PowerQuad.
    let (mut cp0, mut cp1) = pq.init();

    let reset  = unsafe { core::ptr::read_volatile((0x50000000 + 0x108) as *const u32) };
    let enable = unsafe { core::ptr::read_volatile((0x50000000 + 0x208) as *const u32) };

    defmt::info!("PQ unreset: {} | PQ enabled: {}", ((reset >> 19) & 1) == 0, ((enable >> 19) & 1) == 1);

    core::sync::atomic::compiler_fence( core::sync::atomic::Ordering::SeqCst );

    // Select the divisors.
    let div0 = 2.0;
    let div1 = 3.0;

    core::sync::atomic::compiler_fence( core::sync::atomic::Ordering::SeqCst );

    defmt::info!("Going into the unknown");

    core::sync::atomic::compiler_fence( core::sync::atomic::Ordering::SeqCst );

    // Execute a function and read the value.
    let op1 = cp1.exp( div1 );
    let op0 = cp0.exp( div0 );

    core::sync::atomic::compiler_fence( core::sync::atomic::Ordering::SeqCst );

    defmt::info!("The call of the void did not explode");

    core::sync::atomic::compiler_fence( core::sync::atomic::Ordering::SeqCst );

    // Finish both instructions.
    let res1 = op1.finish();
    let res0 = op0.finish();

    core::sync::atomic::compiler_fence( core::sync::atomic::Ordering::SeqCst );

    defmt::info!("exp({}) = {}", div0, res0);
    defmt::info!("exp({}) = {}", div1, res1);
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
