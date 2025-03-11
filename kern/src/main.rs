#![no_std]
#![no_main]
#![feature(decl_macro)]

#[cfg(not(test))]
mod init;

use pi::gpio::Gpio;

pub mod console;
pub mod shell;


#[unsafe(no_mangle)]
fn kmain() -> ! {
    Gpio::new(16).into_output().set();
    shell::shell(&">")
}