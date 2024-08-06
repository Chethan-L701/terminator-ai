use crate::utils;
use crate::{config::configfile::Config, config::display::DisplayMode};
use std::io::Result;
use std::path::Path;

#[derive(Clone)]

pub struct Flags {
    pub query: Option<String>,
    pub image: Option<String>,
    pub image_path: Option<String>,
    pub response: Option<String>,
    pub result: Option<String>,
    pub custom_command: Option<String>,
    pub display_mode: DisplayMode,
    pub temp: bool,
}
impl Flags {
    pub fn default() -> Self {
        Self {
            query: None,
            display_mode: DisplayMode::Normal,
            image: None,
            image_path: None,
            response: None,
            custom_command: None,
            result: None,
            temp: false,
        }
    }

    pub fn parse(config: &Config, args: Vec<String>) -> Result<Self> {
        let mut flags = Flags::default();

        let basedir: String;
        let mut session: String;
        let mut output: String = String::new();

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
                    flags.image = utils::read_image(&flags.image_path.clone().unwrap())?.into();
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
                "--plain" => flags.display_mode = DisplayMode::Plain,
                _ => {}
            }
        }
        let session = session
            .split(' ')
            .map(|x| x.to_string())
            .fold(String::new(), |acc, x| acc + &x)
            .to_string();
        let session_path = format!("{}/{}", &basedir, &session);
        let path = Path::new(&session_path);
        utils::make_session(&path);

        if !flags.temp {
            flags.response = format!("{}/response.json", &session_path).into();
            flags.result = format!("{}/result.md", &session_path).into();
        } else {
            flags.response = "response.json".to_string().into();
            flags.response = "result.md".to_string().into();
        }

        if output != "" {
            flags.result = Some(output);
        }

        Ok(flags)
    }
}
