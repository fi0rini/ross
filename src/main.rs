#![no_std]
#![no_main]


#[cfg(not(test))]
mod init;

const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

use core::arch::asm;

#[inline(never)]
fn spin_sleep_ms(ms: usize) {
    for _ in 0..(ms * 6000) {
        unsafe { asm!("nop", options(nomem, nostack, preserves_flags)); }
    }
}

fn gpio_init() {
    let reg = GPIO_FSEL1 as *mut u32;
    let mut val = unsafe { reg.read_volatile() };
    val &= !(0b111 << 18);
    val |= 0b001 << 18;
    unsafe  { GPIO_FSEL1.write_volatile(val); }
}

fn gpio_set() {
    unsafe { GPIO_SET0.write_volatile(1 <<16 ); }
}

fn gpio_clear() {
    unsafe { GPIO_CLR0.write_volatile(1 << 16); }
}

#[no_mangle]
fn kmain() -> ! {
    gpio_init();

    loop {
        gpio_set();
        spin_sleep_ms(10000);
        gpio_clear();
        spin_sleep_ms(10000);
    }
}