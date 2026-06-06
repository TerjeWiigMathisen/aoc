// Fastest run 220.7 us

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

fn process(inp:String) -> (i64, i64)
{
    let lines = inp.lines().collect::<Vec<&str>>();
    let rows = lines.len();
    let cols = lines[0].len();
    let lr = rows-1;
    let ops = lines[lr].as_bytes();
    let mut operands:Vec<usize> = Vec::new();
    let mut oper:Vec<u8> = Vec::new();
    for x in 0..ops.len() {
        let op = ops[x];
        if op == b'+' {
            operands.push(x);
            oper.push(op);
        }
        else if op == b'*' {
            operands.push(x);
            oper.push(op);
        }
    }
    operands.push(ops.len()+1);

    let mut part1 = 0;
    let mut part2 = 0;

    // part1
    for o in 0..operands.len()-1 {
        let left = operands[o];
        let right = operands[o+1]-1;

        // Current operation from left to right-1 inclusive

        let mut nums:Vec<i64> = Vec::new();
        for y in 0..lr {
            nums.push(lines[y][left..right].trim().parse::<i64>().unwrap());
        }
        if oper[o] == b'*' {
            let mut acc:i64 = 1;
            for x in 0..nums.len() {
                acc *= nums[x];
            }
            //println!("{acc} <- {} {:?}", oper[o],nums);
            part1 += acc;
        }
        else {
            let mut acc = 0;
            for x in 0..nums.len() {
                acc += nums[x];
            }
            //println!("{acc} <- {} {:?}", oper[o],nums);
            part1 += acc;
        }
    }
    // part2
    let opline = lines[lr];
    for col in 0..cols {
        let op = opline.as_bytes()[col];
        if op != b' ' {
            let mut nums:Vec<i64> = Vec::new();
            for x in col..cols {
                let mut v = String::new();
                for y in 0..lr {
                    v.push(lines[y].as_bytes()[x] as char);
                }
                let n = v.trim().parse::<i64>();
                if n.is_err() {
                    break;
                }
                nums.push(n.unwrap());
            }
            if op == b'*' {
                let mut acc:i64 = 1;
                for x in 0..nums.len() {
                    acc *= nums[x];
                }
                //println!("{acc} <- {} {:?}", oper[o],nums);
                part2 += acc;
            }
            else {
                let mut acc = 0;
                for x in 0..nums.len() {
                    acc += nums[x];
                }
                //println!("{acc} <- {} {:?}", oper[o],nums);
                part2 += acc;
            }
        }
    }
    (part1, part2)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut fname = "input.txt".to_string();
    if args.len() > 1 { fname = args[1].clone(); }

    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}