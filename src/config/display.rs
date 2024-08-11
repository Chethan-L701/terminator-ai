use crate::utils;
use crate::Config;
use crate::Flags;
use std::ffi::CString;
use std::io::Write;

extern "C" {
    pub fn run(file: *const u8);
}

#[derive(Clone, Copy)]
pub enum DisplayMode {
    Defualt,
    Raw,
    Config,
    Hidden,
    Custom,
}

impl DisplayMode {
    pub fn cmd(self: &Self, config: &Config, flags: &Flags) -> Option<String> {
        match self {
            DisplayMode::Raw => Some(String::from("cat {}")),
            DisplayMode::Defualt => Some(String::from("pandoc -t plain {}")),
            DisplayMode::Config => {
                if let Some(cmd) = &config.default_viewer {
                    Some(cmd.into())
                } else {
                    None
                }
            }
            DisplayMode::Custom => {
                let cmd = flags.custom_command.clone().unwrap();
                Some(cmd)
            }
            DisplayMode::Hidden => None,
        }
    }
}

pub fn display(flags: &Flags, config: &Config) {
    unsafe {
        match flags.display_mode {
            DisplayMode::Hidden => {}
            DisplayMode::Defualt => {
                let temppath = format!("{}/tempresult.md", &flags.savedir.clone());
                let mut tempresultfile = utils::overwrite(&temppath).unwrap();
                let _ = tempresultfile.write_all(&flags.resulttext.clone().unwrap().into_bytes());

                let cmd = CString::new(
                    flags
                        .display_mode
                        .clone()
                        .cmd(config, flags)
                        .unwrap()
                        .replace("{}", &temppath),
                );
                run(cmd.unwrap().to_bytes().as_ptr());
            }
            _ => {
                let cmd = CString::new(
                    flags
                        .display_mode
                        .clone()
                        .cmd(config, flags)
                        .unwrap()
                        .replace("{}", &flags.resultfile.clone().unwrap()),
                );
                run(cmd.unwrap().to_bytes().as_ptr());
            }
        }
    }
}
