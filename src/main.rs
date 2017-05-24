#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;
extern crate rand;
extern crate sysfs_gpio;

mod errors;
mod shift_register;

use errors::*;
use shift_register::{ShiftRegister, ShiftRegisterType};

fn main() {
    let mut leds = ShiftRegister::new(ShiftRegisterType::LED);
    let mut relais = ShiftRegister::new(ShiftRegisterType::Relais);

    loop {
        leds.test();
        relais.test();
    }
}
