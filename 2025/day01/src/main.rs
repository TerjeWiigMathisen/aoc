// Fastest run 55.1 us
// Modulo ops with u32
// Surface 8.6 us

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

pub fn process(inp:&str) -> (u32, u32)
{
    let mut part1 = 0;
    let mut part2 = 0;
    let mut pos = 50;
    let mut i = 0;
    let input = inp.as_bytes();
    while i < input.len() {
        let dir = input[i]; i += 1;
        let mut len = 0;
        while input[i] >= b'0' {
            len = len * 10 + (input[i] - b'0') as u32;
            i += 1;
        }
        i += 1; // skip newline
        if dir == b'L' {
            if pos > 0 {pos = 100 - pos;}
            pos += len;
            let pdiv = pos / 100;
            pos = pos % 100;
            part2 += pdiv;
            if pos > 0 {pos = 100 - pos;}

        } else { //if dir == b'R' {
            pos += len;
            let pdiv = pos / 100;
            pos = pos % 100;
            part2 += pdiv;
        }
        if pos == 0 {
            part1 += 1;
        }
    }
    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] != '\n' as u8 {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}