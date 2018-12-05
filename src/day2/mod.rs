use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug,Deserialize,Clone,Hash,PartialEq,Eq)]
pub struct BoxID (String);

impl BoxID {
	pub fn new(s:String) -> BoxID {
		BoxID(s)
	}

	pub fn exactly_n(&self, n:i32) -> bool {
		let mut chars = HashMap::new();

		for c in self.0.chars() {
			let tmp = chars.entry(c).or_insert(0);
			*tmp += 1;
		};

		chars.values().filter(|x| **x == n).collect::<Vec<&i32>>().len() > 0
	}
}
