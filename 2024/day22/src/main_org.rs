//use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn iterate(secure:i32) -> i32 
{
    let mut sec = secure;
    sec = ((sec << 6) ^ sec) & 16777215;
    sec = ((sec >> 5) ^ sec) & 16777215;
    sec = ((sec << 11) ^ sec) & 16777215;
    sec
}

fn process(inp:String) -> (i64, i64)
{
    let mut part1= 0;
    let mut sum = HashMap::<u32, u32>::new();

    for line in inp.lines() {
        let mut secret = line.parse::<i32>().unwrap();
        let mut s = (secret % 10) as i8;
        let mut prev = s;
        let mut fourdelta:u32 = (s+9) as u32;
        let mut seen = HashSet::<u32>::new();
        for i in 1..=2000 {
            secret = iterate(secret);
            s = (secret % 10) as i8;
            fourdelta = (fourdelta << 8) | ((s+9-prev)) as u32;
            prev = s;
            if i < 3 { continue; }

            let price = s;
            if seen.insert(fourdelta) {
                *sum.entry(fourdelta).or_insert(0) += price as u32;
            }
            
        }
        part1 += secret as i64;
    }
    let mut max = 0;
    let mut maxkey = 0;
    for (key, value) in sum.iter() {
        if *value > max {
            max = *value;
            maxkey = *key;
        }
    }
    let smax = maxkey as i32;
    let key = format!("{},{},{},{}", (smax >> 24)-9, ((smax >> 16) & 255)-9, ((smax >> 8) & 255)-9, (smax & 255)-9 );
    let part2 = max as i64;
    println!("Part1 = {part1}, Part2 = {part2} Key = {}", key);
    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == b'\n' {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}