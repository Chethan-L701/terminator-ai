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
    let res_path = path::Path::new(file_path);
    if res_path.exists() {
        fs::remove_file(res_path)?;
    }
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

pub fn make_session(path: &path::Path) {
    if !path.is_dir() {
        let _ = fs::create_dir_all(path.to_str().unwrap());
    };
}
