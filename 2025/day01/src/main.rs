// Fastest run 55.1 us

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

pub fn process(inp:String) -> (i32, i32)
{
    let mut part1 = 0;
    let mut part2 = 0;
    let mut pos = 50;
    for line in inp.lines() {
        let dir = line.as_bytes()[0];
        let len = line[1..].parse::<i32>().unwrap();
        if dir == b'L' {
            if pos == 0 {
                part2 -= 1;
            }
            pos -= len;
            loop {
                if pos < 0 {
                    pos += 100;
                    part2 += 1;
                }
                if pos >= 0 { break; }
            }
            if pos == 0 {
                part1 += 1;
                part2 += 1;
            }
        } else if dir == b'R' {
            pos += len;
            while pos >= 100 {
                pos -= 100;
                part2 += 1;
            }
            if pos == 0 {
                part1 += 1;
            }
        }
    }
    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}