//!
//! This file implements the game SmallBall (see smallball.rs for details) on an
//! Adafruit Feather RP2040 connected to an SSD1306 OLED display and an MPU6050
//! 6-axis IMU via STEMMA QT cables. The pitch and roll measurements from the IMU
//! are the game control inputs.  
//!

#![no_std]
#![no_main]

use adafruit_feather_rp2040::XOSC_CRYSTAL_FREQ;
use config::{
    DELAY_MS, FULL_SCREEN_OUTLINE_SIZE, FULL_SCREEN_OUTLINE_TOP_LET, GAME_NAME, GAME_NAME_LOCATION,
    GAME_OVER_LOCATION, GAME_OVER_LOW_SCORE_LOCATION, GAME_OVER_SCORE_LOCATION, GAME_OVER_TEXT,
    LOW_SCORE_TEXT, SCORE_LOCATION, SCORE_TEXT, SPLASH_SCREEN_SHAPE_LOCATIONS,
    SPLASH_SCREEN_SHAPE_SIZE,
};
use core::fmt::Write;
use cortex_m_rt::entry;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    prelude::{Primitive, Size},
    primitives::{Circle, PrimitiveStyleBuilder, Rectangle},
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
use smallball::{Mode, State};
use ssd1306::{
    mode::DisplayConfig, rotation::DisplayRotation, size::DisplaySize128x64, I2CDisplayInterface,
    Ssd1306,
};

mod config;
mod math;
mod smallball;

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

    // Configure delay to be used for waiting
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

    // Configure two pins as being I2C, not GPIO. From the datasheet for the Feather RP2040,
    // pins GPIO2 and GPIO3 are the SDA and SCL pins, respectively, attached to the JST SH 4-pin
    // connector and cable we are using to connect the hardware.
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

    // Set the style for drawing shapes
    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    // get the led pin for blinking
    let mut led_pin = pins.gpio13.into_push_pull_output();

    // initialize the SmallBall game state
    let mut state = State::new();

    loop {
        display.clear();

        match state.mode() {
            Mode::Intro => {
                // draw screen outline
                Rectangle::new(FULL_SCREEN_OUTLINE_TOP_LET, FULL_SCREEN_OUTLINE_SIZE)
                    .into_styled(style)
                    .draw(&mut display)
                    .unwrap();

                // draw Small Ball text
                Text::with_baseline(GAME_NAME, GAME_NAME_LOCATION, text_style, Baseline::Top)
                    .draw(&mut display)
                    .unwrap();

                // draw a square
                Rectangle::new(SPLASH_SCREEN_SHAPE_LOCATIONS[0], SPLASH_SCREEN_SHAPE_SIZE)
                    .into_styled(style)
                    .draw(&mut display)
                    .unwrap();

                // draw a circle
                Circle::new(
                    SPLASH_SCREEN_SHAPE_LOCATIONS[1],
                    SPLASH_SCREEN_SHAPE_SIZE.width,
                )
                .into_styled(style)
                .draw(&mut display)
                .unwrap();

                // draw a square
                Rectangle::new(SPLASH_SCREEN_SHAPE_LOCATIONS[2], SPLASH_SCREEN_SHAPE_SIZE)
                    .into_styled(style)
                    .draw(&mut display)
                    .unwrap();

                display.flush().unwrap();
                delay.delay_ms(DELAY_MS);
            }
            Mode::Play => {
                // draw the screen outline
                Rectangle::new(state.screen_outline_top_left(), state.screen_outline_size())
                    .into_styled(style)
                    .draw(&mut display)
                    .unwrap();

                // draw the goals that are alive
                for goal in state.goals_alive() {
                    Rectangle::new(goal.location(), Size::new_equal(goal.size()))
                        .into_styled(style)
                        .draw(&mut display)
                        .unwrap();
                }

                // draw the ball
                Circle::new(state.ball().location(), state.ball().size())
                    .into_styled(style)
                    .draw(&mut display)
                    .unwrap();

                // draw the score
                let mut score_text = String::<20>::from(SCORE_TEXT);
                write!(score_text, "{}", state.score()).unwrap();
                Text::with_baseline(
                    score_text.as_str(),
                    SCORE_LOCATION,
                    text_style,
                    Baseline::Top,
                )
                .draw(&mut display)
                .unwrap();

                display.flush().unwrap();
            }
            Mode::Over => {
                // draw screen outline
                Rectangle::new(FULL_SCREEN_OUTLINE_TOP_LET, FULL_SCREEN_OUTLINE_SIZE)
                    .into_styled(style)
                    .draw(&mut display)
                    .unwrap();

                // draw Game Over text
                Text::with_baseline(
                    GAME_OVER_TEXT,
                    GAME_OVER_LOCATION,
                    text_style,
                    Baseline::Top,
                )
                .draw(&mut display)
                .unwrap();

                // draw the score
                let mut score_text = String::<20>::from(SCORE_TEXT);
                write!(score_text, "{}", state.score()).unwrap();
                Text::with_baseline(
                    score_text.as_str(),
                    GAME_OVER_SCORE_LOCATION,
                    text_style,
                    Baseline::Top,
                )
                .draw(&mut display)
                .unwrap();

                // draw the high score
                let mut score_text = String::<20>::from(LOW_SCORE_TEXT);
                write!(score_text, "{}", state.low_score()).unwrap();
                Text::with_baseline(
                    score_text.as_str(),
                    GAME_OVER_LOW_SCORE_LOCATION,
                    text_style,
                    Baseline::Top,
                )
                .draw(&mut display)
                .unwrap();

                display.flush().unwrap();
                delay.delay_ms(DELAY_MS);
            }
        }

        // blink the LED
        led_pin.set_high().unwrap();
        delay.delay_ms(10);
        led_pin.set_low().unwrap();
        delay.delay_ms(10);

        // get mpu control input for the SmallBall game
        let acc_angles = mpu.get_acc_angles().unwrap();
        let roll = acc_angles.get(0).unwrap();
        let pitch = acc_angles.get(1).unwrap();

        // update the state of the game based on the latest control inputs
        state.update(pitch, roll);
    }
}
