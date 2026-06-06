//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
//use std::fs;
use md5;
use std::thread;
//use hex_literal::hex;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn md5block(inp:&str, first:usize, last:usize) -> (usize, usize) 
{
    let mut part1 = 0;
    let mut part2 = 0;
    for i in first..last {
        let s = format!("{}{}", inp, i);
        let digest = md5::compute(s.as_bytes());
        if digest[0] == 0 && digest[1] == 0 && digest[2] < 16
        {   
            if part1 == 0 {
                part1 = i;
            }
            if digest[2] == 0 {
                part2 = i;
                break;
            }
        }
    }
    (part1, part2)
}

fn process(inp:String) -> (usize, usize)
{
    let mut part1 = 0;
    let mut part2 = 0;
    let maxthreads = 8;
    let mut threads = Vec::new();
    let mut i:usize = 0;
    loop {
        let inner:usize = i.clone();
        let handle = thread::spawn( || {
            let begin = inner.clone();
            let end = begin+1000;
            let (p1,p2) = md5block(&inp, begin, end);
            (p1,p2)
        });
        i += 1000;
        threads.push(handle);
        if threads.len() >= maxthreads {
            for handle in threads.drain(..) {
                let (p1,p2) = handle.join().unwrap();
                if p1 != 0 {
                    part1 = p1;
                }
                if p2 != 0 {
                    part2 = p2;
                    break;
                }
            }
        }
    }
    (part1, part2)
}

fn main() {
    //let fname = "input.txt"; // instead of args[1]
    let input:String = "iwrupvqb".to_string(); // fs::read_to_string(fname).expect("Error readin input file");
    //if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}