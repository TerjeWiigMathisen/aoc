//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn safe(l:Vec<i32>) -> i32 {
    let mut p = l[0];
    let mut c = l[1];
    if c > p {
        for i in 1..l.len() {
            c = l[i];
            let d = c-p;
            if d < 1 || d > 3 {
                return 0;
            }
            p = c;
        }
        return 1;
    }
    if c < p {
        for i in 1..l.len() {
            c = l[i];
            let d = p-c;
            if d < 1 || d > 3 {
                return 0;
            }
            p = c;
        }
        return 1;
    }
    return 0;
}

fn safe2(l:Vec<i32>) -> i32 {
    for i in 0..l.len() {
        let mut s = l.clone();
        s.remove(i);
        if safe(s) == 1 {
            return 1;
        }
    }
    return 0;
}

fn process(inp:String) -> (i32, i32)
{
    let mut part1 = 0;
    let mut part2 = 0;
    for line in inp.lines() {
        let l = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if safe(l.clone()) == 1 {
            part1 += 1;
            part2 += 1;
        }
        else {
            part2 += safe2(l);
        }
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