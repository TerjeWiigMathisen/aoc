// Acer 5976 us

//use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

struct Patterns<'a> {
    patterns:HashSet<&'a str>,
    maxpathlen:usize,
    tails:HashMap<&'a str, usize>,
}

fn ways<'a>(need:&'a str, pat:&mut Patterns<'a>) -> usize
{
    if let Some(n) = pat.tails.get(need) { return *n;}

	let mut w = 0;
    let len = if need.len() < pat.maxpathlen {need.len()} else {pat.maxpathlen};
	for i in 1..=len {
		let t = &need[0..i];
		if pat.patterns.contains(t) {
            if i == need.len() { w += 1; }
            else {
    			let rem = &need[i..];
	    		w += ways(&rem, pat);
            }
		}
	}
	pat.tails.insert(need, w);
	w
}

fn process(inp:&str) -> (usize, usize)
{
    let mut pat = Patterns{patterns:HashSet::new(), maxpathlen:0, tails:HashMap::new()};
    let (patterns, targets) = inp.split_once("\n\n").unwrap();
    let pats = patterns.split(", ");
    for p in pats {
        if p.len() > pat.maxpathlen { pat.maxpathlen = p.len();} 
        pat.patterns.insert(p); 
    }
    let mut part1 = 0;
    let mut part2 = 0;
    for target in targets.lines() {
        let w = ways(target, &mut pat);
//        println!("{target} -> {w}");
        part1 += (w > 0) as usize;
        part2 += w;
        pat.tails.clear();
    }
    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
//    let fname = "test.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}