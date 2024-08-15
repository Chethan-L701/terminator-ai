pub mod api;
pub mod config;
pub mod context;
pub mod utils;

use config::configfile::Config;
use std::env;
use std::io::Result;

use config::flags::Flags;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let userconf = Config::parse()?;
    let mut flags = Flags::parse(&userconf, args)?;
    let api = &userconf.api;
    let response_status = api::api_call(flags.clone(), api.into())?;

    if response_status >= 200 || response_status < 300 {
        api::write_result(&mut flags)?;
        config::display::display(&flags, &userconf);
    } else {
        println!("Request Failed");
    }

    Ok(())
}

// fn main() -> Result<()> {
//     let imgpath = String::from("C:/Users/Cheth/Pictures/neon.png");
//     utils::read_image(&imgpath)?;
//     return Ok(());
// }
