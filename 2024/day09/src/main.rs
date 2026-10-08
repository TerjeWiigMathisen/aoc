//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

struct FileBlock {
    file_nr: u8,
    start: usize,
    length: usize,
}

struct FreeBlock {
    start: usize,
    length: usize,
}

fn process(inp:&str) -> (usize, usize)
{
    let input = inp.as_bytes();
    let mut part1 = 0;
    let mut part2 = 0;
    let mut filenr = 0;
    let mut file_blocks:Vec<FileBlock> = Vec::new();
    let mut free_blocks:Vec<FreeBlock> = Vec::new();
    let mut diskpos = 0;
    for (filespace, freespace) in input.iter().chunks(2) {
        file_blocks.push(FileBlock{file_nr: filenr, start: diskpos, length: *filespace as usize});
        diskpos += *filespace as usize;
        filenr += 1;
        free_blocks.push(FreeBlock{start: diskpos, length: *freespace as usize});
        diskpos += *freespace as usize;
    }
    // Part1 compaction
    let mut p1disk:Vec<FileBlock> = Vec::new();

    (part1, part2)
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