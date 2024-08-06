use config::configfile::Config;
use core::time;
use curl::easy::{Easy, List};
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::{self, Value};
use std::env;
use std::fs::File;
use std::io::{Read, Result, Write};
use std::thread;
pub mod config;
pub mod utils;

use config::flags::Flags;

fn api_call(flags: Flags, api: String) -> Result<u32> {
    let url = format!(
        r#"https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}"#,
        api
    );

    let mut headers = List::new();
    headers.append("Content-Type: application/json")?;

    let mut response = utils::overwrite(&flags.response.clone().unwrap())?;
    let mut easy = Easy::new();
    easy.url(&url)?;
    easy.post(true)?;
    easy.http_headers(headers)?;
    easy.write_function(move |data| {
        let _ = response.write_all(data);
        Ok(data.len())
    })?;

    let post_data: String;
    if let Some(image) = &flags.image {
        post_data = format!(
            r#"
                {{
                    "contents":[
                        {{
                            "parts":[
                                {{
                                    "text":"{}"
                                }},
                                {{ 
                                    "inlineData": 
                                        {{
                                            "mimeType": "image/png",
                                            "data": "{}"
                                        }}
                                }}
                            ]
                        }}
                    ]
                }}"#,
            &flags.query.unwrap(),
            image
        );
    } else {
        post_data = format!(
            r#"
                {{
                    "contents":[
                        {{
                            "parts":[
                                {{
                                    "text":"{}"
                                }}
                            ]
                        }}
                     ]
                }}"#,
            &flags.query.unwrap()
        );
    }
    let response_code = std::sync::Arc::new(std::sync::Mutex::new(0));
    let response_code_clone = response_code.clone();
    easy.post_fields_copy(&post_data.into_bytes())?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⡿⣟⣯⣷⣾⣽⣻⢿")
            .template("{spinner} {msg}")
            .expect("Failed to set template"),
    );
    spinner.enable_steady_tick(time::Duration::from_millis(100));
    spinner.set_message("Fetching Result...");

    let handle = thread::spawn(move || {
        easy.perform().unwrap();
        let code = easy.response_code().unwrap();
        let mut response_code = response_code_clone.lock().unwrap();
        *response_code = code;
    });

    let _ = handle.join();
    spinner.finish_with_message("Done!");

    return Ok(*response_code.lock().unwrap());
}

fn write_result(flags: &Flags) -> Result<()> {
    let mut response_json = File::open(flags.response.clone().unwrap())?;
    let mut content = String::new();
    response_json.read_to_string(&mut content)?;

    let data: Value = serde_json::from_str(&content)?;

    let result = utils::process_newlines(
        &data["candidates"][0]["content"]["parts"][0]["text"]
            .to_string()
            .trim_matches('"'),
    )
    .replace("** ", "**")
    .replace(":", ": ")
    .replace("\\\"", "\"")
    .lines()
    .map(|x| x.to_string())
    .fold(String::new(), |acc, line| acc + &line + "\n")
    .to_string();

    let mut md = utils::open(&flags.result.clone().unwrap())?;

    if let Some(image_path) = &flags.image_path {
        md.write(
            &format!(
                r#"
# Prompt : {}
## Image :
![uploaded image]({})

"#,
                flags.query.clone().unwrap(),
                utils::get_absolute_path(&image_path)?
            )
            .into_bytes(),
        )?;
    } else {
        md.write(
            &format!(
                r#"
# Prompt : {}

"#,
                flags.query.clone().unwrap()
            )
            .into_bytes(),
        )?;
    }
    md.write_all(&result.into_bytes())?;
    md.write(
        &format!(
            r#"

"#
        )
        .into_bytes(),
    )?;

    return Ok(());
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let userconf = Config::parse()?;
    let flags = Flags::parse(&userconf, args)?;
    let api = &userconf.api;
    let response_status = api_call(flags.clone(), api.into())?;

    if response_status >= 200 || response_status < 300 {
        write_result(&flags)?;
        config::display::display(flags);
    } else {
        println!("Request Failed");
    }

    Ok(())
}
