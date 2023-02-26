use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use walkdir::WalkDir;

use super::categories::get_categories;
use super::MyError;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModInfo {
    id: usize,
    parent_id: Option<usize>,
    metadata_path: String,
    ini_path: String,
    metadata: ModMetadata,
    local_images: Vec<String>,
    is_disabled: bool,
    is_merged: bool,
    children: Vec<String>,
    deep_children: Vec<ModInfo>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ModMetadata {
    #[serde(default)]
    info: Info,
    #[serde(default)]
    author: Author,
    #[serde(default)]
    categories: Vec<String>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    order: Option<u16>,
    #[serde(default)]
    like: bool,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Info {
    name: String,
    #[serde(default)]
    urls: Vec<String>,
    #[serde(default)]
    images: Vec<String>,
    #[serde(default)]
    videos: Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Author {
    #[serde(default)]
    name: String,
    #[serde(default)]
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
        // when the ini file is merged.ini, skip current dir, then handle merge logic
        if path.file_name() == merged {
            iter.skip_current_dir();
            if let Some(merge_mod) = handle_merge(path, ini, merged) {
                mod_list.push(merge_mod);
            }
            continue;
        }
        // when the ini file is not merged.ini, get the mod info
        if let Some(mod_info) = get_mod_info(path, false) {
            mod_list.push(mod_info);
        }
    }
    mod_list
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn generate_id() -> usize {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}

fn handle_merge(ini_path: &Path, ini: Option<&OsStr>, merged: Option<&OsStr>) -> Option<ModInfo> {
    let mut merged_info = get_mod_info(&ini_path, true)?;
    let mut deep_map: HashMap<PathBuf, ModInfo> = HashMap::new();
    let dir_path = ini_path.parent()?;

    for ini_path in WalkDir::new(dir_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_type().is_file() && e.path().extension() == ini && e.path().file_name() != merged
        })
        .map(|e| e.path().to_owned())
    {
        let dir_path = ini_path.parent()?;
        let parent_path = dir_path.parent()?;
        // for 1 level ini, just push the path to children
        if parent_path.join("merged.ini").exists() {
            merged_info.children.push(ini_path.display().to_string());
            continue;
        }

        // for 2 and more level ini, get the modinfo.json and push to deep_children
        if let Some(mod_info) = deep_map.get_mut(parent_path) {
            mod_info.children.push(ini_path.display().to_string());
            continue;
        }
        if let Some(mut mod_info) = get_mod_info(&ini_path, false) {
            mod_info.parent_id = Some(merged_info.id);
            deep_map.insert(parent_path.to_path_buf(), mod_info);
        }
    }

    merged_info.deep_children = deep_map.into_iter().map(|(_, v)| v).collect();
    Some(merged_info)
}

fn get_mod_info(ini_path: &Path, is_merged: bool) -> Option<ModInfo> {
    // get a modinfo.json from current path or parent path
    let dir_path = ini_path.parent()?;
    let metadata_path = if is_merged {
        ini_path.with_file_name("modinfo.json")
    } else {
        let parent_metadata_path = dir_path.parent()?.join("modinfo.json");
        if parent_metadata_path.exists() {
            parent_metadata_path
        } else {
            ini_path.with_file_name("modinfo.json")
        }
    };

    let mut metadata: ModMetadata = get_or_create_metadata(&metadata_path)?;
    let local_images = get_local_img(&metadata_path.parent()?);
    let metadata_path = metadata_path.display().to_string();
    let ini_path_str = ini_path.display().to_string();
    let is_disabled = ini_path_str.to_lowercase().contains("disabled");

    // if categories is empty, get categories from ini file name
    if metadata.categories.is_empty() {
        let ini_name = ini_path
            .file_stem()?
            .to_string_lossy()
            .replace("disabled", "")
            .replace("DISABLED", "");
        metadata.categories = get_categories(&ini_name);
    }

    Some(ModInfo {
        id: generate_id(),
        parent_id: None,
        metadata_path,
        ini_path: ini_path_str,
        metadata,
        local_images,
        is_disabled,
        is_merged,
        children: Vec::new(),
        deep_children: Vec::new(),
    })
}

fn get_or_create_metadata(path: &Path) -> Option<ModMetadata> {
    let contents = fs::read_to_string(path).ok().or_else(|| {
        // if modinfo.json not exist, create it
        let json_string = include_str!("modinfo.json");
        let mut file = File::create(path).ok()?;
        file.write_all(json_string.as_bytes()).ok()?;
        Some(json_string.to_owned())
    })?;
    let metadata: ModMetadata = serde_json::from_str(&contents).ok()?;
    Some(metadata)
}

#[cfg(target_os = "windows")]
const PROTOCOL: &str = "https://asset.localhost/";
#[cfg(not(target_os = "windows"))]
const PROTOCOL: &str = "asset://localhost/";

// convert file path to asset url
fn convert_file_src(file_path: &str) -> String {
    let path = urlencoding::encode(file_path);
    format!("{}{}", PROTOCOL, path)
}

const EXTS: &[&str] = &["png", "jpg", "jpeg", "jfif", "webp"];

fn get_local_img(path: &Path) -> Vec<String> {
    fs::read_dir(path)
        .map(|dir| {
            dir.filter_map(Result::ok)
                .filter_map(|entry| {
                    entry.path().extension().and_then(|extension| {
                        extension
                            .to_str()
                            .filter(|ext| EXTS.contains(&ext))
                            .map(|_| convert_file_src(&entry.path().display().to_string()))
                    })
                })
                .filter(|path| !path.ends_with("ShadowRamp.jpg"))
                .collect()
        })
        .unwrap_or_default()
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
