#![no_std]

mod modules {
    pub mod graphics;
    pub mod screen;
    pub mod shell;
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (crate::print!("{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (crate::modules::screen::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! print_graphic {
    ($arg:expr) => {
        unsafe { crate::modules::screen::SCREEN.print_graphic($arg) }
    };
}
#[macro_export]
macro_rules! run_command {
    ($arg:expr) => {
        shell::run_command(shell::str_to_command!($arg));
    };
}
