use colored::*;
use curl::easy::{Easy, List};
use serde_json::{self, Value};
use std::env;
use std::ffi::CString;
use std::fs::File;
use std::io::{Read, Result, Write};

extern "C" {
    pub fn run(file: *const u8);
}

#[derive(Clone, Copy)]
enum DisplayMode {
    Normal,
    Plain,
    Raw,
    Hidden,
    Custom,
}

impl DisplayMode {
    fn cmd(self: &Self) -> Option<String> {
        match self {
            DisplayMode::Raw => Some(String::from("cat {}")),
            DisplayMode::Plain => Some(String::from("pandoc -t plain {}")),
            DisplayMode::Normal => Some(String::from("nvim -c \"set wrap\" {}")),
            DisplayMode::Hidden => None,
            DisplayMode::Custom => None,
        }
    }
}

#[derive(Clone)]
struct Flags {
    query: Option<String>,
    image: Option<String>,
    image_path: Option<String>,
    response: Option<String>,
    result: Option<String>,
    custom_command: Option<String>,
    display_mode: DisplayMode,
}

impl Flags {
    fn default() -> Self {
        Self {
            query: None,
            display_mode: DisplayMode::Normal,
            image: None,
            image_path: None,
            response: None,
            custom_command: None,
            result: None,
        }
    }

    fn parse(args: Vec<String>) -> Result<Self> {
        let mut flags = Flags::default();

        if let Ok(dir) = env::var("GEMINI_DIR") {
            flags.response = format!("{}/response.json", dir).into();
            flags.result = format!("{}/results.md", dir).into();
        } else {
            println!(
                "{} : GEMINI_DIR env is not set, using current directory {} ",
                "Warning".yellow(),
                std::env::current_dir()?.display()
            );
            flags.response = "response.json".to_string().into();
            flags.result = "result.md".to_string().into();
        }

        for (index, flag) in args.iter().enumerate() {
            match flag.as_str() {
                "--output" => {
                    flags.result = args[index + 1].clone().into();
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
                "--cwd" => {
                    flags.response = "response.json".to_string().into();
                    flags.result = "result.md".to_string().into();
                }
                "--open-with" => {
                    flags.display_mode = DisplayMode::Custom;
                    flags.custom_command = args[index + 1].clone().into();
                }
                "--plain" => flags.display_mode = DisplayMode::Plain,
                _ => {}
            }
        }
        Ok(flags)
    }
}

mod utils {

    use regex::Regex;
    use std::fs;
    use std::io::{ErrorKind, Result};
    use std::path;
    use std::process::Command;

    pub fn open(file_path: &String) -> Result<std::fs::File> {
        fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .append(true)
            .open(file_path)
    }

    pub fn overwrite(file_path: &String) -> Result<fs::File> {
        fs::remove_file(file_path)?;
        fs::OpenOptions::new()
            .create(true)
            .append(false)
            .write(true)
            .open(file_path)
    }

    pub fn process_newlines(input: &str) -> String {
        let placeholder = "__ESCAPED_N__";
        let re_escaped_n = Regex::new(r"\\\\n").unwrap();
        let intermediate = re_escaped_n.replace_all(&input, placeholder);

        let re_newline = Regex::new(r"\\n").unwrap();
        let result = re_newline.replace_all(&intermediate, "\n");

        let result = result.replace(placeholder, "\\n");
        return result;
    }

    pub fn read_image(path: &str) -> Result<String> {
        println!("Enter the image path :");
        let output = Command::new("base64")
            .arg("-w0")
            .arg(path.trim())
            .output()
            .unwrap();

        if output.status.success() {
            println!("Command exucuted successfully");
            let data = String::from_utf8(output.stdout.clone()).unwrap();
            return Ok(data);
        } else {
            return Err(ErrorKind::Other.into());
        }
    }

    pub fn get_absolute_path(path: &str) -> Result<String> {
        let path = path::Path::new(path);
        let absolute_path = path.canonicalize()?;
        Ok(absolute_path.to_str().unwrap().to_string())
    }
}

fn api_call(flags: Flags, api: String) -> Result<u32> {
    let url = format!(
        r#"https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}"#,
        api
    );

    let mut headers = List::new();
    headers.append("Content-Type: application/json")?;

    let mut response = utils::overwrite(&flags.response.unwrap())?;

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

    easy.post_fields_copy(&post_data.into_bytes())?;
    easy.perform()?;

    return Ok(easy.response_code()?);
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

fn display(flags: Flags) {
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

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("{} : No arguments provided", "Error".red());
        return Ok(());
    }

    let flags = Flags::parse(args)?;

    if let Ok(api) = env::var("GEMINI_API_KEY") {
        let response_status = api_call(flags.clone(), api)?;
        if response_status >= 200 || response_status < 300 {
            write_result(&flags)?;
            display(flags);
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
