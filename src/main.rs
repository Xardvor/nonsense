mod stories;
mod modifiers;

use std::string::*;
use std::vec::Vec;
use std::collections::HashMap;

fn main() {
	let origin: Vec<String> = vec![String::from("#mood and #mood, the #path was #mood with #substance"), String::from("#nearby #path #amove through the #path, filling me with #substance")];
	let nearby = vec![String::from("far away"), String::from("ahead"), String::from("behind me")];
	let substance = vec![String::from("light"), String::from("reflections"), String::from("mist"), String::from("shadow"), String::from("darkness"), String::from("brightness"), String::from("gaiety"), String::from("merriment")];
	let mood = vec![String::from("overcast"), String::from("alight"), String::from("clear"), String::from("darkened"), String::from("blue"), String::from("shadowed"), String::from("illuminated"), String::from("silver"), String::from("cool"), String::from("warm"), String::from("summer-warmed")];
	let path = vec![String::from("stream"), String::from("brook"), String::from("path"), String::from("ravine"), String::from("forest"), String::from("fence"), String::from("stone wall")];
	let amove = vec![String::from("spiral"), String::from("twirl"), String::from("curl"), String::from("dance"), String::from("twine"), String::from("weave"), String::from("meander"), String::from("wander"), String::from("flow")];

	let story: HashMap<String, Vec<String>> = [(String::from("origin"), origin),
	(String::from("nearby"), nearby),
	(String::from("substance"), substance),
	(String::from("mood"), mood),
	(String::from("path"), path),
	(String::from("amove"), amove)].iter().cloned().collect();

	let generator = stories::Storyteller::create(&story);
	let generated_story = generator.generate("origin");

    println!("{}",generated_story);
}
