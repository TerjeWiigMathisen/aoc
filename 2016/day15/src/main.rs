// Fastest run 138 ns

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

struct Disk {
    size:u32,
    pos:u32,
}

#[inline(never)]
fn process(inp:&str) -> (u32, u32)
{
    let mut disks:Vec<Disk> = Vec::new();

    let mut layer = 1;
    for line in inp.lines() {
        let bytes = line.as_bytes();
        if bytes.len() < 42 { break; }
        let mut s = 0;
        let mut p = 0;
        let mut i = 12;
        loop {
            let b = bytes[i];
            i += 1;
            if b == b'.' { break; }
            if b < b'0' {
                s = p;
                p = 0;
                i += 40;
                continue;
            }
            p = p*10 + (b - b'0') as u32;
        }
        disks.push(Disk { size:s, pos:(p+layer) % s });
        layer += 1;
    }
    let mut step = 1;
    let mut t = 0;
    let mut s;
    let mut p;
    for d in 0..disks.len() {
        let disk = &disks[d];
        (s,p) = (disk.size, disk.pos + t);
        p %= s;
        let stepmod = step % s;
        while p != 0 {
            t += step;
            let pstepmod = p + stepmod;
            p = if pstepmod >= s { pstepmod - s } else { pstepmod };
        }
        step *= s;
    }
    let part1 = t;
    // Add one more disk for part2:
    p = (layer + t) % 11;
    while p != 0 {
            t += step;
            p = (p + step) % 11;
    }
    let part2 = t;
    (part1, part2)
}

fn process1k(inp:&str) -> (u32, u32)
{
    let mut p1 = 0;
    let mut p2 = 0;
    for _ in 0..1000 {
        (p1,p2) = process(inp);
    }
    (p1,p2)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut fname = "input.txt".to_string();
    if args.len() > 1 { fname = args[1].clone(); }

    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process1k(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}