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
    ax:Vec<i64>,
    ay:Vec<i64>,
    bx:Vec<i64>,
    by:Vec<i64>,
    px:Vec<i64>,
    py:Vec<i64>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { ax:Vec::with_capacity(400), ay:Vec::with_capacity(400), bx:Vec::with_capacity(400), by:Vec::with_capacity(400), px:Vec::with_capacity(400), py:Vec::with_capacity(400) }
    }
    fn solve(&self) -> (Vec<i64>, Vec<i64>) {
        let mut results = vec![0; self.px.len()];
        let mut results2 = vec![0; self.px.len()];
        for i in 0..self.px.len() {
            let mut px = self.px[i];
            let mut py = self.py[i];
//            println!("ax={:?} ay={:?} bx={:?} by={:?} px={:?} py={:?}", self.ax[i], self.ay[i], 
//                self.bx[i], self.by[i], px, py);
            let bp = (px*self.ay[i]-py*self.ax[i])/(self.bx[i]*self.ay[i]-self.by[i]*self.ax[i]);
            let ap = (px-bp*self.bx[i])/self.ax[i];
            if ap*self.ax[i]+bp*self.bx[i] == px && ap*self.ay[i]+bp*self.by[i] == py {
//                println!("ap={ap} bp={bp}");
                results[i] = ap*3+bp;
            }
            px += 10000000000000;
            py += 10000000000000;
            let bp = (px*self.ay[i]-py*self.ax[i])/(self.bx[i]*self.ay[i]-self.by[i]*self.ax[i]);
            let ap = (px-bp*self.bx[i])/self.ax[i];
            if ap*self.ax[i]+bp*self.bx[i] == px && ap*self.ay[i]+bp*self.by[i] == py {
//                println!("ap={ap} bp={bp}");
                results2[i] = ap*3+bp;
            }
        }
        (results, results2)
    }
}       

fn process(inp:&str) -> (i64, i64)
{
    let mut part1 = 0;
    let mut part2 = 0;

    // Split the input string into blocks separated by a blank line.

    let mut puzzles = Puzzle::new();
    let input = inp.as_bytes();
    let mut i = 0;
    while i+10 < input.len() {
        // Parse each puzzle block
        i += 11; // Skip 'Button A: X'
        assert!(input[i-1] == b'X');
        let mut sign = 1;
        if input[i] == b'-' {sign = -1;} // else '+'
        i += 1;
        let mut ax = 0;
        while input[i] != b',' {
            ax = ax*10 + (input[i]-b'0') as i64;
            i += 1;
        }
        ax *= sign;

        i += 3; // skip ", Y"
        assert!(input[i-1] == b'Y');
        sign = 1;
        if input[i] == b'-' {sign = -1;} // else '+'
        i += 1;
        let mut ay = 0;
        while input[i] != b'\n' {
            ay = ay*10 + (input[i]-b'0') as i64;
            i += 1;
        }
        ay *= sign;

        i += 12; // B line
        assert!(input[i-1] == b'X');
        sign = 1;
        if input[i] == b'-' {sign = -1;} // else '+'
        i += 1;
        let mut bx = 0;
        while input[i] != b',' {
            bx = bx*10 + (input[i]-b'0') as i64;
            i += 1;
        }
        bx *= sign;

        i += 3; // skip ", Y"
        assert!(input[i-1] == b'Y');
        sign = 1;
        if input[i] == b'-' {sign = -1;} // else '+'
        i += 1;
        let mut by = 0;
        while input[i] != b'\n' {
            by = by*10 + (input[i]-b'0') as i64;
            i += 1;
        }
        by *= sign;

        i += 10; // skip '\nPrize: X='
        assert!(input[i-2] == b'X');
        let mut px = 0;
        while input[i] != b',' {
            px = px*10 + (input[i]-b'0') as i64;
            i += 1;
        }
        i += 4; // skip ", Y="
        assert!(input[i-2] == b'Y');
        let mut py = 0;
        while input[i] != b'\n' {
            py = py*10 + (input[i]-b'0') as i64;
            i += 1;
        }
        assert!(input[i] == b'\n');
        i += 2; // skip '\n\n'

        puzzles.ax.push(ax);
        puzzles.ay.push(ay);
        puzzles.bx.push(bx);
        puzzles.by.push(by);
        puzzles.px.push(px);
        puzzles.py.push(py);
    }
    let (p1, p2) = puzzles.solve();
    for i in 0..p1.len() {
        part1 += p1[i];
        part2 += p2[i];
    }

    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] != '\n' as u8 {input.push('\n');}

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