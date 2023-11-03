use crate::{graphics, print, print_graphic, println, screen::SCREEN_WIDTH};

pub static mut SHELL: Shell = Shell::new();

pub struct Shell {
    pub command_input: bool,
    pub command_running: bool,
    pub read_keys: bool,
}
impl Shell {
    pub fn initialize_shell(&mut self) {
        print!("> ");
        self.command_input = true;
        self.read_keys = true;
    }
    pub fn run_command(&mut self, command: [u32; SCREEN_WIDTH / 8 - 3]) {
        self.command_running = true;
        match Commands::command_to_enum(command) {
            Commands::Lain => println!("Let's All Love Lain"),
            Commands::Cat => {
                let graphic = graphics::cat_graphic();
                print_graphic!(crate::graphics::load_gf(graphic));
                println!(":3");
            }
            Commands::Unknown => println!("Unknown Command"),
        }
        self.command_running = false;
        self.command_input = true;
    }
    const fn new() -> Shell {
        Shell {
            command_input: false,
            command_running: false,
            read_keys: false,
        }
    }
}

enum Commands {
    Lain,
    Cat,
    Unknown,
}
impl Commands {
    fn command_to_enum(command: [u32; SCREEN_WIDTH / 8 - 3]) -> Commands {
        if command == crate::str_to_command!("lain") {
            Commands::Lain
        } else if command == crate::str_to_command!("cat") {
            Commands::Cat
        } else {
            Commands::Unknown
        }
    }
}

#[macro_export]
macro_rules! str_to_command {
    ($x:expr) => {{
        let mut command = [0; SCREEN_WIDTH / 8 - 3];
        for (i, c) in $x.chars().enumerate() {
            command[i] = c as u32;
        }
        command
    }};
}
