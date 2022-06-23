//!
//! Firmware demonstration for an Adafruit Feather RP2040 connected to an
//! SSD1306 OLED display and an MPU6050 6-axis IMU via STEMMA QT cables.
//!

#![no_std]
#![no_main]

use adafruit_feather_rp2040::XOSC_CRYSTAL_FREQ;
use core::fmt::Write;
use cortex_m_rt::entry;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::{
    image::{Image, ImageRaw},
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    prelude::{Point, Primitive, Size},
    primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Triangle},
    text::{Baseline, Text},
    Drawable,
};
use embedded_hal::digital::v2::OutputPin;
use embedded_time::{fixed_point::FixedPoint, rate::Extensions};
use hal::{pac, Clock};
use heapless::String;
use mpu6050::Mpu6050;
use panic_halt as _;
use rp2040_hal as hal;
use ssd1306::{
    mode::DisplayConfig, rotation::DisplayRotation, size::DisplaySize128x64, I2CDisplayInterface,
    Ssd1306,
};

/// Entry point to our bare-metal application.
#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure two pins as being I2C, not GPIO. From the datasheet for the Feather RP2040
    // we see that GPIO2 and GPIO3 are the SDA and SCL pins, respectively, attached to the
    // JST SH 4-pin connector and cable we are using to connect the hardware.
    let sda_pin = pins.gpio2.into_mode::<hal::gpio::FunctionI2C>();
    let scl_pin = pins.gpio3.into_mode::<hal::gpio::FunctionI2C>();

    // Create the I2C drive, using the two pre-configured pins.
    let i2c = hal::I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        clocks.peripheral_clock,
    );

    // We are using two drivers, one for the display and one for the mpu. Since each driver
    // needs to "own" the I2C bus peripheral, we are using the shared_bus crate to easily
    // allow us to share the I2C bus between the two drivers.
    let bus = shared_bus::BusManagerSimple::new(i2c);

    // Configure the display driver
    let interface = I2CDisplayInterface::new_alternate_address(bus.acquire_i2c());
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    // Configure the mpu driver
    let mut mpu = Mpu6050::new(bus.acquire_i2c());
    mpu.init(&mut delay).unwrap();

    // Set the text style for drawing text to the display
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let draw_mode = 2;
    match draw_mode {
        0 => {
            let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64);
            let im = Image::new(&raw, Point::new(32, 0));
            im.draw(&mut display).unwrap();
        }
        1 => {
            Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
                .draw(&mut display)
                .unwrap();
            Text::with_baseline("Hello Rust!", Point::new(0, 16), text_style, Baseline::Top)
                .draw(&mut display)
                .unwrap();
        }
        2 => {
            let yoffset = 20;

            let style = PrimitiveStyleBuilder::new()
                .stroke_width(1)
                .stroke_color(BinaryColor::On)
                .build();

            // screen outline
            // default display size is 128x64 if you don't pass a _DisplaySize_
            // enum to the _Builder_ struct
            Rectangle::new(Point::new(0, 0), Size::new(127, 63))
                .into_styled(style)
                .draw(&mut display)
                .unwrap();

            // triangle
            Triangle::new(
                Point::new(16, 16 + yoffset),
                Point::new(16 + 16, 16 + yoffset),
                Point::new(16 + 8, yoffset),
            )
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

            // square
            Rectangle::new(Point::new(52, yoffset), Size::new_equal(16))
                .into_styled(style)
                .draw(&mut display)
                .unwrap();

            // circle
            Circle::new(Point::new(88, yoffset), 16)
                .into_styled(style)
                .draw(&mut display)
                .unwrap();
        }
        _ => (),
    }

    display.flush().unwrap();

    let mut led_pin = pins.gpio13.into_push_pull_output();

    delay.delay_ms(5000);

    loop {
        led_pin.set_high().unwrap();
        delay.delay_ms(10);
        led_pin.set_low().unwrap();
        delay.delay_ms(10);

        let acc_angles = mpu.get_acc_angles().unwrap();

        let mut roll = String::<20>::from("roll:");
        write!(roll, "{}", acc_angles.get(0).unwrap()).unwrap();

        let mut pitch = String::<20>::from("pitch:");
        write!(pitch, "{}", acc_angles.get(1).unwrap()).unwrap();

        let acc = mpu.get_acc().unwrap();
        let mut acc_z = String::<20>::from("acc_z:");
        write!(acc_z, "{}", acc.get(2).unwrap()).unwrap();

        display.clear();

        Text::with_baseline(roll.as_str(), Point::new(0, 0), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        Text::with_baseline(pitch.as_str(), Point::new(0, 16), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        Text::with_baseline(acc_z.as_str(), Point::new(0, 32), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();
    }
}
