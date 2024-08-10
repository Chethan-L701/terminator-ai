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

pub fn initialize_context(savedir: &String) -> Result<()> {
    let context = Context { contents: vec![] };
    let contextpath = format!("{}/context.json", savedir);
    let mut contextfile = utils::overwrite(&contextpath)?;
    contextfile.write(&json!(context).to_string().into_bytes())?;
    return Ok(());
}
