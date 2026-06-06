// Best runtime 57.0 us

//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use std::iter;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

struct Puzzle {
    ax:i64,
    ay:i64,
    bx:i64,
    by:i64,
    px:i64,
    py:i64,
}

impl Puzzle {
    fn solve(&self) -> i64 {
        let bp = (self.px*self.ay-self.py*self.ax)/(self.bx*self.ay-self.by*self.ax);
        let ap = (self.px-bp*self.bx)/self.ax;
        if ap*self.ax+bp*self.bx == self.px && ap*self.ay+bp*self.by == self.py {
            //println!("ap={ap} bp={bp}");
            return ap*3+bp;
        }
        0
    }
}       

fn process(inp:&str) -> (i64, i64)
{
    let mut part1 = 0;
    let mut part2 = 0;

    // Split the input string into blocks separated by a blank line.

    let puzzles:Vec<&str> = inp.split("\n\n").collect();
    // Split each block into lines
    for puzzle in puzzles {
        let lines:Vec<&str> = puzzle.split("\n").collect();
        let comma:usize = lines[0].find(",").unwrap();
        let ax = lines[0][12..comma].parse().unwrap();
        let ay = lines[0][comma+4..].parse().unwrap();
        let comma:usize = lines[1].find(",").unwrap();
        let bx = lines[1][12..comma].parse().unwrap();
        let by = lines[1][comma+4..].parse().unwrap();
        let comma:usize = lines[2].find(",").unwrap();
        let px = lines[2][9..comma].parse().unwrap();
        let py = lines[2][comma+4..].parse().unwrap();

        let p1 = Puzzle{ax:ax, ay:ay, bx:bx, by:by, px:px, py:py};
        //println!("ax={ax} ay={ay} bx={bx} by={by} px={px} py={py}");
        part1 += p1.solve();
        let p2 = Puzzle{ax:ax, ay:ay, bx:bx, by:by, px:px+10000000000000, py:py+10000000000000};
        part2 += p2.solve();
    }

    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();
    let inp = input.as_str();

    let bench_result = run_benchmark(1000, |_| { process(inp); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(inp);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}