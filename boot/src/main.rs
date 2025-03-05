#![no_std]
#![no_main]

#[cfg(not(test))]
mod init;

use pi;
use pi::timer::spin_sleep;
use pi::gpio::Gpio;
use core::arch::asm;
use core::time::Duration;

/// Start address of the binary to load and of the bootloader.
const BINARY_START_ADDR: usize = 0x80000;
const BOOTLOADER_START_ADDR: usize = 0x4000000;

/// Pointer to where the loaded binary expects to be laoded.
const BINARY_START: *mut u8 = BINARY_START_ADDR as *mut u8;

/// Free space between the bootloader and the loaded binary's start address.
const MAX_BINARY_SIZE: usize = BOOTLOADER_START_ADDR - BINARY_START_ADDR;

/// Branches to the address `addr` unconditionally.
/// 
#[inline(always)]
unsafe fn jump_to(addr: *mut u8) -> ! {
    asm!("br ${0}", in(reg) addr, options(noreturn));
    
    loop {
        asm!("wfe", options(nomem, nostack, preserves_flags, volatile));
    }
}

#[unsafe(no_mangle)]
fn kmain() -> ! {
    let delay = Duration::from_secs(1);
    let mut pin16 = Gpio::new(16).into_output();
    let mut uartTx = Gpio::new(14).into_alt(pi::gpio::Function::Alt0);
    let mut uartRx = Gpio::new(15).into_alt(pi::gpio::Function::Alt0);

    loop {
        pin16.set();
        spin_sleep(&delay);
        pin16.clear();
        spin_sleep(&delay);
    }
}