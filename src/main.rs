extern crate copter;

use i2cdev::core::I2CDevice; // needed for trait
use std::thread;
use byteorder::{LittleEndian, ByteOrder, BigEndian};
use std::io;
use std::io::prelude::*;

// real code should not use unwrap() so liberally
fn main() {
    let mut motors = Motors::new("/dev/i2c-1");
    thread::sleep_ms(100);

    motors.calibrate();

    motors.power(0.5, 0.0, 0.0, 0.0);
    thread::sleep_ms(5000);
    motors.power(0.0, 0.5, 0.0, 0.0);
    thread::sleep_ms(5000);
    motors.power(0.0, 0.0, 0.5, 0.0);
    thread::sleep_ms(5000);
    motors.power(0.0, 0.0, 0.0, 0.5);
    thread::sleep_ms(5000);
    motors.power(0.0, 0.0, 0.0, 0.0);

    // Now wake the 6050 up as it starts in sleep mode
    // device.smbus_write_byte_data(0x6b, 0);
    // thread::sleep_ms(100);
    // loop {
    //     let mut buf: [u8; 6] = [0u8; 6];
    //     device.write(&[0x3b]); // start reading from start of x
    //     device.read(&mut buf);

    //     let result_x: i16 = BigEndian::read_i16(&[buf[0], buf[1]]);
    //     let result_y: i16 = BigEndian::read_i16(&[buf[2], buf[3]]);
    //     let result_z: i16 = BigEndian::read_i16(&[buf[4], buf[5]]);

    //     thread::sleep_ms(100);
    //     println!("Numbers: {:016b} {:016b} {:016b}", result_x, result_y, result_z);
    // }
}
