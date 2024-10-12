use base64::display::Base64Display;
use colored::*;
use rand::{self, Rng};
use regex::Regex;
use sha256;
use std::fs;
use std::io::{self, Read, Result};
use std::iter;
use std::path::{self, Path};

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
        .read(true)
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

pub fn read_image(imgpath: &String) -> Result<(String, String)> {
    let mut file = fs::File::open(imgpath)?;
    let mut filebinary: Vec<u8> = vec![];
    file.read_to_end(&mut filebinary)?;
    let value = Base64Display::new(&filebinary, &base64::engine::general_purpose::STANDARD);
    let hash = generate_random_hash();
    return Ok((hash, value.to_string()));
}

pub fn get_absolute_path(path: &str) -> Result<String> {
    let path = path::Path::new(path);
    let absolute_path = path.canonicalize()?;
    Ok(absolute_path.to_str().unwrap().to_string())
}

pub fn make_session(savedir: &String, term_mode: bool) -> Result<()> {
    let path = Path::new(&savedir);
    if !path.exists() {
        let _ = fs::create_dir_all(path.to_str().unwrap());
        crate::context::initialize_context(&savedir.clone(), term_mode)?;
    };
    return Ok(());
}

pub fn list_sessions(basedir: &String) -> Result<()> {
    for session in fs::read_dir(basedir)? {
        let entry = session?.path();
        if entry.is_dir() {
            let sessionpath = entry.to_str().unwrap();
            let sessionname = sessionpath.split('\\').last().unwrap().to_string();
            let resultstr = format!("{}/result.md", sessionpath);
            let resultfile = path::Path::new(&resultstr);
            if resultfile.exists() {
                println!("{}", sessionname.blue());
            } else {
                println!("{}", sessionname.yellow());
            }
        }
    }
    std::process::exit(0);
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

fn find_executable(path: &String) -> Vec<String> {
    // COM;EXE;BAT;CMD;VBS;VBE;WSF;WSH;MSC;PS1;
    let pathtext: Vec<String> = "com;exe;bat;cmd;vbs;vbe;wsf;wsh;msc;ps1"
        .split(';')
        .map(|t| t.to_string())
        .collect();

    let mut executables: Vec<String> = vec![];

    for entry in fs::read_dir(path).unwrap() {
        let entrypath = entry.unwrap().path();
        if !entrypath.is_dir() {
            let file_type = entrypath
                .to_str()
                .unwrap()
                .split('.')
                .last()
                .unwrap()
                .to_string();
            if pathtext.contains(&file_type) {
                executables.push(entrypath.to_str().unwrap().to_string());
            }
        }
    }
    return executables;
}

pub fn executables() -> Vec<Vec<String>> {
    let pathvar = std::env::var("path").unwrap();
    let paths: Vec<String> = pathvar.split(';').map(|path| path.to_string()).collect();
    let mut executables: Vec<Vec<String>> = vec![];
    for pathstr in paths {
        let path = path::Path::new(&pathstr);
        if path.exists() {
            let pathexes = find_executable(&path.to_str().unwrap().to_string());
            if pathexes.len() > 0 {
                executables.push(pathexes);
            }
        }
    }
    return executables;
}
