//Fastest 189 us
//use std::io;
//use std::env;
use std::fs;
use std::collections::HashSet;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

fn letter_histogram(word:&str) -> [u8;32]
{
    let mut hist: [u8;32] = [0;32];
    for c in word.bytes() {
        hist[(c & 31) as usize] += 1;
    }
    return hist;
}

fn process(inp:String) -> (i32,i32)
{
    let mut pp:u32 = 0;

    let lines = inp.split("\n");
    let mut seen :HashSet<&str> = HashSet::new();
    let mut ana :HashSet<[u8;32]> = HashSet::new();
    for li in lines {
        if li.len() == 0 {break; }
        let words = li.split_ascii_whitespace();
        seen.clear();
        ana.clear();
        let mut ok:u32 = 1 + (1 << 16);
        for w in words {
            if seen.contains(w) {
                ok = 0;
                break;
            }
            seen.insert(w);
            
            if ok > 1 {
                let hist = letter_histogram(w);
                if ana.contains(&hist) {
                    ok = 1;
                }
                else {
                    ana.insert(hist);
                }
            }
        }
        pp += ok;
    }
    return ((pp & 0xffff) as i32,(pp >> 16) as i32);
}

fn main() {

    let fname = "input.txt"; // instead of args[1]
    let input = fs::read_to_string(fname).expect("Error readin input file");
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| {
        process(input.clone());
    });
    bench_result.print_stats();

    devtime.start();
    let (part1,part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}