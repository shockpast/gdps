#[allow(unused)]
pub struct Difficulty<'a> {
    pub is_auto: bool,
    pub is_demon: bool,
    pub difficulty: i32,
    pub name: &'a str,
}

#[allow(unused)]
pub struct Demon<'a> {
    pub difficulty: i32,
    pub name: &'a str,
}

pub fn get_difficulty_from_stars(stars: i32) -> Difficulty<'static> {
    match stars {
        1 => Difficulty {
            is_auto: true,
            is_demon: false,
            difficulty: 50,
            name: "Auto",
        },
        2 => Difficulty {
            is_auto: false,
            is_demon: false,
            difficulty: 10,
            name: "Easy",
        },
        3 => Difficulty {
            is_auto: false,
            is_demon: false,
            difficulty: 20,
            name: "Normal",
        },
        4..=5 => Difficulty {
            is_auto: false,
            is_demon: false,
            difficulty: 30,
            name: "Hard",
        },
        6..=7 => Difficulty {
            is_auto: false,
            is_demon: false,
            difficulty: 40,
            name: "Harder",
        },
        8..=9 => Difficulty {
            is_auto: false,
            is_demon: false,
            difficulty: 50,
            name: "Insane",
        },
        10 => Difficulty {
            is_auto: false,
            is_demon: true,
            difficulty: 50,
            name: "Demon",
        },
        _ => Difficulty {
            is_auto: false,
            is_demon: false,
            difficulty: 0,
            name: "N/A",
        },
    }
}

pub fn get_demon_from_index(index: i32) -> Demon<'static> {
    match index {
        1 => Demon {
            difficulty: 3,
            name: "Easy",
        },
        2 => Demon {
            difficulty: 4,
            name: "Medium",
        },
        3 => Demon {
            difficulty: 0,
            name: "Hard",
        },
        4 => Demon {
            difficulty: 5,
            name: "Insane",
        },
        5 => Demon {
            difficulty: 6,
            name: "Extreme",
        },
        _ => Demon {
            difficulty: 0,
            name: "Hard",
        },
    }
}
