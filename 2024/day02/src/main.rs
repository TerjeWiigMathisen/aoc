// Fastest run 155 us

//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn safe2(l:Vec<i32>) -> (i32, i32) {
    let llen = l.len()-1;
    let mut diffs:Vec<i32> = vec![0;llen];
    let mut prev = l[0];
    let mut inc = 0;
    let mut dec = 0;
    for i in 1..=llen {
        let curr = l[i];
        let d = curr-prev;
        diffs[i-1] = d;
        inc += (d > 0 && d <= 3) as usize;
        dec += (d < 0 && d >= -3) as usize;
        prev = curr;
    }
    if inc > dec {
        if inc == llen { return (1,1); }
        if inc+2 < llen { return (0,0); }
        for i in 0..llen {
            if diffs[i] < 1 || diffs[i] > 3 {
                if i == 0 || i+1 == llen {
                    inc += 1;
                    if inc == llen { return (0,1); }
                    continue;
                }
                let d2 = diffs[i-1] + diffs[i];
                if d2 > 0 && d2 <= 3 {
                    inc += 1;
                    if inc == llen { return (0,1); }
                    continue;
                }
                let d2 = diffs[i] + diffs[i+1];
                if d2 > 0 && d2 <= 3 {
                    inc += 1;
                    if inc == llen { return (0,1); }
                    continue;
                }
            }
        }
        return (0,(inc == llen) as i32);
    }
    if dec == llen { return (1,1); }
    if dec+2 < llen { return (0,0); }
    for i in 0..llen {
        if -diffs[i] < 1 || -diffs[i] > 3 {
            if i == 0 || i+1 == llen {
                dec += 1;
                continue;
            }
            let d2 = diffs[i-1] + diffs[i];
            if -d2 > 0 && -d2 <= 3 {
                dec += 1;
                if dec == llen { return (0,1); }
                continue;
            }
            let d2 = diffs[i] + diffs[i+1];
            if -d2 > 0 && -d2 <= 3 {
                dec += 1;
                if dec == llen { return (0,1); }
                continue;
            }
        }
    }
    return (0,(dec == llen) as i32);
}

fn process(inp:String) -> (i32, i32)
{
    let mut part1 = 0;
    let mut part2 = 0;
    for line in inp.lines() {
        let l = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let (p1,p2) = safe2(l);
        part1 += p1;
        part2 += p2;
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