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

struct Motors {
    device: LinuxI2CDevice,
}

const MOTORS_ADDR: u16 = 0x40;

// see also
// https://learn.adafruit.com/downloads/pdf/adafruit-16-channel-servo-driver-with-raspberry-pi.pdf
// https://github.com/adafruit/Adafruit-Raspberry-Pi-Python-Code
//       /blob/master/Adafruit_PWM_Servo_Driver/Adafruit_PWM_Servo_Driver.py
// http://distantorion.com/2014/10/24/motor-control/
impl Motors {
    fn new(i2c_name: &'static str) -> Motors {
        let mut motors = Motors {
            device: LinuxI2CDevice::new(i2c_name, MOTORS_ADDR).unwrap()
        };

        thread::sleep_ms(100);
        motors.power(0.0,0.0,0.0,0.0);

        // reset PCA9685
        motors.device.smbus_write_byte_data(MODE2, OUTDRV);
        motors.device.smbus_write_byte_data(MODE1, ALLCALL);
        thread::sleep_ms(5); // wait for oscillator

        // turn off sleep mode
        let mode1 = motors.device.smbus_read_byte_data(MODE1).unwrap() & !SLEEP;
        motors.device.smbus_write_byte_data(MODE1, mode1);
        thread::sleep_ms(5); // wait for oscillator

        motors
    }

    pub fn calibrate(&mut self) {
        self.power(1.0, 1.0, 1.0, 1.0);
        println!("Get ready to plug in battery...");
        thread::sleep_ms(2000);
        println!("Now!");
        thread::sleep_ms(4000);
        self.power(0.0, 0.0, 0.0, 0.0);
        thread::sleep_ms(5000);
    }

    pub fn power(&mut self, motor1: f64, motor2: f64, motor3: f64, motor4: f64) {
        self.power_one(0, 0, (motor1 * 800.0) as u16 + 810);
        self.power_one(1, 0, (motor2 * 800.0) as u16 + 810);
        self.power_one(2, 0, (motor3 * 800.0) as u16 + 810);
        self.power_one(3, 0, (motor4 * 800.0) as u16 + 810);
    }

    fn power_one(&mut self, chan: u8, on: u16, off: u16) {
        self.device.smbus_write_byte_data(LED0_ON_L+4*chan, (on & 0xFF) as u8);
        self.device.smbus_write_byte_data(LED0_ON_H+4*chan, (on >> 8) as u8);
        self.device.smbus_write_byte_data(LED0_OFF_L+4*chan, (off & 0xFF) as u8);
        self.device.smbus_write_byte_data(LED0_OFF_H+4*chan, (off >> 8) as u8);
    }
}

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
