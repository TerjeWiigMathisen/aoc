// fastest run 144 us
use rustc_hash::FxHashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

fn _process(test:String) -> (i32,i32)
{
    let mut regs:FxHashMap<&str,i32> = FxHashMap::default();
    let mut part2 = 0;
    for line in test.lines() {
        let tokens:Vec<&str> = line.split_whitespace().collect();
        let dest = tokens[0];
        //assert!(tokens[3] == "if");
        let srct = tokens[4];
        let op = tokens[5];
        let limit = tokens[6].parse::<i32>().unwrap();

        let src:i32;
        if regs.contains_key(srct) {
            src = *regs.get(srct).unwrap();
        } else {
            regs.insert(srct,0);
            src = 0;
        }
        let mut flag = false;
        match op {
            "==" => flag = src == limit,
            "!=" => flag = src != limit,
            ">" => flag = src > limit,
            "<" => flag = src < limit,
            ">=" => flag = src >= limit,
            "<=" => flag = src <= limit,
            _ => assert!(false),
        }
        if flag {
            let increment = tokens[2].parse::<i32>().unwrap();
            let dreg = regs.entry(dest).or_insert(0);
            *dreg += if tokens[1] == "inc" {increment} else {-increment};
            if *dreg > part2 {
                part2 = *dreg;
            }
        }   
    }
    let mut part1 = i32::min_value();
    for val in regs.values() {
        if *val > part1 {
            part1 = *val;
        }
    }
    return (part1, part2);
}

fn flatkey(reg:&str)->usize
{
    let bytes = reg.as_bytes();
    let mut r = (bytes[0] & 31) as usize;
    for i in 1..bytes.len() { r = r*27 + (bytes[i] & 31) as usize; }
    r
}

fn processflat(test:String) -> (i32,i32)
{
    let mut regs:[i16;27*27*27] = [0;27*27*27];
    let mut part2 = 0;
    for line in test.lines() {
        let tokens:Vec<&str> = line.split_whitespace().collect();
        let dest = tokens[0];
        //assert!(tokens[3] == "if");
        let srct = tokens[4];
        let op = tokens[5];
        let limit = tokens[6].parse::<i16>().unwrap();

        let src = regs[flatkey(srct)];
        let mut flag = false;
        match op {
            "==" => flag = src == limit,
            "!=" => flag = src != limit,
            ">" => flag = src > limit,
            "<" => flag = src < limit,
            ">=" => flag = src >= limit,
            "<=" => flag = src <= limit,
            _ => assert!(false),
        }
        if flag {
            let increment = tokens[2].parse::<i16>().unwrap();
            let dreg = flatkey(dest);
            regs[dreg] += if tokens[1] == "inc" {increment} else {-increment};
            if regs[dreg] > part2 {
                part2 = regs[dreg];
            }
        }   
    }
    let part1 = *regs.iter().max().unwrap();
    (part1 as i32, part2 as i32)
}

fn main() {
    let fname = "input.txt";
    let input = fs::read_to_string(fname).expect("Error readin input file");

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { processflat(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1,part2) = processflat(input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}