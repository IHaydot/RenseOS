use core::arch::asm;
use x86_64::instructions::port::Port;

pub fn outb(port: u16, data: u8){
    unsafe{
        asm!("out dx, al",
        in("dx") port,
        in("al") data
        );
    }
}
pub fn inb(port: u16) -> u8{
    /*let mut returnValue: u8 = 0;
    unsafe{
        asm!("in dx, al",
        in("dx") port,
        in("al") returnValue
        );
        return returnValue;
    }*/
    let mut port = Port::new(port);
    unsafe{port.read()}
}

#[no_mangle]
extern "C" fn C__outb(port: u16, data: u8){
    outb(port, data);
}

#[no_mangle]
extern "C" fn C__inb(port: u16) -> u8{
    inb(port)
}