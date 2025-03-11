use core::panic::PanicInfo;
use crate::console::kprintln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    kprintln!();
    kprintln!("    The pi is overdone.");
    kprintln!();
    kprintln!("---------- PANIC ----------");
    
    if let Some(location) = _info.location() {
        kprintln!("FILE: {}", location.file());
        kprintln!("LINE: {}", location.line());
    } else {
        kprintln!("No idea where the panic occured");
    }

    let msg = _info.message();
    kprintln!("{:?}", msg);
    kprintln!();

    loop {}
}