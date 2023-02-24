mod constant;
use constant::{
    CHARACTER_MAP, ENEMY_MAP, ENTITIES_MAP, NPC_MAP, OBJECT_MAP, TCGCARD_MAP, WEAPON_MAP,
};
use tauri::utils::assets::phf;

struct Mapping {
    map: &'static phf::Map<&'static str, &'static str>,
    category: &'static str,
}

const MAPPINGS: [Mapping; 7] = [
    Mapping {
        map: &CHARACTER_MAP,
        category: "character",
    },
    Mapping {
        map: &NPC_MAP,
        category: "npc",
    },
    Mapping {
        map: &ENEMY_MAP,
        category: "enemy",
    },
    Mapping {
        map: &OBJECT_MAP,
        category: "object",
    },
    Mapping {
        map: &TCGCARD_MAP,
        category: "tcgcard",
    },
    Mapping {
        map: &WEAPON_MAP,
        category: "weapon",
    },
    Mapping {
        map: &ENTITIES_MAP,
        category: "entities",
    },
];

pub fn get_categories(s: &str) -> Vec<String> {
    for mapping in MAPPINGS.iter() {
        if let Some(value) = mapping.map.get(s) {
            return vec![mapping.category.into(), value.to_string()];
        }
    }
    return vec!["other".into(), s.into()];
}
