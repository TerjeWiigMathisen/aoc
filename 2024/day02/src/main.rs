// Fastest run 32.5 us
// Surface 72.6 us

//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn safe2(l:&Vec<i32>) -> (i32, i32) {
    let llen = l.len();
    let mut p = l[0];
    let mut c = l[1];
    let mut ok = true;
    if c > p {
        for i in 1..llen {
            c = l[i];
            let d = c-p;
            if d < 1 || d > 3 {
                ok = false;
                break;
            }
            p = c;
        }
        if ok { return (1,1); }
    }
    if c < p {
        for i in 1..llen {
            c = l[i];
            let d = p-c;
            if d < 1 || d > 3 {
                ok = false;
                break;
            }
            p = c;
        }
        if ok { return (1,1); }
    }

    let mut l = l.clone();
    let mut saved = l.pop().unwrap();
    let llen = l.len();
    let mut index = llen;
    
    loop {
        let mut p = l[0];
        let mut c = l[1];
        let mut ok = true;
        if c > p {
            for i in 1..llen {
                c = l[i];
                let d = c-p;
                if d < 1 || d > 3 {
                    ok = false;
                    break;
                }
                p = c;
            }
            if ok { return (0,1); }
        }
        if c < p {
            for i in 1..llen {
                c = l[i];
                let d = p-c;
                if d < 1 || d > 3 {
                    ok = false;
                    break;
                }
                p = c;
            }
            if ok { return (0,1); }
        }
        if index == 0 { break; }
        index -= 1;
        let curr = l[index];
        l[index] = saved;
        saved = curr;
    }
    return (0,0);
}

fn process(inp:String) -> (i32, i32)
{
    let mut part1 = 0;
    let mut part2 = 0;
    let mut nums:Vec<i32> = Vec::new();
    let mut i = 0;
    let input = inp.as_bytes();
    while i < input.len() {
        let mut n = (input[i] - b'0') as i32; i += 1;
        while input[i] >= b'0' {
            n = n*10 + (input[i] - b'0') as i32; i += 1;
        }
        nums.push(n);
        if input[i] == b'\n' {
            let (p1,p2) = safe2(&nums);
            part1 += p1;
            part2 += p2;
            nums.clear();
        }
        i += 1;
    }
    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] != '\n' as u8 {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}