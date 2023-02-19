// Kernel v0.0.1 (2023-02-19)

#![no_std] // doesn't link Rust std library
#![no_main] // disables all Rust-level entry points 

use core::panic::PanicInfo;

mod boot {
    use core::arch::global_asm;

    global_asm!{
        ".section .text._start"
    }
}

#[no_mangle] // doesn't mangle with the name of this function  
pub extern "C"  fn _start() -> ! {

// this function is the entry point 
// named it _start by default 

    loop{}
}


// This function will be called on panic 
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
