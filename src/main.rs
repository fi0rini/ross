#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[allow(non_snake_case)]
#[repr(C)]
struct Pl011UartRegs {
    DR:      u32, // Data Register
    RSRECR:  u32, // Receive status / error clear register
    FR:      u32, // Flag register
    ILPR:    u32, // Not in use
    IBRD:    u32, // Integer Baud rate divisor
    FBRD:    u32, // Fractional Baud rate divisor
    LCRH:    u32, // Line control register
    CR:      u32, // Control register
    IFLS:    u32, // Interrupt FIFO level select
    IMSC:    u32, // Interrupt mask set clear
    RIS:     u32, // Raw interrupt status
    MIS:     u32, // Masked interrupt status
    ICR:     u32, // Interrupt clear register
    DMACR:   u32, // DMA control register
    _unused: [u32; 13],
    PeriphID: [u32; 4],
    PCellID:  [u32; 4],
}

/// The base address for the PL011 on Raspberry Pi 4
const UART0_BASE: usize = 0xFE20_1000;

/// Safety: We assume no one else is using or mutating these UART registers concurrently.
fn regs() -> &'static mut Pl011UartRegs {
    unsafe { &mut *(UART0_BASE as *mut Pl011UartRegs) }
}

/// A small helper to initialize the PL011 enough to print in QEMU.
/// On real hardware you would also configure GPIO pins (14,15) to ALT0 for TX/RX,
/// set up the correct baud rate, etc. QEMU often doesn't require as much.
pub fn uart_init() {
    let uart = regs();

    // 1. Disable UART before configuration.
    uart.CR = 0;

    // 2. Clear and disable all interrupts.
    uart.ICR = 0x7FF;
    
    // 3. Set baud rate divisors (just a placeholder).
    //    For QEMU, exact values often don't matter much for a simple test,
    //    but let's pretend we want 115200.
    //    BaudRateDivisor = UARTCLK / (16 * BaudRate).
    //    On real hardware, UARTCLK might be 48 MHz or something else depending on config.
    uart.IBRD = 2;  // integer part (example)
    uart.FBRD = 0xB; // fractional part (example)

    // 4. Set data format in LCRH (8 bits, no parity, FIFO enabled).
    uart.LCRH = (1 << 4) | (3 << 5); // Enable FIFOs (bit 4), 8 bits (bits 5-6)

    // 5. Enable UART, TX, and RX in CR register.
    uart.CR = (1 << 0) | (1 << 8) | (1 << 9);
    // bit 0 = UART enable, bit 8 = TX enable, bit 9 = RX enable
}

/// Wait until the transmitter is ready, then write a single character.
pub fn uart_write_char(c: u8) {
    let uart = regs();

    // FR (Flag Register), bit 5 = TXFF (transmit FIFO full)
    // Wait while FIFO is full.
    while (uart.FR & (1 << 5)) != 0 {
        core::hint::spin_loop();
    }
    // Write data
    uart.DR = c as u32;
}

/// Write a string (no newline handling or anything fancy).
pub fn uart_write_str(s: &str) {
    for byte in s.bytes() {
        // Convert `\n` to `\r\n` if you want a carriage return on newlines
        if byte == b'\n' {
            uart_write_char(b'\r');
        }
        uart_write_char(byte);
    }
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    // Initialize UART (QEMU is fairly forgiving).
    uart_init();

    // Print a test message.
    uart_write_str("Hello from Rust on Pi 4!\n");


    loop {}
}