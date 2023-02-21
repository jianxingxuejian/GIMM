use std::error::Error;
use std::path::Path;
use std::{collections::HashMap, fmt};

use self::file::ModInfo;

pub mod file;

type CmdResult<T = ()> = Result<T, String>;

#[macro_export]
macro_rules! wrap_result {
    ($stat: expr) => {
        match $stat {
            Ok(a) => Ok(a),
            Err(err) => Err(format!("{}", err.to_string())),
        }
    };
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::IOError => write!(f, "IOError"),
            MyError::RarError => write!(f, "RarError"),
        }
    }
}

impl Error for MyError {}

#[derive(Debug)]
pub enum MyError {
    IOError,
    RarError,
}

#[tauri::command]
pub fn get_mod_list(path: String) -> HashMap<String, ModInfo> {
    return file::get_mod_list(path);
}

#[tauri::command]
pub fn rename(path: String, new_path: String) -> CmdResult {
    let result = file::rename(path, new_path);
    wrap_result!(result)
}

#[tauri::command]
pub fn write_file(path: String, contents: String) -> CmdResult {
    let result = file::write_file(path, contents);
    wrap_result!(result)
}

#[tauri::command]
pub fn extract(path: String) -> CmdResult {
    let path = Path::new(&path);
    let extension = path
        .extension()
        .ok_or("Extension error")?
        .to_str()
        .ok_or("Os to str error")?;
    match extension {
        "zip" => file::unzip(path),
        "rar" => file::unrar(path),
        "7z" => file::un7z(path),
        _ => Err("Unsupported extension".into()),
    }
    .ok();
    Ok(())
}
