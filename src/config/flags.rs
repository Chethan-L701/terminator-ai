use colored::Colorize;

use crate::utils;
use crate::{config::configfile::Config, config::display::DisplayMode};
use std::io::Result;

use super::configfile;

#[derive(Clone)]

pub struct Flags {
    pub query: Option<String>,
    pub image: Option<String>,
    pub image_path: Option<String>,
    pub responsefile: Option<String>,
    pub resultfile: Option<String>,
    pub resulttext: Option<String>,
    pub custom_command: Option<String>,
    pub savedir: String,
    pub display_mode: DisplayMode,
    pub temp: bool,
    pub imghash: Option<String>,
    pub delete: bool,
}
impl Flags {
    pub fn default() -> Self {
        Self {
            query: None,
            display_mode: DisplayMode::Defualt,
            image: None,
            image_path: None,
            responsefile: None,
            custom_command: None,
            savedir: "".into(),
            resultfile: None,
            resulttext: None,
            imghash: None,
            temp: false,
            delete: false,
        }
    }

    pub fn parse(config: &Config, args: Vec<String>) -> Result<Self> {
        let mut flags = Flags::default();

        let basedir: String;
        let mut session: String;
        let mut output: String = String::new();

        if args.len() <= 1 {
            println!("{} : Provide arguments to the program!", "Error".red());
            std::process::exit(0);
        }

        if args[1] == "config" {
            configfile::create()?;
        }

        match &config.basedir {
            Some(dir) => {
                basedir = dir.into();
            }
            None => {
                basedir = ".".into();
            }
        }

        match &config.default_session {
            Some(sess) => {
                session = sess.into();
            }
            None => session = "".into(),
        }

        if let Some(_) = &config.default_viewer {
            flags.display_mode = DisplayMode::Config
        }

        for (index, flag) in args.iter().enumerate() {
            match flag.as_str() {
                "--output" => {
                    output = args[index + 1].clone().into();
                }
                "--no-display" => {
                    flags.display_mode = DisplayMode::Hidden;
                }
                "--prompt" => {
                    flags.query = args[index + 1].clone().into();
                }
                "--raw" => {
                    flags.display_mode = DisplayMode::Raw;
                }
                "--image" => {
                    flags.image_path = args[index + 1].clone().into();
                    let (hash, image_data): (String, String) =
                        utils::read_image(&flags.image_path.clone().unwrap())?.into();
                    flags.image = image_data.into();
                    flags.imghash = hash.into();
                }
                "--temp" => {
                    flags.temp = true;
                }
                "--open-with" => {
                    flags.display_mode = DisplayMode::Custom;
                    flags.custom_command = args[index + 1].clone().into();
                }
                "--session" => {
                    session = args[index + 1].clone().into();
                }
                "--custom" => {
                    flags.display_mode = DisplayMode::Custom;
                    flags.custom_command = args[index + 1].clone().into()
                }
                "--delete" => flags.delete = true,
                _ => {}
            }
        }

        let session_path = format!("{}/{}", &basedir, &session);
        let session = session
            .split(' ')
            .map(|x| x.to_string())
            .fold(String::new(), |acc, x| acc + &x)
            .to_string();

        if flags.delete {
            utils::delete_session(&session_path, &session)?;
        }

        flags.savedir = session_path.clone();
        utils::make_session(&flags)?;

        if !flags.temp {
            flags.responsefile = format!("{}/response.json", &session_path).into();
            flags.resultfile = format!("{}/result.md", &session_path).into();
        } else {
            flags.responsefile = "response.json".to_string().into();
            flags.responsefile = "result.md".to_string().into();
        }

        if let Some(image_path) = &flags.image_path.clone() {
            flags.image_path = utils::copy_image(
                image_path,
                &flags.savedir.clone(),
                &flags.imghash.clone().unwrap(),
            )?
            .into();
            println!("{}", &flags.image_path.clone().unwrap());
        }

        if output != "" {
            flags.resultfile = Some(output);
        }

        Ok(flags)
    }
}
