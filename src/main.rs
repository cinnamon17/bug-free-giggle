#![no_std] 
#![no_main] 

use core::panic::PanicInfo;

#[unsafe(no_mangle)] 
pub extern "C" fn _start() -> ! {
    unsafe {
        
        core::arch::asm!(
            "lui a1, 0x10000",     
            "li  a0, 72",          
            "sb  a0, 0(a1)",
            "li  a0, 111",         
            "sb  a0, 0(a1)",
            "li  a0, 108",         
            "sb  a0, 0(a1)",
            "li  a0, 97",          
            "sb  a0, 0(a1)",
            "li  a0, 10",          
            "sb  a0, 0(a1)",
        );
    }

    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
