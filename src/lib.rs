//! This is a platform agnostic Rust driver for the HRS3300 heart rate sensor
//! using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable/disable heart rate sensor. See: [`enable_hrs()`].
//! - Enable/disable oscillator. See: [`enable_oscillator()`].
//! - Initialize the device. See: [`init()`].
//! - Set the conversion delay. See: [`set_conversion_delay()`].
//! - Set the gain. See: [`set_gain()`].
//! - Set the ambient light sensor resolution. See: [`set_als_resolution()`].
//! - Set the LED current. See: [`set_led_current()`].
//! - Read the device id. See: [`device_id()`].
//! - Read the last heart rate sensor measurement. See: [`read_hrs()`].
//! - Read the last ambient light sensor measurement. See: [`read_als()`].
//!
//! [`enable_hrs()`]: struct.Hrs3300.html#method.enable_hrs
//! [`enable_oscillator()`]: struct.Hrs3300.html#method.enable_oscillator
//! [`init()`]: struct.Hrs3300.html#method.init
//! [`set_conversion_delay()`]: struct.Hrs3300.html#method.set_conversion_delay
//! [`set_gain()`]: struct.Hrs3300.html#method.set_gain
//! [`set_als_resolution()`]: struct.Hrs3300.html#method.set_als_resolution
//! [`set_led_current()`]: struct.Hrs3300.html#method.set_led_current
//! [`device_id()`]: struct.Hrs3300.html#method.device_id
//! [`read_hrs()`]: struct.Hrs3300.html#method.read_hrs
//! [`read_als()`]: struct.Hrs3300.html#method.read_als
//!
//! <!-- TODO
//! [Introductory blog post](TODO)
//! -->
//!
//! ## The device
//!
//! HRSS3300 is an optical digital heart rate sensor/monitor featuring a 525nm
//! green LED and a reflection light detector for the PPG signal from the human
//! body.
//! The typical heart rate measurement samples the reflected PPG signal at
//! 25Hz then the results can be read via the I2C bus.
//!
//! Datasheet:
//! - [HRS3300](http://files.pine64.org/doc/datasheet/pinetime/HRS3300%20Heart%20Rate%20Sensor.pdf)
//!
//! <!--
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the appropriate device.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//! -->
#![deny(unsafe_code, missing_docs)]
#![no_std]

extern crate embedded_hal as hal;

/// HRS3300 device driver
#[derive(Debug)]
pub struct Hrs3300<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
}

mod device_impl;
mod types;
pub use types::{AlsResolution, ConversionDelay, Error, Gain, LedCurrent};
