extern crate i2cdev;
extern crate byteorder;
use i2cdev::core::I2CDevice; // needed for trait
use i2cdev::linux::LinuxI2CDevice;
use std::thread;
use byteorder::{LittleEndian, ByteOrder, BigEndian};
use std::io;
use std::io::prelude::*;


// addresses
const MODE1: u8 = 0x00;
const MODE2: u8 = 0x01;
const SUBADR1: u8 = 0x02;
const SUBADR2: u8 = 0x03;
const SUBADR3: u8 = 0x04;
const PRESCALE: u8 = 0xFE;
const LED0_ON_L: u8  = 0x06;
const LED0_ON_H : u8 = 0x07;
const LED0_OFF_L: u8 = 0x08;
const LED0_OFF_H: u8 = 0x09;
const ALL_LED_ON_L: u8 = 0xFA;
const ALL_LED_ON_H: u8 = 0xFB;
const ALL_LED_OFF_L: u8 = 0xFC;
const ALL_LED_OFF_H: u8 = 0xFD;

// Bits
const RESTART: u8 = 0x80;
const SLEEP: u8 = 0x10;
const ALLCALL: u8 = 0x01;
const INVRT: u8 = 0x10;
const OUTDRV: u8 = 0x04;

// const TARGET_ADDR: u16 = 0x68; // accelerometer
const TARGET_ADDR: u16 = 0x40; // motor controllers

fn setPWM(dev: &mut LinuxI2CDevice, chan: u8, on: u16, off: u16) {
    dev.smbus_write_byte_data(LED0_ON_L+4*chan, (on & 0xFF) as u8);
    dev.smbus_write_byte_data(LED0_ON_H+4*chan, (on >> 8) as u8);
    dev.smbus_write_byte_data(LED0_OFF_L+4*chan, (off & 0xFF) as u8);
    dev.smbus_write_byte_data(LED0_OFF_H+4*chan, (off >> 8) as u8);
}

// power must be 0 < n < 100
fn setPower(dev: &mut LinuxI2CDevice, chan: u8, power: u16) {
    if power <= 100 && power >= 0 {
        // see http://distantorion.com/2014/10/24/motor-control/
        // this makes output pulses between 1ms and 2ms
        setPWM(dev, chan, 0, power * 8 + 810);
    }
}

// real code should not use unwrap() so liberally
fn main() {
    let mut device = LinuxI2CDevice::new("/dev/i2c-1", TARGET_ADDR).unwrap();
    thread::sleep_ms(100);

    // reset PCA9685
    setPWM(&mut device, 0, 0, 0);
    setPWM(&mut device, 1, 0, 0);
    setPWM(&mut device, 2, 0, 0);
    setPWM(&mut device, 3, 0, 0);
    device.smbus_write_byte_data(MODE2, OUTDRV);
    device.smbus_write_byte_data(MODE1, ALLCALL);
    thread::sleep_ms(5); // wait for oscillator

    // turn off sleep mode
    let mode1 = device.smbus_read_byte_data(MODE1).unwrap() & !SLEEP;
    device.smbus_write_byte_data(MODE1, mode1);
    thread::sleep_ms(5); // wait for oscillator

    let mut input = String::new();

    // set max value and then wait for battery to be connected
    setPower(&mut device, 0, 100);
    setPower(&mut device, 1, 100);
    setPower(&mut device, 2, 100);
    setPower(&mut device, 3, 100);
    println!("Plug in the battery now! Press enter when you hear the beep...");
    io::stdin().read_line(&mut input);
    thread::sleep_ms(3000); // wait 3 seconds after beep to drop pulse length
    setPower(&mut device, 0, 0);
    setPower(&mut device, 1, 0);
    setPower(&mut device, 2, 0);
    setPower(&mut device, 3, 0);

    thread::sleep_ms(5000); // wait 5 seconds and then try powering the motor
    setPower(&mut device, 0, 50);
    thread::sleep_ms(5000);
    setPower(&mut device, 0, 0);
    setPower(&mut device, 1, 50);
    thread::sleep_ms(5000);
    setPower(&mut device, 1, 0);
    setPower(&mut device, 2, 50);
    thread::sleep_ms(5000);
    setPower(&mut device, 2, 0);
    setPower(&mut device, 3, 50);
    thread::sleep_ms(5000);
    setPower(&mut device, 3, 0);

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
