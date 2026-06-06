// Fastest run 165 us

//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;
use rustc_hash::FxHashMap;

fn process(inp:String) -> (i32, i32)
{
    let mut left:Vec<i32> = Vec::new();
    let mut right:Vec<i32> = Vec::new();
    let mut rdict:FxHashMap<i32, i32> = FxHashMap::default();

    for line in inp.lines() {
        let pair = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        left.push(pair[0]);
        right.push(pair[1]);
        if rdict.contains_key(&pair[1]) {
            let val = rdict.get_mut(&pair[1]).unwrap();
            *val += 1;
        } else {
            rdict.insert(pair[1], 1);
        }
    }
    left.sort();
    right.sort();
    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0..left.len() {
        let l = left[i];
        let r = right[i];
        let diff = (l - r).abs();
        part1 += diff;
        if rdict.contains_key(&l) {
            part2 += l * rdict.get(&l).unwrap();
        }
    }
    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}