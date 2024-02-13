use core::fmt::{Arguments, Write};

use psf_rs::Font;
use x86_64::instructions::port::Port;

use crate::{graphics::Graphics, shell::SHELL};

pub static mut SCREEN: Screen = Screen::new();
pub const SCREEN_WIDTH: usize = 320;
pub const SCREEN_HEIGHT: usize = 200;

struct Buffer {
    chars: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
}
pub struct Screen {
    pub chars: [[u32; SCREEN_WIDTH / 8]; SCREEN_HEIGHT / 16],
    pub column: usize,
    pub row: usize,
    pub font: Option<Font>,
    buffer: *mut Buffer,
    color: u8,
}
impl Screen {
    fn print(&mut self, string: &str) {
        for utf8 in string.chars() {
            self.handle_utf8(utf8.into());
        }
    }
    fn print_utf8(&mut self, utf8: u32) {
        if unsafe { SHELL.command_input } && self.column == SCREEN_WIDTH / 8 - 1 {
            return;
        }

        let Some(font) = self.font.as_ref() else {
            return;
        };
        let buffer = unsafe { self.buffer.as_mut().unwrap() };

        font.display_glyph(utf8, |bit, x, y| {
            buffer.chars[self.row * 16 + y as usize][self.column * 8 + x as usize] =
                bit * self.color;
        });

        self.chars[self.row][self.column] = utf8;
        self.inc_pos();
    }
    pub fn print_graphics(&mut self, graphics: Graphics) {
        unsafe { Port::new(0x3C8).write(0xF8 as u8) };
        let mut data_port = Port::new(0x3C9);
        for i in graphics.color_pallete.unwrap_or([(63, 63, 63); 8]) {
            unsafe {
                data_port.write(i.0);
                data_port.write(i.1);
                data_port.write(i.2);
            }
        }
        let mut color_index = 0;
        for y in 0..graphics.height {
            if y != 0 {
                self.newline();
            }
            for x in 0..graphics.width {
                self.print_graphic(&graphics, &mut color_index, y, x);
            }
        }
    }
    fn print_graphic(&mut self, graphic: &Graphics, color_index: &mut usize, y: u16, x: u16) {
        let buffer = unsafe { self.buffer.as_mut().unwrap() };
        for h in 0..16 {
            for w in 0..8 {
                buffer.chars[self.row * 16 + h][self.column * 8 + w] = if ((graphic
                    .data
                    .get((y * graphic.width + x) as usize)
                    .unwrap_or(&[0; 16])[h as usize]
                    & 0x80 >> w)
                    << w)
                    / 0x80
                    == 1
                {
                    let color = 0xF8
                        + ((*graphic
                            .color_data
                            .unwrap_or(&[])
                            .get(*color_index / 2)
                            .unwrap_or(&0)
                            & (0x03 << (*color_index % 2) * 4))
                            >> ((*color_index % 2) * 4));
                    *color_index += 1;
                    color
                } else {
                    0x00
                };
            }
        }
        self.inc_pos();
    }
    fn newline(&mut self) {
        self.column = 0;
        self.row += 1;
        if self.row >= SCREEN_HEIGHT / 16 {
            let buffer = unsafe { self.buffer.as_mut().unwrap() };
            for y in 0..SCREEN_HEIGHT / 16 - 1 {
                self.chars[y] = self.chars[y + 1];
                for y2 in 0..16 {
                    for x in 0..SCREEN_WIDTH {
                        buffer.chars[y * 16 + y2][x] = buffer.chars[(y + 1) * 16 + y2][x];
                    }
                }
            }
            self.chars[SCREEN_HEIGHT / 16 - 1] = [0; SCREEN_WIDTH / 8];
            for y in 0..15 {
                for x in 0..SCREEN_WIDTH {
                    buffer.chars[(SCREEN_HEIGHT - 16) + y][x] = 0x00;
                }
            }
            self.row -= 1;
        }
    }
    fn backspace(&mut self) {
        let buffer = unsafe { self.buffer.as_mut().unwrap() };
        #[allow(static_mut_ref)]
        let shell = unsafe { &SHELL };
        if shell.command_input {
            if self.column != 2 {
                self.dec_pos();
                self.chars[self.row][self.column] = 0;
                for y in 0..15 {
                    for x in 0..8 {
                        buffer.chars[self.row * 16 + y][self.column * 8 + x] = 0x00;
                    }
                }
            }
        } else {
            self.dec_pos();
            self.chars[self.row][self.column] = 0;
            for y in 0..15 {
                for x in 0..8 {
                    buffer.chars[self.row * 16 + y][self.column * 8 + x] = 0x00;
                }
            }
        }
    }
    fn dec_pos(&mut self) {
        if self.row != 0 {
            if self.column != 0 {
                self.column -= 1;
            } else {
                self.row -= 1;
                self.column = (SCREEN_WIDTH / 8) - 1;
            }
        } else {
            if self.column != 0 {
                self.column -= 1;
            }
        }
    }
    fn inc_pos(&mut self) {
        self.column += 1;
        if self.column >= SCREEN_WIDTH / 8 {
            self.newline()
        }
    }
    fn handle_utf8(&mut self, utf8: u32) {
        match utf8 {
            0x08 => self.backspace(),
            0x09 => self.print("    "),
            0x0a => {
                self.newline();
                if unsafe { SHELL.command_input } {
                    let command = self.chars[self.row - 1]
                        .get(2..SCREEN_WIDTH / 8 - 1)
                        .unwrap();
                    unsafe {
                        SHELL.command_input = false;
                        SHELL.run_command(command.try_into().unwrap());
                    }
                    crate::print!("> ");
                }
            }
            _ => self.print_utf8(utf8),
        }
    }
    pub fn clear_screen(&mut self) {
        let buffer = unsafe { self.buffer.as_mut().unwrap() };
        for y in 0..SCREEN_HEIGHT - 1 {
            for x in 0..SCREEN_WIDTH - 1 {
                buffer.chars[y][x] = 0x00;
            }
        }
    }
    const fn new() -> Screen {
        Screen {
            chars: [[0; SCREEN_WIDTH / 8]; SCREEN_HEIGHT / 16],
            column: 0,
            row: 0,
            font: None,
            buffer: 0xa0000 as *mut Buffer,
            color: 0x0F,
        }
    }
}

pub fn load_font(bytes: &'static [u8]) {
    unsafe { SCREEN.font = Some(psf_rs::Font::load(bytes)) };
}

impl core::fmt::Write for Screen {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}

pub fn _print(args: Arguments) {
    unsafe {
        SCREEN.write_fmt(args).unwrap();
    }
}
