use crate::utils;
use colored::*;
use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};
use std::{
    fs,
    io::{self, BufRead, ErrorKind, Result, Write},
    path::Path,
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api: String,
    pub basedir: Option<String>,
    pub default_session: Option<String>,
    pub default_viewer: Option<String>,
}

impl Config {
    pub fn parse() -> Result<Config> {
        let homedir = dir::home_dir().unwrap();
        let homestr = homedir.to_str().unwrap();
        let configpath = format!("{}/.config/terminator/config.json", homestr);
        let configfile = fs::read(&configpath);
        if let Ok(conf) = configfile {
            let conf: String = conf
                .lines()
                .fold(String::new(), |acc, x| acc + &x.unwrap() + "\n")
                .to_string();
            if let Ok(vals) = serde_json::from_str::<Config>(&conf) {
                if let Some(basedir) = &vals.basedir {
                    let path = Path::new(&basedir);
                    if !path.is_dir() {
                        let _ = fs::create_dir_all(basedir);
                    }
                }
                return Ok(Self {
                    api: vals.api,
                    basedir: vals.basedir,
                    default_session: vals.default_session,
                    default_viewer: vals.default_viewer,
                });
            } else {
                println!("{} : failed to parse the config file", "Error".red());
                return Err(ErrorKind::Other.into());
            }
        }
        println!(
            "{} : could not find the config file for terminator",
            "Error".red()
        );
        println!(
            "{} : {}",
            "To create a config file for use".yellow(),
            "terminator config".green()
        );
        println!("{}", "or".blue());
        println!(
            "{} : {}",
            "Create it manually at".yellow(),
            configpath.green()
        );
        return Err(ErrorKind::Other.into());
    }

    pub fn print(self: &Self) {
        println!("API KEY : {}", self.api);
        println!(
            "BASE DIR : {}",
            self.basedir.clone().unwrap_or("Not Set".to_string())
        );
        println!(
            "DEFUALT SESSION : {}",
            self.default_session
                .clone()
                .unwrap_or("NOT SET".to_string())
        );
    }
}

fn write_config(path: &String, content: &String) -> Result<()> {
    let mut configfile = utils::overwrite(&path)?;
    configfile.write_all(&content.clone().into_bytes())?;
    return Ok(());
}

pub fn create() -> Result<()> {
    let mut apikey = String::new();
    let mut basedir = String::new();
    let mut default_session = String::new();
    let mut default_viewer = String::new();

    const SCHEMA_URL: &str =
        "https://raw.githubusercontent.com/Chethan-L701/terminator-ai/main/src/config/schema.json";

    loop {
        println!("Enter the {} : ", "gemini api key".green());
        io::stdin().read_line(&mut apikey).unwrap();

        if apikey.trim() != "" {
            break;
        }
        println!("API KEY can not be empty!");
    }

    println!(
        "Enter the {}  (absolute path) to store the program session files: ",
        "base directory".green()
    );
    io::stdin().read_line(&mut basedir).unwrap();

    println!("Enter a name for the {}", "defautl session".green());
    io::stdin().read_line(&mut default_session).unwrap();

    println!(
        "Enter the program that you would like to open as the {}, use {{}} as the placeholder for the result file",
        "result viewer".green()
    );
    io::stdin().read_line(&mut default_viewer).unwrap();

    let mut config: Config = Config {
        api: apikey.trim().into(),
        basedir: None,
        default_session: None,
        default_viewer: None,
    };

    if basedir.trim() != "" {
        config.basedir = basedir.trim().to_string().into();
    }

    if default_viewer.trim() != "" {
        config.default_viewer = default_viewer.trim().to_string().into();
    }

    if default_session.trim() != "" {
        config.default_session = default_session.trim().to_string().into();
    }

    let mut val: Value = json!(config);
    val["$schema"] = SCHEMA_URL.into();

    let config = val.to_string();

    let homedir = dir::home_dir().unwrap();
    let homestr = homedir.to_str().unwrap();
    let configpath = format!("{}/.config/terminator/config.json", homestr);

    let configpath = Path::new(&configpath);

    if configpath.exists() {
        println!(
            "The config file seems to already exists, do you want to re-write the config file?[Y/_]"
        );

        let mut ch = String::new();
        io::stdin().read_line(&mut ch).unwrap();

        match ch.trim() {
            "y" | "Y" => {
                println!("re-writing the config!");
                write_config(&configpath.to_str().unwrap().into(), &config)?;
            }
            _ => {}
        };
    } else {
        println!("writing the config at {}: ", &configpath.to_str().unwrap());
        write_config(&configpath.to_str().unwrap().into(), &config)?;
    }

    std::process::exit(0);
}
