use crate::{
    println,
    io::{
        inb,
        outb
    },
    vga::{
        writer, 
        Color, 
        create_vga_color
    }
};
//use core::arch::asm;

use lazy_static::lazy_static;
use spin::Mutex;

extern "C"{
    fn C__init_idt();
}

///NOTE: ONLY CALL THIS ONCE
pub unsafe fn init(){
    println!("Initializing IDT...");
    println!("Going into C for easier interaction with assembly...");
    C__init_idt();
    println!("IDT initialized successfully!");
}

//Handlers
//TODO finish the handlers and improve them

#[no_mangle]
extern "C" fn isr1_handler(){ //TODO
    let scan_code = inb(0x60);
    
    outb(0x20, 0x20);
    outb(0xa0, 0x20);
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct InterruptsInfo{
    pub breakpoint_count: usize,
    pub interrupts_count: usize,
}

impl Default for InterruptsInfo{
    fn default() -> Self{
        Self{breakpoint_count: 0, interrupts_count: 0}
    }
}

lazy_static!{
    pub static ref INTERRUPTS_INFO: Mutex<InterruptsInfo> = Mutex::new(
        InterruptsInfo::default()
    );
}

#[no_mangle]
extern "C" fn breakpoint_handler_C(){
    let color = writer.lock().color();
    INTERRUPTS_INFO.lock().breakpoint_count += 1;
    INTERRUPTS_INFO.lock().interrupts_count += 1;
    writer.lock().set_color(create_vga_color(Color::Yellow, Color::Black));
    println!("THE KERNEL HAS HIT BREAKPOINT #{}", INTERRUPTS_INFO.lock().breakpoint_count);
    writer.lock().set_color(color);
}

#[no_mangle]
extern "C" fn general_handler_C(){
    writer.lock().set_color(create_vga_color(Color::Red, Color::Black));
    writer.lock().cls();
    println!("");
    done();
}

#[no_mangle]
extern "C" fn division_handler_C(){
    writer.lock().set_color(create_vga_color(Color::Red, Color::Black));
    writer.lock().cls();
    println!("division fault");
    done();
}

#[no_mangle]
extern "C" fn overflow_handler_C(){
    writer.lock().set_color(create_vga_color(Color::Red, Color::Black));
    writer.lock().cls();
    println!("overflow fault");
    done();
}

#[no_mangle]
extern "C" fn double_fault_handler_C(){
    writer.lock().set_color(create_vga_color(Color::Red, Color::Black));
    writer.lock().cls();
    println!("double fault");
    done();
}

#[no_mangle]
extern "C" fn page_fault_handler_C(){
    writer.lock().set_color(create_vga_color(Color::Red, Color::Black));
    writer.lock().cls();
    let cr2 = x86_64::registers::control::Cr2::read_raw();
    println!("ERROR: ");
    println!("THE OPERATING SYSTEM HAS HIT A PAGE FAULT!");
    println!("ACCESSED ADDRESS: 0x{:x}", cr2);
    println!("EXITING THE KERNEL...");
    done();
}

fn done() -> !{
    loop{
        x86_64::instructions::hlt();
    }
}