#![no_std]
#![no_main]

#[cfg(not(test))]
mod init;

use core::arch::asm;
use core::fmt::Write;
use core::time::Duration;

use pi::{self, timer};
use pi::gpio::{Gpio, PinOut};
use pi::uart::MiniUart;

use xmodem::{Progress, Xmodem};

/// Start address of the binary to load and of the bootloader.
const BINARY_START_ADDR:        usize = 0x0080000;
const BOOTLOADER_START_ADDR:    usize = 0x4000000;

/// Pointer to where the loaded binary expects to be laoded.
const BINARY_START: *mut u8 = BINARY_START_ADDR as *mut u8;

/// Free space between the bootloader and the loaded binary's start address.
const MAX_BINARY_SIZE: usize = BOOTLOADER_START_ADDR - BINARY_START_ADDR;

/// Branches to the address `addr` unconditionally.
/// 
#[inline(always)]
fn jump_to(addr: *mut u8) -> ! {
    unsafe { asm!("br {}", in(reg) addr, options(noreturn)) };
    
    loop {
        unsafe { asm!("wfe", options(nomem, nostack, preserves_flags)) };
    }
}

use mutex::Mutex;
use pi::gpio::Output;

/// Global `PinOut` singleton.
pub static PIN_16: Mutex<PinOut<Output>> = Mutex::new(PinOut::new(16));

fn flash_pin(num: u8, milli: u64) {
    let mut pin = PIN_16.lock();
    let flash = Duration::from_millis(milli);

    for _ in 0..num {
        pin.on();
        timer::spin_sleep(&flash);
        pin.off();
        timer::spin_sleep(&flash);
    }
}

#[unsafe(no_mangle)]
fn kmain() -> ! {
    // let delay = Duration::from_secs(1);
    
    // let mut buf =
    flash_pin(5, 450);

    let mut uart = MiniUart::new();
    uart.set_read_timeout(Duration::from_secs(1));
    
    let mut kernel = unsafe { core::slice::from_raw_parts_mut(BINARY_START, MAX_BINARY_SIZE) };

    loop {
        match Xmodem::receive(&mut uart, &mut kernel) {
            Ok(_) => {
                flash_pin(2, 100);
                timer::spin_sleep(&Duration::from_secs(2));
                jump_to(BINARY_START)
            },
            Err(_) => {
                flash_pin(10, 100);
                timer::spin_sleep(&Duration::from_secs(2));

                continue
            }
        }
    }
}