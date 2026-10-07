// Acer 162.8 us
// Surface 267 us

use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn extract_u64(bytes:&[u8], res:&mut Vec<u64>)
{
    res.clear();
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
}

fn try_part1(target:u64, nums:&Vec<u64>, pos:usize) -> bool
{
    if pos == 0 {return false;}
    let pos = pos - 1;
    let n = nums[pos];
    if pos == 0 { return n == target; }
    if target % n == 0 {
        let rem = target / n;
        if try_part1(rem, nums, pos) {return true;}
    }
    if target <= n { return false; }
    let diff = target - n;
    try_part1(diff, nums, pos)
}

fn try_part2(target:u64, nums:&Vec<u64>, pos:usize) -> bool
{
    if pos == 0 {return false;}
    let pos = pos - 1;
    let n = nums[pos];
    if pos == 0 { return n == target; }

    if n < 10 && target % 10 == n {
        if try_part2(target / 10, nums, pos) { return true;}
    }
    else if n < 100 && target % 100 == n {
        if try_part2(target / 100, nums, pos) { return true;}
    }
    else if target % 1000 == n {
        if try_part2(target / 1000, nums, pos) { return true;}
    }

    if target % n == 0 {
        if try_part2(target / n, nums, pos) {return true;}
    }

    let diff = target - n;
    try_part2(diff, nums, pos)
}

fn process(inp:&str) -> (u64, u64)
{
    let mut part1 = 0;
    let mut part2 = 0;
    let lines = inp.split("\n");
    let mut nums = Vec::new();
    for line in lines {
        extract_u64(line.as_bytes(), &mut nums);
        let target = nums[0];
        let nums = nums[1..].to_vec();
        let pos = nums.len();
        if try_part1(target, &nums, pos) {
            part1 += target;
            part2 += target;
        }
        else {
//            println!("Testing {}", line);
//            let org = try_part13(target, &nums, pos);
            let ny = try_part2(target, &nums, pos);
            // if org != ny {
            //     println!("{}: {} != {}", line, org, ny);
            //     try_part13(target, &nums, pos);
            //     try_part2(target, &nums, pos);
            // }
            if ny {
//                println!("OK, adding {}", target);
                part2 += target;
            }
        }
    }
    (part1, part2)
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let fname: &str = if args.len() < 2 { "input.txt" } else { args[1].as_str() };
    let mut input = fs::read_to_string(fname).expect("Error reading input file");
//    if input.as_bytes()[input.as_bytes().len()-1] != b'\n' {input.push('\n');}
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}