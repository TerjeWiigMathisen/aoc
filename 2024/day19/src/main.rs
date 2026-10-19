// Acer 5976 us
// Acer 1958 us with tail end counting

//use std::collections::VecDeque;
//use std::collections::HashMap;
use std::collections::HashSet;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

struct Patterns {
    patterns:Vec<&str>,
    pstart:[u16;32],
    maxpathlen:usize,
}

fn ways(need:&str, pat:&Patterns) -> usize
{
    let needb = need.as_bytes();
    let nlen = needb.len();
    let mut tails:Vec<usize> = vec![0;nlen+1];
    tails[0] = 1;
	for i in 1..=nlen {
    	let mut w = 0;
        let slice_start = nlen-i;
        let mut idx = pat.pstart[(needb[slice_start] & 31) as usize];
        for slice_end in slice_start+1..=nlen {
            if slice_end - slice_start > pat.maxpathlen {break;}
            let t = &need[slice_start..slice_end];
            if pat.patterns.contains(t) {
                w += tails[nlen-slice_end];
            }
        }
    	tails[i] = w;
	}
	tails[nlen]
}

fn process(inp:&str) -> (usize, usize)
{
    let mut pat = Patterns{patterns:HashSet::new(), maxpathlen:0}; //, tails:Vec::with_capacity(64)};
    let (patterns, targets) = inp.split_once("\n\n").unwrap();
    let pats = patterns.split(", ");
    for p in pats {
        if p.len() > pat.maxpathlen { pat.maxpathlen = p.len();} 
        pat.patterns.insert(p); 
    }
    let mut part1 = 0;
    let mut part2 = 0;
    for target in targets.lines() {
        let w = ways(target, &pat);
        part1 += (w > 0) as usize;
        part2 += w;
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