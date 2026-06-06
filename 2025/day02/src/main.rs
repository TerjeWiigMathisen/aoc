// Fastest run 25.7 us

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

fn process(inp:String) -> (u64, u64)
{
    let mut part1 = 0;
    let mut part2 = 0;

    let tens = [10,10,100,1000,10000,100000,1000000,10000000,100000000];

    for pair in inp.split(",") {
        let (f,l) = pair.split_once("-").unwrap();
        let fu = f.parse::<u64>().unwrap();
        let lu = l.parse::<u64>().unwrap();
//        println!("Input range: {}-{}",fu,lu);

        let mut seen2:FxHashMap<u64,u8> = FxHashMap::default();
        let mut seen:FxHashMap<u64,u8> = FxHashMap::default();

        for div in 2..=l.len() {
            let prelen = f.len() / div;
            let pre = &f[0..prelen];
            let mut preu:u64 = if prelen == 0 {1} else {pre.parse::<u64>().unwrap()};
            let mut multiplier = tens[prelen];
            //println!("{div} {l2} {} {}",pre,multiplier);
            loop {
                let mut invalid = preu;
                for _i in 1..div {
                    invalid = invalid*multiplier+preu;
                }
                //println!("{}-{}",preu,inv);
                if invalid > lu {break;} // Outside top end of range
                if fu <= invalid { // Inside range!
                    if div == 2 { // part1
                        seen2.entry(invalid).or_insert(1);
                    }
                    seen.entry(invalid).or_insert(1);
                    //println!("{}",inv);
                }
                preu += 1;
                if preu >= multiplier {
                    multiplier *= 10;
                }
            }
        }
        for s in seen2.into_keys() {
            part1 += s;
        }
        for s in seen.keys() {
            part2 += s;
            //println!("{}",s);
        }
    }
    (part1, part2)
}

fn main() {
    let fname:String = std::env::args().nth(1).unwrap_or("input.txt".into());

    let mut input = fs::read_to_string(fname).expect("Error reading input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10000, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}