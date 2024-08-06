use colored::*;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{BufRead, ErrorKind, Result},
    path::Path,
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api: String,
    pub basedir: Option<String>,
    pub default_session: Option<String>,
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
