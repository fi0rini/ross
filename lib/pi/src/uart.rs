use core::fmt;
use core::time::Duration;

use volatile::prelude::*;
use volatile::{Volatile, ReadVolatile, Reserved};

use crate::timer::Timer;
use crate::common::IO_BASE;
use crate::gpio::{Gpio, Function};

/// The base address for the `MU` registers.
const MU_REG_BASE: usize = IO_BASE + 0x215040;

/// The `AUXENB` register from page 9 of the BCM2837 documentation.
const AUX_ENABLES: *mut Volatile<u8> = (IO_BASE + 0x215004) as *mut Volatile<u8>;

/// Enum representing bit fields of the `AUX_MU_LSR_REG` register.
#[repr(u8)]
enum LsrStatus {
    DataReady = 1,
    TxIdle = 1 << 6, // finished shifting out the last bit
    TxEmpty = 1 << 5, // this bit is set if the transmit FIFO can accept at least one byte
}

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    // FIXME: Declare the "MU" registers from page 8.
    IO: Volatile<u8>,       // i/o data
    _r0: [Reserved<u8>; 3],
    IER: Volatile<u8>,      // interrupt enable
    _r1: [Reserved<u8>; 3],
    IIR: Volatile<u8>,      // interrupt identify
    _r2: [Reserved<u8>; 3],
    LCR: Volatile<u8>,      // line control
    _r3: [Reserved<u8>; 3],
    MCR: Volatile<u8>,      // modem control
    _r4: [Reserved<u8>; 3],
    LSR: ReadVolatile<u8>,      // line status
    _r5: [Reserved<u8>; 3],
    MSR: ReadVolatile<u8>,      // modem status
    _r6: [Reserved<u8>; 3],
    SCRATCH: Volatile<u8>,  // scratch
    _r7: [Reserved<u8>; 3],
    CNTL: Volatile<u8>,     // extra control
    _r8: [Reserved<u8>; 3],
    STAT: ReadVolatile<u32>,    // extra status
    BAUD: Volatile<u16>,    // baudrate
}

/// The Raspberry Pi's "mini UART".
pub struct MiniUart {
    registers: &'static mut Registers,
    timeout: Option<Duration>,
}

impl MiniUart {
    /// Initializes the mini UART by enabling it as an auxiliary peripheral,
    /// setting the data size to 8 bits, setting the BAUD rate to ~115200 (baud
    /// divider of 270), setting GPIO pins 14 and 15 to alternative function 5
    /// (TXD1/RDXD1), and finally enabling the UART transmitter and receiver.
    ///
    /// By default, reads will never time out. To set a read timeout, use
    /// `set_read_timeout()`.
    pub fn new() -> MiniUart {
        let registers = unsafe {
            // Enable the mini UART as an auxiliary device.
            (*AUX_ENABLES).or_mask(1);
            &mut *(MU_REG_BASE as *mut Registers)
        };

        Gpio::new(14).into_alt(Function::Alt5);
        Gpio::new(15).into_alt(Function::Alt5);

        registers.CNTL.write(0b00); // turn off tx,rx
        registers.LCR.write(0b11);
        registers.BAUD.write(270);
        registers.CNTL.write(0b11); // turn on tx,rx

        return MiniUart {
            registers: registers,
            timeout: None,
        };
    }

    /// Set the read timeout to `t` duration.
    pub fn set_read_timeout(&mut self, t: Duration) {
        self.timeout = Some(t);
    }

    /// Write the byte `byte`. This method blocks until there is space available
    /// in the output FIFO.
    pub fn write_byte(&mut self, byte: u8) {
        while (self.registers.LSR.read() & LsrStatus::TxEmpty as u8) == 0 {}
        self.registers.IO.write(byte);
    }

    /// Returns `true` if there is at least one byte ready to be read. If this
    /// method returns `true`, a subsequent call to `read_byte` is guaranteed to
    /// return immediately. This method does not block.
    pub fn has_byte(&self) -> bool {
        (self.registers.LSR.read() & LsrStatus::DataReady as u8) != 0
    }

    /// Blocks until there is a byte ready to read. If a read timeout is set,
    /// this method blocks for at most that amount of time. Otherwise, this
    /// method blocks indefinitely until there is a byte to read.
    ///
    /// Returns `Ok(())` if a byte is ready to read. Returns `Err(())` if the
    /// timeout expired while waiting for a byte to be ready. If this method
    /// returns `Ok(())`, a subsequent call to `read_byte` is guaranteed to
    /// return immediately.
    pub fn wait_for_byte(&self) -> Result<(), ()> {
        match self.timeout {
            None => {
                while !self.has_byte() {}
                return Ok(())
            },
            Some(t) => {
                let timer = Timer::new();
                let timeout = timer.read() + t;
                while timer.read() < timeout {
                    if self.has_byte() {
                        return Ok(())
                    }
                }
                return Err(())
            }
        }
    }

    /// Reads a byte. Blocks indefinitely until a byte is ready to be read.
    pub fn read_byte(&mut self) -> u8 {
        while !self.has_byte() {}
        self.registers.IO.read()
    }

    pub fn is_idle(&self) -> bool {
        self.registers.LSR.read() & LsrStatus::TxIdle as u8 != 0
    }
}

impl fmt::Write for MiniUart {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for c in s.bytes() {
            if c == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(c);
        }
        return Ok(());
    }
}

mod uart_io {
    use super::MiniUart;
    use core2::io;
    
    impl io::Read for MiniUart {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
            let has_byte = self.wait_for_byte();
            if has_byte == Err(()) {
                return Err(io::Error::new(io::ErrorKind::TimedOut, "Timed Out"));
            }
            let mut idx = 0;
            while self.has_byte() && idx < buf.len() {
                buf[idx] = self.read_byte();
                idx += 1;
            }
            return Ok(idx);
        }
    }

    impl io::Write for MiniUart {
        fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
            for &b in buf {
                self.write_byte(b);
            }
            return Ok(buf.len());
        }

        fn flush(&mut self) -> Result<(), io::Error> {
            while !self.is_idle() {} 
            Ok(())
        }
    }
    
}
