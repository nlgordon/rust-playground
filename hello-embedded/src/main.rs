#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use core::fmt::Write;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hio};

#[entry]
fn main() -> ! {

    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Hello, world!").unwrap();

    debug::exit(debug::EXIT_SUCCESS);

    loop {
        // your code goes here
    }
}
