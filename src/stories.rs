extern crate rand;

use std::string::*;
use std::vec::Vec;
use std::collections::HashMap;
use self::rand::*;

use modifiers::Modifiers;

pub struct Storyteller {
	tokens: HashMap<String, Vec<String>>,
	modifiers: Modifiers,
}

impl Storyteller {
	pub fn create(pieces: &HashMap<String, Vec<String>>) -> Storyteller {
	    Storyteller {
	    	tokens: pieces.clone(),
	    	modifiers: Modifiers::create(),
	    }
	}

	pub fn generate(&self, start: &str) -> String {
		self.process_token(start, &Vec::new())
	}

	fn process_token(&self, token: &str, mods: &Vec<String>) -> String {
		let variants = self.tokens.get(token).expect("Wrong token!");

		let index = rand::thread_rng().gen_range(0, variants.len());
		let mut choosed_token: String = match variants.get(index) {
			None => {
				println!("Wrong random index!");
				String::new()
			},
			Some(s) => s.clone()
		};

		choosed_token = self.modifiers.apply(&choosed_token, mods);

		let mut piece = String::new();
		let iter = choosed_token.split_whitespace();
		for word in iter {
			if word.starts_with(&"#") {
				let mut has_punctuation: bool = false;
				let mut punctuation: String = String::new();
				if word.ends_with(|c| ".,?!".contains(c)) {
					let mut matches = word.rsplit(char::is_alphanumeric);
					match matches.next() {
						None => {},
						Some(p) => {
							has_punctuation = true;
							punctuation = String::from(p);
						},
					};
				}
				let mut extracted = self.extract_token(word);
				piece.push_str(&self.process_token(&extracted.0, &extracted.1));
				if has_punctuation {
					piece.push_str(&punctuation);
				}
			} else {
				piece.push_str(word);
			};

			piece.push(' ');
		};

		piece.pop();

		piece
	}

	fn extract_token(&self, input: &str) -> (String, Vec<String>) { //(token itself and modifiers)
		if !input.starts_with(&"#") {
			println!("Not a token!");
			return (String::from(input), Vec::new())
		};

		let mut pieces = input.split(|c:char| c.is_ascii_punctuation() ).skip(1);
		let token = pieces.next().expect("Empty token!");

		let mut mods: Vec<String> = Vec::new();
		for modifier in pieces {
			if !modifier.is_empty() {
				mods.push(String::from(modifier));
			}
		};

		(String::from(token), mods)
	}
}
