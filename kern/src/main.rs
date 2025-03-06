#![no_std]
#![no_main]


#[cfg(not(test))]
mod init;

use pi::timer::spin_sleep;
use pi::gpio::Gpio;
use pi::uart::MiniUart;
use core::fmt::Write;
use core::time::Duration;

#[unsafe(no_mangle)]
fn kmain() -> ! {
    Gpio::new(16).into_output().set();

    // let mut uartTx = Gpio::new(14).into_alt(pi::gpio::Function::Alt0);
    // let mut uartRx = Gpio::new(15).into_alt(pi::gpio::Function::Alt0);
    let mut uart = MiniUart::new();
    uart.set_read_timeout(Duration::from_secs(1));
    
    let delay = Duration::from_secs(1);

    loop {
        spin_sleep(&delay);
        uart.write_str("Hello!");
    }
}