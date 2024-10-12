use crate::config::flags::Flags;
use crate::utils;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{Read, Result, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Context {
    pub contents: Vec<Content>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Model,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: Role,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Part {
    Text { text: String },
    InlineData { inlineData: InlineData },
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineData {
    pub mimeType: String,
    pub data: String,
}

pub fn add_user_context_without_image(flags: &Flags) -> Result<()> {
    let contextpath = format!("{}/context.json", &flags.savedir.clone());
    let mut contextfile = utils::open(&contextpath)?;
    let mut contexttext = String::new();
    contextfile.read_to_string(&mut contexttext)?;
    let mut context: Context = serde_json::from_str(&contexttext)?;
    context.contents.push(Content {
        parts: vec![Part::Text {
            text: flags.query.clone().unwrap(),
        }],
        role: Role::User,
    });
    let mut contextfile = utils::overwrite(&contextpath)?;
    contextfile.write(&json!(context).to_string().into_bytes())?;
    return Ok(());
}

pub fn add_model_context(flags: &Flags, data: String) -> Result<()> {
    let contextpath = format!("{}/context.json", &flags.savedir.clone());
    let mut contextfile = utils::open(&contextpath)?;
    let mut contexttext = String::new();
    contextfile.read_to_string(&mut contexttext)?;
    let mut context: Context = serde_json::from_str(&contexttext)?;
    context.contents.push(Content {
        parts: vec![Part::Text { text: data }],
        role: Role::Model,
    });
    let mut contextfile = utils::overwrite(&contextpath)?;
    contextfile.write(&json!(context).to_string().into_bytes())?;
    return Ok(());
}

pub fn add_user_context_with_image(flags: &Flags) -> Result<()> {
    let contextpath = format!("{}/context.json", &flags.savedir.clone());
    let mut contextfile = utils::open(&contextpath)?;
    let mut contexttext = String::new();
    contextfile.read_to_string(&mut contexttext)?;
    let mut context: Context = serde_json::from_str(&contexttext)?;
    context.contents.push(Content {
        parts: vec![
            Part::Text {
                text: flags.query.clone().unwrap(),
            },
            Part::InlineData {
                inlineData: InlineData {
                    mimeType: format!(
                        "image/{}",
                        flags.image_path.clone().unwrap().split(".").last().unwrap()
                    )
                    .to_string(),
                    data: flags.image.clone().unwrap(),
                },
            },
        ],
        role: Role::User,
    });
    let mut contextfile = utils::overwrite(&contextpath)?;
    contextfile.write(&json!(context).to_string().into_bytes())?;
    return Ok(());
}

pub fn initialize_context(savedir: &String, term_mode: bool) -> Result<()> {
    let contextpath = format!("{}/context.json", savedir);
    let mut contextfile = utils::overwrite(&contextpath)?;
    if !term_mode {
        let context = Context { contents: vec![] };
        contextfile.write(&json!(context).to_string().into_bytes())?;
    } else {
        let executables = format!("{:?}", utils::executables()).replace("\\\\", "\\");
        let context = Context {
            contents: vec![Content {
                parts: vec![Part::Text {
                    text: format!(
                        r#"
from now on you are ai that runs on a terminal ,
when the user gives an input then try try find the appropriate command for the operation they want to do are, 
if the user gives a wrong command then you will correct the command by saying what the expected command is,
try to give the commands as a single command if multiple commands are nessecary then join them with ';' ,
and do not give any other text other then when explicitly said to explain the command given. 
The available commands in the path of user system is in the form of 2d array is carefully look at each and every one of the array entry and remember them: {}, 
the shell that is being used in nushell not powershell or windows cmd,
if the command that you want to run is not among the given command tell the user to install the appropriate exact command package or command they would need to run that command along with the that commnad.
example if the user need to display a image then if that command is not in the path executables provides then you will need say them to install Imagick along with the command to display the image.
Make sure you follow this rules:
1. Do not hallucinate , or give wrong commands that do not exist.
2. Always check again if the command you give is correct.
3. Check if the flags that you are providing is right, do give wrong flags that do not exists.
4. Use nushells built-in command instead of the Powershell.
"#,
                        executables.replace("\\\"", "\"")
                    ),
                }],
                role: Role::User,
            }],
        };
        contextfile.write(&json!(context).to_string().into_bytes())?;
    }

    return Ok(());
}
