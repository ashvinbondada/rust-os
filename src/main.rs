#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    os::init();

    /* ATTEMPT AT CAUSING A PAGE FAULT IN OS
    let ptr = 0x20425c as *mut u8;
    unsafe { let x = *ptr; }
    println!("read worked!");

    unsafe {
        *ptr = 42;
    }
    println!("write worked!");
    */
    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at : {:?}", level_4_page_table.start_address());

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    os::hlt_loop();
    // loop {
    //     use os::print;
    //     // for _ in 0..10000 {} // addded to slow down printing and see timer interrupt handling
    //     print!("-");
    // }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}
// static HELLO: &[u8] = b"Hello World!";

