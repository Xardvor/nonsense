use std::collections::HashMap;

type Processor = fn(&str) -> String;

pub struct Modifiers {
    mods: HashMap<String, Processor>,
    empty_mod: Processor,
}

impl Modifiers {
    fn is_vowel(c: char) -> bool {
        (c == 'a') || (c == 'e') || (c == 'i') || (c == 'o') || (c == 'u')
    }

    pub fn create() -> Modifiers {
        let nothing: Processor = |i: &str| -> String {String::from(i)};

        let capitalize: Processor = |input: &str| -> String {
            let mut iter = input.chars();
            match iter.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + iter.as_str(),
            }
        };

        let article: Processor = |input: &str| -> String {
            let mut iter = input.chars();
            match iter.next() {
                None => String::from(input),
                Some(v) if Modifiers::is_vowel(v) => {
                    if v == 'u' && input.chars().nth(2) == Some('i') {
                        return String::from("a ") + input;
                    };

                    String::from("an ") + input
                },
                _ => {
                    String::from("a ") + input
                }
            }
        };

        let plural: Processor = |input: &str| -> String {
            let iter = input.chars();
            match iter.last() {
                Some(c) if c == 's' || c == 'x' || c == 'h' => {
                    String::from(input) + "es"
                },
                Some('y') => {
                    let wo_y = &input[..input.len()-1];
                    match wo_y.chars().last() {
                        Some(v) if Modifiers::is_vowel(v) => {
                             String::from(input) + "s"
                        },
                        _ => String::from(wo_y) + "ies",
                    }
                },
                Some(_) => String::from(input) + "s",
                None => String::from(input),
            }
        };

        let mut map: HashMap<String, Processor> = HashMap::new();
        map.insert(String::from("capitalize"), capitalize);
        map.insert(String::from("a"), article);
        map.insert(String::from("s"), plural);

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
