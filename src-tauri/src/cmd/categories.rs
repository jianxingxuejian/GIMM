mod constant;
use constant::{
    CHARACTER_SET, ENEMY_SET, ENTITIES_SET, NPC_SET, OBJECT_SET, TCGCARD_SET, WEAPON_SET,
};
use tauri::utils::assets::phf;

struct Mapping {
    set: &'static phf::Set<&'static str>,
    category: &'static str,
}

const MAPPINGS: [Mapping; 7] = [
    Mapping {
        set: &CHARACTER_SET,
        category: "Character",
    },
    Mapping {
        set: &NPC_SET,
        category: "NPC",
    },
    Mapping {
        set: &ENEMY_SET,
        category: "Enemy",
    },
    Mapping {
        set: &OBJECT_SET,
        category: "Object",
    },
    Mapping {
        set: &TCGCARD_SET,
        category: "TCG_Card",
    },
    Mapping {
        set: &WEAPON_SET,
        category: "Weapon",
    },
    Mapping {
        set: &ENTITIES_SET,
        category: "Entities",
    },
];

pub fn get_categories(s: &str) -> Vec<String> {
    for mapping in MAPPINGS.iter() {
        if mapping.set.contains(s) {
            return vec![mapping.category.into(), s.into()];
        }
    }
    return vec!["other".into(), s.into()];
}
