

#![no_std]
#![no_main]

#![feature(generic_const_exprs)]

use defmt_rtt as _;
//use lpc5500 as _;
use cortex_m as _;



lpc5500::entry!(main);

#[inline(never)]
fn main(hal: lpc5500::Peripherals) -> ! {
    use lpc5500::gpio::Output;
    use lpc5500::security::random;

    defmt::error!("Hello from main");

    // Get the user interface.
    let mut user = hal.user;

    // Get the clock state.
    let (frequency, source) = user.main.getclock();
    defmt::debug!("Device is using {} as main clock [{} Hz]", source, frequency);

    user.setclock(lpc5500::system::user::ClockSource::FRO1Mhz);
    let (frequency, source) = user.main.getclock();
    defmt::debug!("Device is using {} as main clock [{} Hz]", source, frequency);

    // Configure Flash acceleration.
    let cfg = accelcfg();
    user.flash.acceleration( Some( cfg ) );

    defmt::info!("Using Flash acceleration: {}", cfg);

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

    defmt::info!("Coprocessor ended");


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

            defmt::info!("Set clock source {}", source);

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
    use lpc5500::powerquad::{
        coprocessor::traits::*,
        engine::matrix::{
            MatrixTrait, TypedMatrix,
        },
    };

    // Initialize the PowerQuad.
    let (mut cp0, mut cp1, mut engine) = pq.init();

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

    core::sync::atomic::compiler_fence( core::sync::atomic::Ordering::SeqCst );

    // Finish both instructions.
    let res1 = op1.finish();
    let res0 = op0.finish();

    core::sync::atomic::compiler_fence( core::sync::atomic::Ordering::SeqCst );

    defmt::info!("exp({}) = {}", div0, res0);
    defmt::info!("exp({}) = {}", div1, res1);

    // Create a matrix.
    let mut eye: TypedMatrix<5, 5> = TypedMatrix::eye();

    // Modify the 3,4 element
    eye[3][4] = 5.0;
    eye[0][2] = 5.0;
    eye[1][2] = 5.0;
    eye[2][2] = 5.0;
    eye[3][2] = 5.0;

    defmt::info!("Modified the identity Matrix:\n{}", eye);


    // Create three matrices.
    let a: TypedMatrix<5, 5> = TypedMatrix::zeroes();
    let b: TypedMatrix<5, 5> = TypedMatrix::eye();
    let mut c: TypedMatrix<5, 5> = TypedMatrix::zeroes();

    // Add A + B = C.
    defmt::info!("Launching matrix addition.");
    //let _ = engine.add(&a, &b, &mut c);


    // Launch another operation to see if this conflicts.
    let r = cp0.exp(7.0);

    defmt::info!("Waiting for amtrix addition to finish");
    let r7 = r.finish();
    //engine.finish();
    defmt::info!("Matrix addition is done.");

    defmt::info!("exp(7) = {}", r7);
    defmt::info!("A + B = {}", c);
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
