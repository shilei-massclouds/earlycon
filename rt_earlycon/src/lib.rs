#![no_std]

use core::panic::PanicInfo;

/// Entry
#[no_mangle]
pub extern "Rust" fn runtime_main(_cpu_id: usize, _dtb_pa: usize) {
    let msg = "\n[early_console]: Hello, ArceOS!\n";
    earlycon::write_bytes(msg.as_bytes());
    hal_base::terminate();
}

pub fn panic(info: &PanicInfo) -> ! {
    boot::panic(info)
}
