//use std::collections::VecDeque;
use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

const BSIZE = 26*26;
struct Bitset {
    bits:[u64;BSIZE],
}

impl Bitset {
    fn new(size:usize) -> Bitset
    {
        let bs:BitSet = Bitset{ bits:vec![0;(size+63)>>6]};
        bs
    }

    fn add(&mut self, index:usize)
    {
        self.bits[index >> 6] |= 1 << index & 63;
    }

    fn all(&self, other:&Bitset) -> bool
    {
        for i in 0..self.len() {
            if (self.bits[i] & other.bits[i]) != self.bits[i] {return false;}
        }
    }

    fn count(&self) -> usize
    {
        let mut c = 0;
        for i in 0..self.len() {
            c += self.bits[i].count_ones();
        }
        c
    }
}

fn name2index(n2:&[u8;2]) -> usize
{
    (n2[0] - b'a') as usize * 26 + (n2[1] - b'a') as usize
}

fn process(inp:&str) -> (usize, String)
{
    let mut bitmaps = vec![Bitmap::new(26*26);]
    let input = inp.as_bytes();
    let mut i = 4;
    while i < input.len() {
        let a = name2index(input[i-4..i-2]);
        let b = 

    }
    for line in inp.lines() {
        let pair = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        left.push(pair[0]);
        right.push(pair[1]);
        if rdict.contains_key(&pair[1]) {
            let val = rdict.get_mut(&pair[1]).unwrap();
            *val += 1;
        } else {
            rdict.insert(pair[1], 1);
        }
    }
    let mut part1 = 0;
    let mut part2 = 0;
    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { process(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}