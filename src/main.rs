extern crate csv;
#[macro_use]
extern crate itertools;
#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate fine_grained;

pub mod day1;
pub mod day2;

use std::io;
use std::collections::HashSet;
use std::convert::From;
use clap::{Arg,App};
use fine_grained::Stopwatch;
use day1::{Change,ChangeRecord,ChangeList};
use day2::{BoxID, BoxDiff};

#[derive(Debug)]
enum ResultValue {
	S(String),
	Num(i32)
}


fn process(day:i32, part:i32) -> ResultValue {
	let mut rdr = csv::ReaderBuilder::new()
		.has_headers(false)
		.from_reader(io::stdin());

	match day {
		1 => {
			let input:Vec<ChangeRecord> = rdr.deserialize().map(|v| v.unwrap()).collect();
			let cl = ChangeList::new(input);

			match part {
				1 => ResultValue::Num(cl.result()),
				2 => ResultValue::Num(cl.first_dup()),
				_ => panic!("Invalid part")
			}
		},
		2 => {
			let input:Vec<BoxID> = rdr.deserialize().map(|v| v.unwrap()).collect();

			match part {
				1 => {
					let twos:Vec<BoxID> = input.clone().into_iter().filter(|x| x.exactly_n(2)).collect();
					let threes:Vec<BoxID> = input.clone().into_iter().filter(|x| x.exactly_n(3)).collect();

					ResultValue::Num((twos.len() * threes.len()) as i32)
				},

				2 => {
					let ids = input.clone();
					let ids2 = input.clone();

					let result = iproduct!(ids, ids2)
						.map(|(l, r)| {
							(l.clone(), r.clone(), l.clone() - r.clone())
						})
						.filter(|(l, r, v)| {
							v.len() == 1
						})
						.collect::<Vec<(BoxID, BoxID, Vec<BoxDiff>)>>();

					assert!(result.len() == 2);

					ResultValue::S(result[0].0.clone() & result[0].1.clone())
				},
				_ => panic!("Invalid part")
			}
		}
		_ => unimplemented!()
	}
}

fn main() {
	let matches = App::new("Advent of Code 2018: day1")
		.version("1.0")
		.author("Cameron Reid <csreid.cr@gmail.com>")
		.arg(
			Arg::with_name("day")
				.short("d")
				.long("day")
				.help("Which day?")
				.takes_value(true)
				.required(true)
		)
		.arg(
			Arg::with_name("part")
				.short("p")
				.long("part")
				.help("Part one or part two?")
				.takes_value(true)
				.possible_values(&["1", "2"])
				.required(true)
		)
		.get_matches();

	

	let part:i32 = matches
		.value_of("part")
		.unwrap()
		.parse()
		.unwrap();

	let day:i32 = matches
		.value_of("day")
		.unwrap()
		.parse()
		.unwrap();

	let mut stopwatch = Stopwatch::start_new();
	let result = process(day, part);
	let time = stopwatch.lap();
	println!("{:?} in {}ns", result, time);
	stopwatch.stop();
}
