#![no_std]

pub mod graphics;
pub mod screen;
pub mod shell;

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (crate::screen::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! print_graphic {
    ($arg:expr) => {
        unsafe { crate::screen::SCREEN.print_graphic($arg) }
    };
}
#[macro_export]
macro_rules! run_command {
    ($arg:expr) => {
        shell::run_command(shell::str_to_command!($arg));
    };
}
