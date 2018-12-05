use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Sub;
use std::ops::BitAnd;
use std::fmt::{Display,Result,Formatter};

#[derive(Debug,Deserialize,Clone,Hash,PartialEq,Eq,PartialOrd,Ord)]
pub struct BoxID (String);
pub struct BoxDiff {
	position: i32,
	left: char,
	right: char
}

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

		chars
			.values()
			.filter(|x| **x == n)
			.collect::<Vec<&i32>>()
			.len() > 0
	}
}

impl Sub<BoxID> for BoxID {
	type Output = Vec<BoxDiff>;

	fn sub(self, other:BoxID) -> Self::Output {
		self.0.chars()
			.zip(other.0.chars())
			.enumerate()
			.filter(|(idx, (l, r))| {
				l != r
			})
			.map(|(idx, (l, r))| BoxDiff {
				position: idx as i32,
				left: l,
				right: r
			})
			.collect::<Vec<BoxDiff>>()
	}
}

impl BitAnd<BoxID> for BoxID {
	type Output = String;

	fn bitand(self, other:BoxID) -> Self::Output {
		self.0.chars()
			.zip(other.0.chars())
			.filter(|(l, r)| {
				l == r
			})
			.map(|(_, r)| r)
			.collect::<String>()
	}
}

impl Display for BoxDiff {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{} -> {} as position {}", self.left, self.right, self.position);
		Ok(())
	}
}
