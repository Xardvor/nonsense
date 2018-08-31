use std::collections::HashMap;

type Processor = fn(&str) -> String;

pub struct Modifiers {
    mods: HashMap<String, Processor>,
    empty_mod: Processor,
}

impl Modifiers {
    pub fn create() -> Modifiers {
        let nothing: Processor = |i: &str| -> String {String::from(i)};

        let capitalize: Processor = |input: &str| -> String {
            let mut iter = input.chars();
            match iter.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + iter.as_str(),
            }
        };

        let mut map: HashMap<String, Processor> = HashMap::new();
        map.insert(String::from("capitalize"), capitalize);

        Modifiers {
            mods: map,
            empty_mod: nothing,
        }
    }

    pub fn apply(&self, input: &str, mods_list: &Vec<String>) -> String {
        let mut to_process = String::from(input);
        for i in 0..mods_list.len() {
            let &modifier = match self.mods.get(&mods_list[i]) {
                None => {
                    println!("No such modifier!");
                    &self.empty_mod
                },
                Some(e) => e,
            };

            to_process = modifier(&to_process);
        };

        to_process
    }
}