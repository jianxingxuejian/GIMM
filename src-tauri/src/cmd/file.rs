use serde::{Deserialize, Serialize};
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use walkdir::WalkDir;

use super::categories::get_categories;
use super::MyError;

#[derive(Serialize, Deserialize)]
pub struct ModInfo {
    id: usize,
    parent_id: Option<usize>,
    path: String,
    metadata: ModMetadata,
    local_images: Vec<String>,
    is_merged: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ModMetadata {
    info: Info,
    author: Author,
    categories: Vec<String>,
    tags: Vec<String>,
    order: Option<u16>,
    like: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Info {
    name: String,
    urls: Vec<String>,
    images: Vec<String>,
    videos: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    name: String,
    urls: Vec<String>,
}

pub fn get_mod_list(path: String) -> Vec<ModInfo> {
    let mut mod_list = Vec::new();
    let path = Path::new(&path);
    let ini = Some(OsStr::new("ini"));
    let merged = Some(OsStr::new("merged.ini"));
    // Iterate the ini file in the mods folder, and sort the merged.ini file to the front
    let mut iter = WalkDir::new(path)
        .sort_by(|a, b| {
            if a.file_name() == "merged.ini" {
                std::cmp::Ordering::Less
            } else if b.file_name() == "merged.ini" {
                std::cmp::Ordering::Greater
            } else {
                a.file_name().cmp(b.file_name())
            }
        })
        .into_iter();
    while let Some(entry) = iter.next() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        let path = entry.path();
        if path.extension() != ini {
            continue;
        }
        // when the ini file is merged.ini, skip current dir
        if path.file_name() == merged {
            iter.skip_current_dir();
            if let Some(parent) = path.parent() {
                mod_list.extend(handle_merge(parent));
            }
            continue;
        }
        // when the ini file is not merged.ini, get the modinfo.json
        if let Some(mod_info) = get_or_create_info(path) {
            mod_list.push(mod_info);
        }
    }
    mod_list
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn generate_id() -> usize {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}

fn handle_merge(path: &Path) -> Vec<ModInfo> {
    let mut mod_list = Vec::new();
    let walker = WalkDir::new(path);
    // todo
    return mod_list;
}

fn get_or_create_info(path: &Path) -> Option<ModInfo> {
    let metadata_path = path.with_file_name("modinfo.json");
    let contents = if metadata_path.exists() {
        let mut file = File::open(&metadata_path).ok()?;
        let mut json_string = String::new();
        file.read_to_string(&mut json_string).ok()?;
        json_string
    } else {
        let json_string = include_str!("modinfo.json");
        let mut file = File::create(&metadata_path).ok()?;
        file.write(json_string.as_bytes()).ok();
        json_string.to_string()
    };
    let mut metadata: ModMetadata = serde_json::from_str(&contents).ok()?;
    if metadata.categories.len() == 0 {
        let ini_name = path.file_stem().unwrap().to_string_lossy().to_string();
        metadata.categories = get_categories(&ini_name);
    }
    let mod_parse = ModInfo {
        id: generate_id(),
        parent_id: None,
        path: metadata_path.display().to_string().replace("\\", "/"),
        metadata,
        local_images: Vec::new(),
        is_merged: false,
    };
    Some(mod_parse)
}

fn is_deep_merge(path: &Path, ini: Option<&OsStr>) -> Option<bool> {
    let entries = fs::read_dir(path).ok()?;
    for entry in entries {
        let path = entry.ok()?.path();
        if !path.is_dir() {
            continue;
        }

        let entries = fs::read_dir(path).ok()?;
        for entry in entries {
            if entry.ok()?.path().extension() == ini {
                return Some(false);
            }
        }
    }
    return Some(true);
}

pub fn rename(path: String, new_path: String) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&path);
    fs::rename(path, new_path)?;
    Ok(())
}

pub fn write_file(path: String, contents: String) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&path);
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

const EXTS: &'static [&'static str] = &["png", "jpg", "jpeg", "jfif"];

fn read_local_img(path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let mut local_img = Vec::new();
    let dir = fs::read_dir(path)?;
    for entry in dir {
        let path = entry?.path();
        if !path.is_file() {
            continue;
        }
        let extension = path
            .extension()
            .and_then(OsStr::to_str)
            .ok_or(MyError::IOError)?;
        if EXTS.contains(&extension) {
            local_img.push(path.display().to_string());
        }
    }
    Ok(local_img)
}

pub fn unzip(path: &Path) -> Result<(), Box<dyn Error>> {
    let zip_file = File::open(path)?;
    let mut archive = zip::ZipArchive::new(zip_file)?;
    let target = path.with_extension("");
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        if file.is_dir() {
            let target = target.join(Path::new(&file.name().replace("\\", "")));
            fs::create_dir_all(target)?;
        } else {
            let file_path = target.join(Path::new(file.name()));
            fs::create_dir_all(file_path.parent().ok_or(MyError::IOError)?)?;
            let mut target_file = if !file_path.exists() {
                File::create(file_path)?
            } else {
                File::open(file_path)?
            };
            io::copy(&mut file, &mut target_file)?;
        }
    }
    Ok(())
}

pub fn unrar(path: &Path) -> Result<(), Box<dyn Error>> {
    let target = path.with_extension("");
    fs::create_dir_all(&target)?;
    let target = target
        .as_path()
        .to_str()
        .ok_or(MyError::IOError)?
        .to_string();
    let archive = unrar::Archive::new(path.to_str().ok_or(MyError::IOError)?.to_string());

    archive
        .extract_to(target)
        .map_err(|_| MyError::RarError)?
        .process()
        .map_err(|_| MyError::RarError)?;
    Ok(())
}

pub fn un7z(path: &Path) -> Result<(), Box<dyn Error>> {
    let target = path.with_extension("");
    fs::create_dir_all(&target)?;
    sevenz_rust::decompress_file(path, target).expect("complete");
    Ok(())
}
