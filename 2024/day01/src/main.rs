// Fastest 
//   Acer 13.3 us
//   Surface 22.9 us

//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

pub fn process(inp:&str) -> (u32, u32)
{
    let input = inp.as_bytes();
    let mut left:Vec<u32> = Vec::with_capacity(1000);
    let mut right:Vec<u32> = Vec::with_capacity(1000);

    let inplen = input.len();
    let mut i = 13;
    let ascii_offset = b'0' as u32 * 11111;
    while i <= inplen {
        let l = input[i-13] as u32 * 10000 + input[i-12] as u32 * 1000 + input[i-11] as u32 * 100 + input[i-10] as u32 * 10 + input[i-9] as u32;
        let r = input[i-5] as u32 * 10000 + input[i-4] as u32 * 1000 + input[i-3] as u32 * 100 + input[i-2] as u32 * 10 + input[i-1] as u32;
        left.push(l - ascii_offset);
        right.push(r - ascii_offset);
        i += 14;
    }

    // for line in inp.lines() {
    //     let pair = line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    //     left.push(pair[0]);
    //     right.push(pair[1]);
    // }
    left.sort_unstable();
    right.sort_unstable();
    let mut part1 = 0;
    let mut part2 = 0;
    let arr_length = left.len();
    for i in 0..arr_length {
        let l = left[i];
        let r = right[i];
        let diff = if l < r {r-l} else {l-r};
        part1 += diff;
    }
    right.push(u32::MAX);
    left.push(u32::MAX);
    let mut j = 0;
    let mut li;
    let mut lcnt;
    let mut ri = right[0];
    let mut rcnt;
    let mut i = 0;
    loop {
        li = left[i]; i += 1;
        lcnt = 1;
        while left[i] == li { 
            i += 1;
            lcnt += 1;
        }
        while ri < li {
            j += 1;
            ri = right[j];
        }
        if ri == li {
            rcnt = 1;
            while right[j+1] == ri {
                j += 1;
                rcnt += 1;
            }
            part2 += li * lcnt * rcnt;
        }
        if i >= arr_length {break;}
    }
    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { process(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}