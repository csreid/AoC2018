use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
	Positive,
	Negative
}

#[derive(Debug,Deserialize,Clone,Hash,PartialEq,Eq)]
pub struct ChangeRecord (String);

#[derive(Debug)]
pub struct Change {
	magnitude: i32,
	direction: Direction
}

impl From<ChangeRecord> for Change {
	fn from(item: ChangeRecord) -> Self {
		let value:String = item.0.chars().skip(1).collect();

		match item.0.chars().next().unwrap() {
			'+' => {
				Change {
					direction: Direction::Positive,
					magnitude: value.parse().unwrap()
				}
			}
			'-' => {
				Change {
					direction: Direction::Negative,
					magnitude: value.parse().unwrap()
				}
			}
			_ => panic!("Something bad happened")
		}
	}
}

#[derive(Clone)]
pub struct ChangeList {
	changes: Vec<ChangeRecord>
}

impl ChangeList {
	pub fn new(list:Vec<ChangeRecord>) -> ChangeList {
		ChangeList { changes: list }
	}
	pub fn result(self) -> i32 {
		self.changes.iter()
			.fold(0, |acc, val| {
				let value:Change = val.clone().into();
				match value.direction {
					Direction::Positive => acc + value.magnitude,
					Direction::Negative => acc - value.magnitude
				}
			})
	}

	pub fn first_dup(self) -> i32 {
		let mut already_seen:HashSet<i32> = HashSet::with_capacity(20000);
		let mut iters = 0;

		let val = self.changes.iter()
			.cycle()
			.scan(0, |acc, c| {
				let value:Change = c.clone().into();

				*acc = match value.direction {
					Direction::Positive => *acc + value.magnitude,
					Direction::Negative => *acc - value.magnitude
				};

				Some(*acc)
			})
			.find(|x| {
				iters += 1;
				if already_seen.contains(x) {
					true
				} else {
					already_seen.insert(*x);
					false
				}
			}).unwrap();
		println!("{}", iters);
		val
	}
}
