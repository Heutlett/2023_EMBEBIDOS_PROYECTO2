#![no_std]
#![no_main]

// 3F20 0008 fsel2 1<<3, para colocarlo como output
// 3F20 001C fset para encender el pin 21.
// 3F20 0028 fclear para apagar el pin 21.


use core::panic::PanicInfo;
use core::arch::asm;

mod boot {
    use core::arch::global_asm;
    
   global_asm!(
        ".section .text._start"
    );
}

#[no_mangle]
pub extern "C" fn _start() -> !{
    unsafe {
        // Para colocar el pin 21
        core::ptr::write_volatile(0x3F20_0008 as *mut u32, 1<<3);
	
        loop {
            // Encender un pin
            core::ptr::write_volatile(0x3F20_001C as *mut u32, 1<<21);

            for _ in 1..50000 {
                asm!("nop");    
            }
        
            // apagar un pin
            core::ptr::write_volatile(0x3F20_0028 as *mut u32, 1<<21);

            for _ in 1..50000 {
                asm!("nop");
            }
        }


    }
}

#[panic_handler]
fn panic (_info: &PanicInfo) -> !{
    loop {}
}