mod parsers;

use clap::command;
use serial;
use xmodem::Xmodem;
use xmodem::Progress;

use std::path::PathBuf;
use std::time::Duration;

use serial::core::{CharSize, BaudRate, StopBits, FlowControl, SerialDevice, SerialPortSettings};

use parsers::{parse_width, parse_stop_bits, parse_flow_control, parse_baud_rate};
use clap::{Parser, ValueHint};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file (defaults to stdin if not set)
    #[arg(short = 'i', value_hint = ValueHint::FilePath)]
    input: Option<PathBuf>,

    /// Set baud rate
    #[arg(short = 'b', long = "baud", value_parser = parse_baud_rate, default_value = "115200")]
    baud_rate: BaudRate,

    /// Set timeout in seconds
    #[arg(short = 't', long = "timeout",  default_value = "10")]
    timeout: u64,

    /// Set data character width in bits
    #[arg(short = 'w', long = "width", value_parser = parse_width, default_value = "8")]
    char_width: CharSize,

    /// Path to TTY device
    #[arg(value_hint = ValueHint::FilePath)]
    tty_path: PathBuf,

    /// Enable flow control ('hardware' or 'software')
    #[arg(short = 'f', long = "flow-control", value_parser = parse_flow_control, default_value = "none")]
    flow_control: FlowControl,

    /// Set number of stop bits
    #[arg(short = 's', long = "stop-bits", value_parser = parse_stop_bits, default_value = "1")]
    stop_bits: StopBits,

    /// Disable XMODEM
    #[arg(short = 'r', long = "raw")]
    raw: bool,
}

fn progress_fn(progress: Progress) {
    println!("Progress: {:?}", progress);
}

fn main() {
    use std::fs::File;
    use std::io::{self, BufReader, Write, Read};

    let opt = Args::parse();
    let mut port = serial::open(&opt.tty_path).expect("path points to invalid TTY");

    let mut settings = port.read_settings().expect("valid");
    settings.set_baud_rate(opt.baud_rate).expect("valid baud rate");
    settings.set_char_size(opt.char_width);
    settings.set_stop_bits(opt.stop_bits);
    settings.set_flow_control(opt.flow_control);
    port.write_settings(&settings).expect("valid settings");
    port.set_timeout(Duration::from_secs(opt.timeout)).expect("valid timeout");

    // FIXME: Implement the `ttywrite` utility.
    match opt.input {
        None => {
            loop {
                let mut buffer = String::new();
                io::stdin().read_to_string(&mut buffer).expect("valid read");
                if opt.raw {
                    (&mut port).write_all(&buffer.as_bytes()).expect("valid write");
                } else {
                    Xmodem::transmit_with_progress(buffer.as_bytes(), &mut port, progress_fn).expect("valid transmit");
                }
            }
        },
        Some(path) => {
            let mut input = BufReader::new(File::open(path).expect("file should exist"));
            if opt.raw {
                std::io::copy(&mut input, &mut port).expect("valid copy");
            } else {
                Xmodem::transmit_with_progress(input, &mut port, progress_fn).expect("valid transmit");
            }
        },
    }

}