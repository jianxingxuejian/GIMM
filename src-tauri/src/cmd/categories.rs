use tauri::utils::assets::phf;

const CHARACTER_MAP: phf::Map<&'static str, &'static str> = phf::phf_map! {
  "AlbedoMod"=> "albedo",


};
const NPC_MAP: phf::Map<&'static str, &'static str> = phf::phf_map! {
  "AlbedoMod" => "albedo",
  "AloyMod" => "aloy",
  "AmberMod" => "amber",
};

pub fn get_categories(s: &str) -> Vec<String> {
    let value = CHARACTER_MAP.get(s);
    if let Some(value) = value {
        return vec![value.to_string()];
    }
    let value = NPC_MAP.get(s);
    if let Some(value) = value {
        return vec![value.to_string()];
    }
    return vec!["other".to_string()];
}
