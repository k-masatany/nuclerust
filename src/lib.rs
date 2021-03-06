#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(const_unique_new)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate volatile;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() {
    // ATTENTION: we have a very small stack and no guard page
    vga_buffer::clear_screen();
    println!("Hello World{}", "!");

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
