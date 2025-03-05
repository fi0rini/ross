#![no_std]
#![no_main]


#[cfg(not(test))]
mod init;

use pi::timer::spin_sleep;
use pi::gpio::Gpio;
use core::time::Duration;

#[unsafe(no_mangle)]
fn kmain() -> ! {
    let delay = Duration::from_secs(1);
    let mut pin16 = Gpio::new(16).into_output();

    loop {
        pin16.set();
        spin_sleep(&delay);
        pin16.clear();
        spin_sleep(&delay);
    }
}