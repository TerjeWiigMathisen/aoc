// Fastest run: 67.7 us
//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
//use std::str;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn process(inp:String) -> (i64, i64)
{
    let lines = inp.split("\n");
    let mut part1 = 0;
    let mut part2 = 0;
    for line in lines
    {
        let mut parts:Vec<i64> = line.split("x").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let surface = 2 * (parts[0]*(parts[1]+parts[2]) + parts[1]*parts[2]);
        // let mut sortedparts = parts.clone();
        //parts.sort();
        if parts[0] > parts[2] {parts.swap(0,2);}
        if parts[1] > parts[2] {parts.swap(1,2);}
        let extra = parts[0]*parts[1];
        part1 += surface + extra;
        part2 += 2*parts[0] + 2*parts[1] + parts[0]*parts[1]*parts[2];
    }
    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}