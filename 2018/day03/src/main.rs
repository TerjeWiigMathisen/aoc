//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

struct Elf {
    nr:u32,
    x0:u32,
    y0:u32,
    sx:u32,
    sy:u32,
}

fn process(inp:String) -> (i64,i64)
{
//    let lines = inp.split("\n");
    let p = parser!(lines("#" enr:u32 " @ " x:u32 "," y:u32 ": " dx:u32 "x" dy:u32 =>
        Elf {nr:enr, x0:x, y0:y, sx:dx, sy:dy}));
    let mut fabric:Vec<Vec<u32>> = vec![vec![0;1000];1000];
    let elfs = p.parse(&inp).unwrap();
    let mut overlap:Vec<bool> = vec![false;elfs.len()+1]; // +1 to start with #1
    for e in elfs {
        for y in e.y0..e.y0+e.sy {
            for x in e.x0..e.x0+e.sx {
                let f = fabric[y as usize][x as usize];
                if f == 0 { // First time seen
                    fabric[y as usize][x as usize] = e.nr*2;
                }
                else {
                    fabric[y as usize][x as usize] = f | 1; // Odd collision flag
                    overlap[e.nr as usize] = true;
                    overlap[(f >> 1) as usize] = true;
                }
            }
        }
    }
    let mut p1 = 0;
    for y in 0..fabric.len() {
        for x in 0..fabric[y].len() {
            p1 += fabric[y][x] & 1; // Count the collision flags!
        }
    }
    let mut p2= 0;
    for i in 1..overlap.len() {
        if !overlap[i] {
            p2 = i as i64;
            break;
        }
    }
    return (p1.into(), p2.into());
}


fn main() {

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}