use crate::io::{outb};
use spin::Mutex;

pub const VGA_DEFAULT_BUFFER: *mut u8 = 0xb8000 as *mut u8;
pub const VGA_DEFAULT_BUFFER_SIZE: usize = 0xb8000 + 4000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color{
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

pub const fn create_vga_color(foreground: Color, background: Color) -> u8{
    (background as u8) << 4 | foreground as u8
}

pub const VGA_DEFAULT_COLOR: u8 = create_vga_color(Color::White, Color::Black);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Writer{

    cursor_pos: u16,

    color: u8,
}

impl Default for Writer{
    fn default() -> Self{
        let mut ret = Writer {color: VGA_DEFAULT_COLOR, cursor_pos: 0 };
        ret.set_cursor_pos(0);
        ret
    }
}

impl Writer {
    #[allow(warnings)]
    pub fn new(buffer: *mut u8) -> Self {
        let mut ret = Writer { color: VGA_DEFAULT_COLOR, cursor_pos: 0 };
        ret.set_cursor_pos(0);
        ret
    }

    pub fn set_cursor_pos(&mut self , pos: u16) {
        outb(0x3d4, 0x0f);
        outb(0x3d5, (pos & 0xff) as u8);
        outb(0x3d4, 0x0e);
        outb(0x3d5, ((pos >> 8) & 0xff) as u8);
        
        self.cursor_pos = pos as u16;    
    }

    pub fn pos_from_grid(&mut self, width: u16, height: u16) -> u16{
        height * 80 + width
    }

    pub fn putc(&mut self, c: char) {
        unsafe{
            match c{
                '\n' => {
                    self.nln();
                }

                _ => { 
                    *VGA_DEFAULT_BUFFER.offset(self.cursor_pos as isize * 2) = c as u8;
                    *VGA_DEFAULT_BUFFER.offset(self.cursor_pos as isize * 2 + 1) = self.color;
                    self.cursor_pos += 1;
                    self.set_cursor_pos(self.cursor_pos);
                }
            }
        }
    }

    pub fn puts(&mut self, s: &str) {
        for item in s.chars() {
            self.putc(item);
        }
    }

    pub fn nln(&mut self){
        self.cursor_pos += 80;
        self.cursor_pos -= self.cursor_pos % 80;
        self.set_cursor_pos(self.cursor_pos);
    }   

    pub fn set_color(&mut self ,color: u8){
        self.color = color;
    }

    pub fn color(&mut self) -> u8 { self.color }

    pub fn cls(&mut self){
        self.cursor_pos = 0;
        
        for _ in 0xb8000..VGA_DEFAULT_BUFFER_SIZE{
            self.putc(' ');
        }

        self.cursor_pos = 0;
        self.set_cursor_pos(self.cursor_pos);
    }
}

use lazy_static::lazy_static;
lazy_static!{
    #[allow(warnings)] //Im not dealing with this shit right now
    pub static ref writer: Mutex<Writer> = Mutex::new(Writer::default());
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result{
        self.puts(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    writer.lock().write_fmt(args).unwrap();
}

