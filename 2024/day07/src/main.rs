//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn extract_u64(bytes:&[u8]) -> Vec<u64>
{
    let mut res:Vec<u64> = Vec::new();
    let mut n = 0;
    let mut valid = false;
    for c in bytes {
        if c.is_ascii_digit() {
            n = n * 10 + (c - b'0') as u64;
            valid = true;
        }
        else if valid {
            res.push(n);
            n = 0;
            valid = false;
        }
    }
    if valid {
        res.push(n);
    }
    res
}

fn process(inp:String) -> (u64, u64)
{
    let mut part1 = 0;
    let mut part2 = 0;
    let lines = inp.split("\n");
    for line in lines {
        let nums = extract_u64(line.as_bytes());
        let target = nums[0];
        let nums = nums[1..].to_vec();
        let pos = nums.len();
        if try_opsr(target, &nums, pos) {
            part1 += target;
            part2 += target;
        }
        else if try_opsr3(target, &nums, pos) {
            part2 += target;
        }
    }
    (part1, part2)
}

fn try_opsr(target:u64, nums:&Vec<u64>, pos:usize) -> bool
{
    if pos == 0 {return false;}
    let pos = pos - 1;
    let n = nums[pos];
    if pos == 0 { return n == target; }
    if target % n == 0 {
        let rem = target / n;
        if try_opsr(rem, nums, pos) {return true;}
    }
    if target <= n { return false; }
    let diff = target - n;
    try_opsr(diff, nums, pos)
}

fn try_opsr3(target:u64, nums:&Vec<u64>, pos:usize) -> bool
{
    if pos == 0 {return false;}
    let pos = pos - 1;
    let n = nums[pos];
    if pos == 0 { return n == target; }
    if target % n == 0 {
        let rem = target / n;
        if try_opsr3(rem, nums, pos) {return true;}
    }
    if n % 10 == target % 10 {
        let targetstr = target.to_string();
        let nstr = n.to_string();
        if targetstr.len() > nstr.len() {
            if targetstr[targetstr.len()-nstr.len()..] == nstr {
                let front = targetstr[..targetstr.len()-nstr.len()].parse::<u64>().unwrap();
                if try_opsr3(front, nums, pos) {return true;}
            }
        }
    }

    if target <= n { return false; }
    let diff = target - n;
    try_opsr3(diff, nums, pos)
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