// Fastest run 34200 ns on Surface 7 Pro Intel(R) Core(TM) i5-1035G4 CPU @ 1.10GHz

//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;
//use rustc_hash::FxHashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
struct Range {
    fra: u64,
    til: u64,
}

fn process(inp:&str) -> (u64, u64)
{
    let mut part1 = 0;
    let mut part2 = 0;

    let mut range:Vec<Range> = Vec::with_capacity(200);
    let mut ingredients:Vec<u64> = Vec::with_capacity(1000);

    let mut a = 0;
    let mut b = 0;
    let mut i = 0;
    let bytes = inp.as_bytes();
    loop {
        let by = bytes[i];
        i += 1;
        if by == b'\n' {
            if b == 0 { break; }

            range.push(Range{fra:a, til:b+1});
            b = 0;
        }
        else if by == b'-' {
            a = b;
            b = 0;
        }
        else {
            b = b*10 + (by - b'0') as u64;
        }
    }
    while i < bytes.len() {
        let by = bytes[i];
        i += 1;
        if by == b'\n' {
            ingredients.push(b);
            b = 0;
        }
        else {
            b = b*10 + (by - b'0') as u64;
        }
    }
    // The last line always ends with a newline, so we don't need to check for b > 0 here.
    // if b > 0 {
    //     ingredients.push(b);
    // }
    range.sort_unstable(); // Sort by fra,til
    ingredients.sort_unstable();

    //println!("Collected {} ranges and {} ingredients", range.len(), ingredients.len());
    // parsing done, now pack the ranges
    range.push(Range{fra:u64::MAX, til:u64::MAX});
    let mut r = range[0];
    let mut out = 0;
    for i in 1..range.len() {
        if r.til < range[i].fra {
            range[out] = r;
            part2 += r.til-r.fra;
            out += 1;
            r = range[i];
        }
        else if r.til < range[i].til {
            r.til = range[i].til;
        }
    }
    range[out] = r;
//    part2 += r.til-r.fra; // zero length guard can be skipped.
    out += 1;
    range.truncate(out);
    //println!("Compacted into {} ranges", range.len());

    let mut r = 0;
    for ing in ingredients {
        while ing >= range[r].til {
            r += 1;
        }
        if ing >= range[r].fra {
            part1 += 1;
        }
    }
    (part1, part2)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut fname = "input.txt".to_string();
    if args.len() > 1 { fname = args[1].clone(); }

    let mut input = fs::read_to_string(fname).expect("Error reading input file");
    if input.as_bytes()[input.as_bytes().len()-1] != b'\n' {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}