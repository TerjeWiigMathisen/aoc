// Best runtime 39.5 us (Vec of puzzles)
// Rewrite as a struct of vectors, making vectorization easier.
// New runtime 6205 ns

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
    ax:Vec<u32>,
    ay:Vec<u32>,
    bx:Vec<u32>,
    by:Vec<u32>,
    px:Vec<u32>,
    py:Vec<u32>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { ax:Vec::with_capacity(400), ay:Vec::with_capacity(400), bx:Vec::with_capacity(400), by:Vec::with_capacity(400), px:Vec::with_capacity(400), py:Vec::with_capacity(400) }
    }
    fn solve2(&self) -> (i64, i64) {
        let mut p1 = 0;
        let mut p2 = 0;
        for i in 0..self.px.len() {
            let mut px = self.px[i] as i64;
            let mut py = self.py[i] as i64;
            let ax = self.ax[i] as i64;
            let ay = self.ay[i] as i64;
            let bx = self.bx[i] as i64;
            let by = self.by[i] as i64;
//            println!("ax={:?} ay={:?} bx={:?} by={:?} px={:?} py={:?}", self.ax[i], self.ay[i], 
//                self.bx[i], self.by[i], px, py);
            let bp = (px*ay-py*ax)/(bx*ay-by*ax);
            let ap = (px-bp*bx)/ax;
            if ap*ax+bp*bx == px && ap*ay+bp*by == py {
//                println!("ap={ap} bp={bp}");
                p1 += ap*3+bp;
            }
            px += 10000000000000;
            py += 10000000000000;
            let bp = (px*ay-py*ax)/(bx*ay-by*ax);
            let ap = (px-bp*bx)/ax;
            if ap*ax+bp*bx == px && ap*ay+bp*by == py {
//                println!("ap={ap} bp={bp}");
                p2 += ap*3+bp;
            }
        }
        (p1, p2)
    }
}       

pub fn process(inp:&str) -> (i64, i64)
{
    // Split the input string into blocks separated by a blank line.
    let mut puzzles = Puzzle::new();
    let input = inp.as_bytes();
    let mut i = 0;
    while i+50 < input.len() {
        // Parse each puzzle block
        i += 12; // Skip 'Button A: X+'
//        assert!(input[i-2] == b'X');
        let mut ax = 0;
        while input[i] != b',' {
            ax = ax*10 + (input[i]-b'0') as u32;
            i += 1;
        }

        i += 4; // skip ", Y+"
//        assert!(input[i-2] == b'Y');
        let mut ay = 0;
        while input[i] != b'\n' {
            ay = ay*10 + (input[i]-b'0') as u32;
            i += 1;
        }

        i += 13; // B line
//        assert!(input[i-2] == b'X');
        let mut bx = 0;
        while input[i] != b',' {
            bx = bx*10 + (input[i]-b'0') as u32;
            i += 1;
        }

        i += 4; // skip ", Y+"
//        assert!(input[i-2] == b'Y');
        // sign = 1;
        // if input[i] == b'-' {sign = -1;} // else '+'
        // i += 1;
        let mut by = 0;
        while input[i] != b'\n' {
            by = by*10 + (input[i]-b'0') as u32;
            i += 1;
        }

        i += 10; // skip '\nPrize: X='
//        assert!(input[i-2] == b'X');
        let mut px = 0;
        while input[i] != b',' {
            px = px*10 + (input[i]-b'0') as u32;
            i += 1;
        }
        i += 4; // skip ", Y="
//        assert!(input[i-2] == b'Y');
        let mut py = 0;
        while input[i] != b'\n' {
            py = py*10 + (input[i]-b'0') as u32;
            i += 1;
        }
        //        assert!(input[i] == b'\n');
        i += 2; // skip '\n\n'

        puzzles.ax.push(ax);
        puzzles.ay.push(ay);
        puzzles.bx.push(bx);
        puzzles.by.push(by);
        puzzles.px.push(px);
        puzzles.py.push(py);
    }
    let (part1, part2) = puzzles.solve2();

    (part1, part2)
}

fn p1k(inp:&str) -> (i64, i64)
{
    for _ in 0..1000 {
        process(inp);
    }
    (0,0)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] != '\n' as u8 {input.push('\n');}

    let mut devtime = DevTime::new_simple();
    let inp = input.as_str();

    let bench_result = run_benchmark(1000, |_| { p1k(inp); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(inp);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}