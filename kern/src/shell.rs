use stack_vec::StackVec;

use crate::console::{kprint, kprintln, CONSOLE};

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs,
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>,
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args[0]        
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// returns if the `exit` command is called.
pub fn shell(prefix: &str) -> ! {
    loop {
        kprint!("{} ", prefix);
        let mut idx: usize = 0;
        let mut buff: [u8; 512] = [0; 512];
        loop {
            let mut c = CONSOLE.lock();
            let b = c.read_byte();
            if b == b'\r' || b == b'\n' {
                buff[idx] = ' ' as u8;
                break;
            }
            if b == 8 { // this is a backspace
                kprint!("{}", 8 as char);
                kprint!("{}", ' ');
                kprint!("{}", 8 as char);
                idx -= 1;
                buff[idx] = 0;
                continue;
            }
            kprint!("{}", b as char);
            buff[idx] = b;
            idx += 1;
        }
        let s = &core::str::from_utf8(&buff).unwrap()[..];
        let mut fields: [&str; 512] = [""; 512];
        match Command::parse(s, &mut fields) {
            Ok(cmd) => {
                match cmd.path() {
                    "echo" => {
                        kprintln!();
                        for i in 1..cmd.args.len() {
                            kprint!("{} ", cmd.args[i]);
                        }
                        kprintln!();
                    },
                    "exit" => {
                        panic!();
                    },
                    _ => {
                        kprintln!();
                        kprintln!("unknown command: {}", cmd.path())
                    }
                }
            }, 
            Err(_) => {
                kprintln!();
                kprintln!("error parsing command!");
            }
        }
    }
}