// Fastest run 39.0 us

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Range {
    start:u64,
    end:u64,
}

struct Ranges {
    ranges:Vec<Range>,
}

impl Ranges {
    fn new() -> Ranges {
        Ranges { ranges:vec![] }
    }
    fn add(&mut self, r:Range) {
        self.ranges.push(r);
    }
    fn merge(&mut self) {
        self.ranges.sort();
        let mut temp = Ranges::new();
        temp.ranges.push(self.ranges[0]);
        for i in 1..self.ranges.len() {
            let out = temp.ranges.len()-1;
            if temp.ranges[out].end >= self.ranges[i].start {
                temp.ranges[out].end = u64::max(temp.ranges[out].end,self.ranges[i].end);
            } else {
                temp.ranges.push(self.ranges[i]);
            }
        }
        self.ranges = temp.ranges;
    }
    fn minimum(&self) -> u64 {
        if self.ranges[0].start > 0 { return 0;}
        return self.ranges[0].end;
    }
    fn open_ranges(&self) -> u64 {
        let mut open = 1 << 32;
        for i in 0..self.ranges.len() {
            open -= self.ranges[i].end - self.ranges[i].start;
        }
        open
    }      
}

fn process(inp:&str) -> (u64, u64)
{
    let mut filter = Ranges::new();
    for line in inp.lines() {
        //let r:Vec<u64> = line.split('-').map(|s| s.parse::<u64>().unwrap()).collect();
        //filter.add(Range { start:r[0], end:r[1]+1 });
        let mut s = 0;
        let mut e = 0;
        let bytes = line.as_bytes();
        for i in 0..bytes.len() {
            if bytes[i] == b'-' {
                s = e;
                e = 0;
            }
            else {
                e = e*10 + (bytes[i] - b'0') as u64;
            }
        }
        filter.add(Range { start:s, end:e+1 });
    }
    //println!("Raw: {}", filter.ranges.len()); // 1029
    filter.merge();
    //println!("Merged: {}", filter.ranges.len()); // 118
    
    let part1 = filter.minimum();
    let part2 = filter.open_ranges();

    (part1, part2)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut fname = "input.txt".to_string();
    if args.len() > 1 { fname = args[1].clone(); }

    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == b'\n' {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}