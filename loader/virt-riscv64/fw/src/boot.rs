#![no_std]
#![no_main]

extern crate panic_halt;
use riscv_rt::entry;

#[entry]
fn tinyboot_main() -> ! {
    loop { }
}
