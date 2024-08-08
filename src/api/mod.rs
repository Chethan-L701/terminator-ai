use core::time;
use curl::easy::{Easy, List};
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::{self, json, Value};
use std::fs::File;
use std::io::{Read, Result, Write};
use std::thread;

use crate::{config::flags::Flags, context, utils};

pub fn api_call(flags: Flags, api: String) -> Result<u32> {
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
    let contextpath = format!("{}/context.json", flags.savedir.clone());
    let mut contextfile = utils::open(&contextpath)?;

    let mut contextdata = String::new();
    contextfile.read_to_string(&mut contextdata)?;
    let mut context: context::Context = serde_json::from_str(&contextdata)?;

    if let Some(image) = &flags.image {
        context.contents.push(context::Content {
            parts: vec![
                context::Part::Text {
                    text: flags.query.clone().unwrap(),
                },
                context::Part::InlineData {
                    inlineData: context::InlineData {
                        mimeType: format!(
                            "image/{}",
                            flags.image_path.clone().unwrap().split('.').last().unwrap()
                        ),
                        data: image.to_string(),
                    },
                },
            ],
            role: context::Role::User,
        });
    } else {
        context.contents.push(context::Content {
            parts: vec![context::Part::Text {
                text: flags.query.clone().unwrap(),
            }],
            role: context::Role::User,
        });
    }

    let response_code = std::sync::Arc::new(std::sync::Mutex::new(0));
    let response_code_clone = response_code.clone();
    easy.post_fields_copy(json!(context).to_string().as_bytes())?;

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
    let response_code = *response_code.lock().unwrap();

    if response_code >= 200 && response_code <= 299 {
        if let Some(_) = &flags.image {
            context::add_user_context_with_image(&flags)?;
        } else {
            context::add_user_context_without_image(&flags)?;
        }
    }

    return Ok(response_code);
}

pub fn write_result(flags: &Flags) -> Result<()> {
    let mut response_json = File::open(flags.response.clone().unwrap())?;
    let mut content = String::new();
    response_json.read_to_string(&mut content)?;

    let data: Value = serde_json::from_str(&content)?;

    let rawtext = data["candidates"][0]["content"]["parts"][0]["text"].to_string();
    let result_data = &rawtext.trim_matches('"');
    context::add_model_context(flags, result_data.to_string())?;
    let result = utils::process_newlines(result_data)
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
                image_path
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
