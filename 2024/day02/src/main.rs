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
    let llen = l.len();
    let mut a = l[0];
    let mut b = l[1];
    let mut c = l[2];
    let mut d = l[3];
    let mut ok = 0;
    let mut increase = (a < b ) as i8 + (b < c) as i8 + (c < d) as i8;
    if increase >= 2 {
        let mut skip = 0;
        for i in 1.llen {
            let diff = l[i]-l[i-1];
            if diff < 1 || diff > 3 {
                if i != 1 && i != llen-1 {
                    // Try to skip l[i]
                    diff = 
                }
            }
        }
        increase -= (b-a > 3) as i8 + (c-b > 3) as i8 + (d-c > 3) as i8;
        if increase < 2 {return (0,0);}
        for i in 4..llen {
            increase -= (l[i] <= d || (l[i]-d > 3)) as i8;
        }
        return ((increase == 3) as i32, (increase >= 2) as i32);
    }
    let mut decrease = (a > b ) as i8 + (b > c) as i8 + (c > d) as i8;
    if decrease >= 2 {
        decrease -= (a-b > 3) as i8 + (b-c > 3) as i8 + (c-d > 3) as i8;
        if decrease < 2 {return (0,0);}
        for i in 4..llen {
            decrease -= (l[i] >= d || (d-l[i] > 3)) as i8;
        }
        return ((increase == 3) as i32, (increase >= 2) as i32);
    }
    return (0,0);
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