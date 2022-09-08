#![feature(lang_items)]
#![no_std]
#![feature(custom_test_frameworks)]
#![feature(panic_info_message)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(dead_code)]

extern crate rlibc;
extern crate multiboot2;

mod io;

#[path = "shared/shared.rs"]
mod shared;

#[path = "Drivers/gfx/vga.rs"]
mod vga;
use crate::vga::{create_vga_color, Color, writer};

#[path = "Memory/mm.rs"]
mod mm;

#[path = "Structures/idt/idt.rs"]
mod idt;

use x86_64::instructions::interrupts::int3;

#[derive(Clone, Debug, PartialEq, Copy, Eq)]
struct KernelInfo{
    start_address: u64,
    size: u64,
    end_address: u64,
    multiboot_info_start_address: u64,
    multiboot_info_end_address: u64,
    multiboot_info_size: u64,
}

#[no_mangle]
pub unsafe extern "C" fn _start(multiboto2_info_structure_addr: usize) -> !{
    let boot_info = multiboot2::load(multiboto2_info_structure_addr).expect("Multiboot2 information structure required!");
    
    //Required because for some reason printing two times without this throws an exception
    writer.lock().cls();
    println!("Hello fmt, {}", 1);
    println!("Hello fmt, {}", 2);
    writer.lock().cls();

    //Getting some boot info
    //Memory map
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required!");
    let mut total_ram = 0;
    for (_i ,map) in memory_map_tag.memory_areas().enumerate() {
        /*println!("Memory map #{}; base: 0x{:x}; lenght: 0x{:x}; end: 0x{:x}", 
            i,
            map.start_address(),
            map.size(),
            map.end_address()
        );*/
        total_ram += map.size();
    }

    //Kernel ELF sections
    let elf_sections_tag = boot_info.elf_sections_tag().expect("ELF sections tag required!");
    for (_i, _section) in elf_sections_tag.sections().enumerate() {
        /*println!("ELF section #{i}; Base: 0x{:x}; lenght: 0x{:x}; end: 0x{:x}; flags: 0x{:x}",
            section.start_address(),
            section.size(),
            section.end_address(),
            section.flags(),
        );*/
    }

    //Save some of info
    let kernel_info = KernelInfo{
        start_address: elf_sections_tag.sections().map(|s| s.start_address()).min().unwrap(),
        size: elf_sections_tag.sections().map(|s| s.end_address()).max().unwrap() - elf_sections_tag.sections().map(|s| s.start_address()).min().unwrap(),
        end_address: elf_sections_tag.sections().map(|s| s.end_address()).max().unwrap(),
        multiboot_info_start_address: boot_info.start_address() as u64,
        multiboot_info_end_address: boot_info.end_address() as u64,
        multiboot_info_size: boot_info.total_size() as u64
    };

    if shared::bytes_to_gigabytes(total_ram) == 0{
        println!("Total number of RAM: {}B or {}MB", total_ram, (total_ram / 1000) / 1000);
    }else{
        println!("Total number of RAM: {}B or {}MB or {}GB",
            total_ram,
            (total_ram / 1000) / 1000,
            shared::bytes_to_gigabytes(total_ram),
        );
    }

    println!("Total kernel size: {}KB", shared::bytes_to_kilobytes(kernel_info.size));
    
    idt::init();

    for _ in 0..10{
        int3();
    }

    #[cfg(test)]
    test_main();
    
    loop{}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    writer.lock().set_color(create_vga_color(Color::Red, Color::Black));
    println!("Kernel has panicked, here is the panic info:");
    if let Some(location) = info.location() {
        println!("File: {}", location.file());
        println!("Line: {}", location.line());
        println!("Column: {}", location.column());
    }
    
    println!("Panic message: {}", info.message().unwrap());
    

    loop{}
}

// Fault handler tests 
// ALL OF THESE ARE UNSAFE BECAUSE THEY CAUSE THE OS TO CRASH
#[allow(warnings)]
unsafe fn stack_overflow(){
    stack_overflow();
}

unsafe fn page_fault(){
    let mem = 0xdeadbeef as *mut u8;
    *mem = b'a';
}