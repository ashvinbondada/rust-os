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

    // x86_64::instructions::interrupts::int3();
    // unsafe {
    //     *(0xdeadbeef Inter::.as_u8();
    // }
    fn stack_overflow() {
        stack_overflow();
    }
    // stack_overflow();

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

