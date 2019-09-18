//! CDC-ACM serial port example using polling in a busy loop.
#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32f3xx_hal::{prelude::*, stm32, hal::digital::v2::OutputPin, hal::digital::v2::InputPin};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    // Configure the on-board LED (LD10, south red)
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let mut led = gpioe.pe13.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    led.set_low(); // Turn off

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let button = gpioa.pa0.into_pull_down_input(&mut gpioa.moder, &mut gpioa.pupdr);

    loop {
        if button.is_high().unwrap() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
