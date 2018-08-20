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
		self.process_token(start)
	}

	fn process_token(&self, token: &str) -> String {
		let variants = self.tokens.get(token).expect("Wrong token!");

		let index = rand::thread_rng().gen_range(0, variants.len());
		let choosed_token = variants.get(index).expect("Wrong random index!");

		let mut piece = String::new();
		let iter = choosed_token.split_whitespace();
		for word in iter {
			if word.starts_with(&"#") {
				let mut processed_token = String::from(word);
				processed_token.retain(|c| c.is_alphabetic());
				piece.push_str(&self.process_token(&processed_token));
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



		(String::from(input), Vec::new())
	}
}