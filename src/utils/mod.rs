use colored::*;
use rand::{self, Rng};
use regex::Regex;
use sha256;
use std::fs;
use std::io::{self, ErrorKind, Result};
use std::iter;
use std::path::{self, Path};
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

pub fn read_image(path: &str) -> Result<(String, String)> {
    let output = Command::new("base64")
        .arg("-w0")
        .arg(path.trim())
        .output()
        .unwrap();

    if output.status.success() {
        let data = String::from_utf8(output.stdout.clone()).unwrap();
        let hash = generate_random_hash();
        return Ok((hash, data));
    } else {
        let data = String::from_utf8(output.stderr.clone()).unwrap();
        println!("{} :\n{}", "Base64 Error".red(), data);
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

fn generate_random_hash() -> String {
    let mut rng = rand::thread_rng();
    let random_string: String = iter::repeat(())
        .map(|()| rng.sample(rand::distributions::Alphanumeric))
        .take(30)
        .map(char::from)
        .collect();

    let hash = sha256::digest(random_string);

    return hash;
}

pub fn copy_image(source: &String, savedir: &String, hash: &String) -> Result<String> {
    let savefile = format!(
        "{}/images/{}.{}",
        savedir,
        hash,
        source.split('.').last().unwrap()
    );
    println!("source : {}\ndest : {}", source, savefile);
    fs::create_dir_all(format!("{}/images", savedir))?;
    fs::copy(source, &savefile)?;
    return Ok(format!(
        "./images/{}.{}",
        hash,
        source.split('.').last().unwrap()
    ));
}

pub fn delete_session(path: &String, session: &String) -> Result<()> {
    println!("path : {}", path);
    let dir = Path::new(&path);
    if dir.exists() {
        println!(
            "Do you really want to {} the session {}?[Y/N]",
            "delete".red(),
            session.yellow()
        );
        let mut ch = String::new();
        io::stdin().read_line(&mut ch)?;
        let ch: String = ch.trim().to_string();
        match &*ch {
            "Y" | "y" => {
                fs::remove_dir_all(&dir.to_str().unwrap())?;
                println!("`{}` session was deleted successfully.", session.red());
            }
            _ => {
                println!("`{}` session was not deleted.", session.green());
            }
        };
    } else {
        println!("Are you sure there is a session called {}?", session.blue());
    }
    std::process::exit(0);
}
