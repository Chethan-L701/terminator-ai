use crate::Flags;
use std::ffi::CString;

extern "C" {
    pub fn run(file: *const u8);
}

#[derive(Clone, Copy)]
pub enum DisplayMode {
    Normal,
    Plain,
    Raw,
    Hidden,
    Custom,
}

impl DisplayMode {
    pub fn cmd(self: &Self) -> Option<String> {
        match self {
            DisplayMode::Raw => Some(String::from("cat {}")),
            DisplayMode::Plain => Some(String::from("pandoc -t plain {}")),
            DisplayMode::Normal => Some(String::from("nvim -c \"set wrap\" {}")),
            DisplayMode::Hidden => None,
            DisplayMode::Custom => None,
        }
    }
}

pub fn display(flags: Flags) {
    unsafe {
        match flags.display_mode {
            DisplayMode::Custom => {
                let cmd = CString::new(
                    flags
                        .custom_command
                        .clone()
                        .unwrap()
                        .replace("{}", &flags.result.clone().unwrap()),
                );
                run(cmd.unwrap().to_bytes().as_ptr());
            }
            DisplayMode::Hidden => {}
            _ => {
                let cmd = CString::new(
                    flags
                        .display_mode
                        .cmd()
                        .unwrap()
                        .replace("{}", &flags.result.unwrap()),
                );
                run(cmd.unwrap().to_bytes().as_ptr());
            }
        }
    }
}
