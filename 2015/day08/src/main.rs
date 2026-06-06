// Fastest run: 9.6 us

//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn process(inp:String) -> (usize, usize)
{
    let lines = inp.split("\n").collect::<Vec<&str>>();
    let mut part1 = 0;
    let mut part2 = 0;
    for line in lines {
        let lineb = line.as_bytes();
        let codes = lineb.len();
        let mut letters = 0;
        let mut i = 1;
        let lineblen = lineb.len();
        while i < lineblen-1 {
            if lineb[i] == b'\\' {
                if lineb[i+1] == b'x' {
                    i += 3;
                } else {
                    i += 1;
                }
            }
            letters += 1;
            i += 1;
        }
        part1 += codes - letters;
        let mut encoded = lineb.len();
        for i in 1..lineb.len()-1 {
            let c = lineb[i];
            if c == b'\\' || c == b'"' {
                encoded += 1;
            }
        }
        part2 += encoded - codes;
    }
    (part1, part2)
}

fn process2(inp:String) -> (usize, usize)
{
    let lines = inp.split("\n").collect::<Vec<&str>>();
    let mut part1 = 0;
    let mut part2 = 0;
    for line in lines {
        let lineb = line.as_bytes();
        let codes = lineb.len();
        let mut letters = 0;
        let mut i = 1;
        let lineblen_m1 = lineb.len() -1;
        let mut encoded = lineb.len() + 4;
        while i < lineblen_m1 {
            let c = lineb[i];
            if c == b'\\' {
                encoded += 1;
                i += 1;
                let c = lineb[i];
                if c == b'x' {
                    i += 2;
                }
                else if c == b'\\' || c == b'"' {
                    encoded += 1;
                }
            }
            else if c == b'"' {
                encoded += 1;
            }
            letters += 1;
            i += 1;
        }
        part1 += codes - letters;
        part2 += encoded - codes;
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
    let (part1, part2) = process2(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}