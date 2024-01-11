

#![no_std]
#![no_main]

#![feature(generic_const_exprs)]

use defmt_rtt as _;
//use lpc5500 as _;
use cortex_m as _;
use lpc5500::system::user::UserSystemControl;



lpc5500::entry!(main);

#[inline(never)]
fn main(hal: lpc5500::Peripherals) -> ! {
    use lpc5500::gpio::Output;
    use lpc5500::security::random;

    defmt::error!("Hello from main");

    // Get the user interface and configure the device.
    let mut user = hal.user;
    sysconfig(&mut user);

    // Get the Port 1 pins.
    let pins = hal.pins.PORT1;

    // Collect the pins in an array.
    let mut leds = [
        // Red LED is PIO1_4.
        Output::new( pins.PIN4.anon() ),
        // Blue LED is PIO1_6.
        Output::new( pins.PIN6.anon() ),
        // Green LED is PIO1_7.
        Output::new( pins.PIN7.anon() ),
    ];

    defmt::info!("Testing random numbers {:X} | {:X} | {:X}", random(), random(), random());

    coprocessor( hal.powerquad );

    defmt::info!("Coprocessor ended");


    // Configure this set of frequencies and repeats.
    let sequence = [
        (lpc5500::system::user::ClockSource::FRO96Mhz, 500000, 60),
        (lpc5500::system::user::ClockSource::FRO12Mhz, 100000,  5),
        (lpc5500::system::user::ClockSource::FRO1Mhz , 100000,  3),
    ];

    loop {
        // Loop over all clock states.
        for (source, delay, reps) in sequence.iter() {
            // Set the frequency of the device.
            user.setclock(*source);

            defmt::info!("Set clock source {}", source);

            for _ in 0..*reps {
                // Loop over the LEDs.
                for led in &mut leds {
                    // Wait the delay given.
                    for _ in 0..*delay { unsafe { core::arch::asm!("nop", options(nomem, nostack)); } }

                    // Toggle the LED.
                    led.toggle();
                }
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

    defmt::info!("Registered PQ handler = 0x{:08X}", unsafe { core::ptr::read_volatile((0x20000000 + (4 * (16 + 57))) as *const u32) });

    // Select the divisors.
    let div0 = 2.0;
    let div1 = 3.0;

    // Execute a function and read the value.
    let op1 = cp1.exp( div1 );
    let op0 = cp0.exp( div0 );

    // Finish both instructions.
    let res1 = op1.finish();
    let res0 = op0.finish();

    defmt::info!("exp({}) = {}", div0, res0);
    defmt::info!("exp({}) = {}", div1, res1);

    // Execute divisions.
    let op0 = cp0.div(2.0, 1.0);
    let op1 = cp1.div(1.0, 2.0);

    // Finish both instructions.
    let res0 = op0.finish();
    let res1 = op1.finish();

    defmt::info!("2.0 / 1.0 = {}\n1.0 / 2.0 = {}", res0, res1);

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
    let _ = engine.add(&a, &b, &mut c);

    unsafe { core::arch::asm!("wfe", options(nomem, nostack)) };


    // Launch another operation to see if this conflicts.
    let r = cp0.exp(7.0);

    defmt::info!("Waiting for matrix addition to finish");
    let r7 = r.finish();
    engine.finish();
    //defmt::info!("Matrix addition is done.");

    //defmt::info!("exp(7) = {}", r7);
    //defmt::info!("A + B = {}", c);
}



#[inline(never)]
fn sysconfig(user: &mut lpc5500::system::user::UserSystemControl) {
    use lpc5500::system::user::{
        Acceleration, BufferUsage, ClockSource,
    };

    // FLASH acceleration configuration.
    const ACCELERATION: Acceleration = Acceleration {
        dbuf: BufferUsage::All,
        ibuf: BufferUsage::All,
        prefetch: true,
    };

    // Set the clock state.
    user.setclock(ClockSource::FRO96Mhz);
    let (frequency, source) = user.main.getclock();
    defmt::info!("Device is using {} as main clock [{} Hz]", source, frequency);

    // Configure Flash acceleration.
    user.flash.acceleration( Some( ACCELERATION ) );
}




#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop { unsafe { core::arch::asm!("wfi") } }
}
