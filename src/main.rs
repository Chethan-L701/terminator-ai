use colored::*;
use curl::easy::{Easy, List};
use regex::Regex;
use serde_json::{self, Value};
use std::env;
use std::ffi::CString;
use std::fs::{self, File};
use std::io::{Read, Result, Write};

fn open(file_path: &String) -> Result<std::fs::File> {
    fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .append(true)
        .open(file_path)
}

fn overwrite(file_path: &String) -> Result<fs::File> {
    fs::remove_file(file_path)?;
    fs::OpenOptions::new()
        .create(true)
        .append(false)
        .write(true)
        .open(file_path)
}

fn process_newlines(input: &str) -> String {
    let placeholder = "__ESCAPED_N__";
    let re_escaped_n = Regex::new(r"\\\\n").unwrap();
    let intermediate = re_escaped_n.replace_all(&input, placeholder);

    let re_newline = Regex::new(r"\\n").unwrap();
    let result = re_newline.replace_all(&intermediate, "\n");

    let result = result.replace(placeholder, "\\n");
    return result;
}

extern "C" {
    pub fn run(file: *const u8);
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut display = true;
    let mut prompt_set = false;
    let mut print_raw = false;
    let mut clear_recent = false;

    if args.len() <= 1 {
        println!("{} : No arguments provided", "Error".red());
        return Ok(());
    }

    let mut query: String = String::new();
    let mut headers = List::new();
    headers.append("Content-Type: application/json")?;

    if let Ok(api) = env::var("GEMINI_API_KEY") {
        let mut easy = Easy::new();
        let dir = env::var("GEMINI_DIR");

        let response_path: String;
        let mut result_path: String;
        if dir.is_err() {
            println!(
                "{} : GEMINI_DIR env is not set, using current directory {} ",
                "Warning".yellow(),
                std::env::current_dir()?.display()
            );

            response_path = "response.json".to_string();
            result_path = "result.md".to_string();
        } else {
            response_path = format!("{}/response.json", dir.clone().unwrap());
            result_path = format!("{}/results.md", dir.clone().unwrap());
        }

        for (index, flag) in args.iter().enumerate() {
            if flag == "--output" {
                result_path = args[index + 1].clone();
            } else if flag == "--no-display" {
                display = false;
            } else if flag == "--prompt" {
                prompt_set = true;
                query = args[index + 1].clone();
            } else if flag == "--raw" {
                display = false;
                print_raw = true;
            } else if flag == "--clear-old" {
                clear_recent = true;
            }
        }

        if !prompt_set {
            println!("{} : Prompt not given", "Error".red());
            return Ok(());
        }

        let url = format!(
            r#"https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}"#,
            api
        );

        let mut response = overwrite(&response_path)?;

        easy.url(&url)?;
        easy.post(true)?;
        easy.http_headers(headers)?;
        easy.write_function(move |data| {
            let _ = response.write_all(data);
            Ok(data.len())
        })?;

        let post_data: String = format!(r#"{{"contents":[{{"parts":[{{"text":"{}"}}]}}]}}"#, query);

        easy.post_fields_copy(&post_data.into_bytes())?;
        easy.perform()?;

        if easy.response_code().unwrap() >= 200 || easy.response_code().unwrap() < 300 {
            let mut contents = String::new();

            let mut response_json = File::open(response_path)?;
            response_json.read_to_string(&mut contents)?;

            let data: Value = serde_json::from_str(&contents)?;

            let result = process_newlines(
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

            if print_raw {
                println!("{}", result);
                return Ok(());
            }

            if clear_recent {
                fs::remove_file(&result_path)?;
            }

            let mut md = open(&result_path)?;
            md.write(
                &format!(
                    r#"
# Prompt : {}

"#,
                    query
                )
                .into_bytes(),
            )?;
            md.write_all(&result.into_bytes())?;
            md.write(
                &format!(
                    r#"

"#
                )
                .into_bytes(),
            )?;

            if display {
                unsafe {
                    let cmd = CString::new(format!(r#"nvim -c "set wrap" {}"#, &result_path));
                    run(cmd.unwrap().to_bytes().as_ptr());
                }
            }
        } else {
            println!("Request Failed");
        }
    } else {
        println!(
            "{} : Please set the GEMINI_API_KEY environment variable.",
            "Error".red()
        );
    }

    Ok(())
}
