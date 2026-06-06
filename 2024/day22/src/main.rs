//use std::collections::VecDeque;
// use std::collections::{HashMap, HashSet};
//use std::io;
use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;
//use std::cell::UnsafeCell;
//use std::marker::Sync;
//use std::sync::atomic::{AtomicU16, AtomicI64, Ordering};
//use std::mem::transmute;
use std::thread;

fn iterate(secure:u32) -> u32 
{
    let mut sec = secure;
    sec = ((sec << 6) ^ sec) & 16777215;
    sec = (sec >> 5) ^ sec; //) & 16777215;
    sec = ((sec << 11) ^ sec) & 16777215;
    sec
}

//const INIT_ATOMIC_U16: AtomicU16 = AtomicU16::new(0);
//static mut SUM: [AtomicU16; 19*19*19*19] = [INIT_ATOMIC_U16; 19*19*19*19];
//static mut PART1: AtomicI64 = AtomicI64::new(0);

fn inner(nums:Vec<u32>) -> (i64,Vec<u16>)
{
    let mut part1= 0;
    let mut sum:Vec<u16> = vec![0;19*19*19*19];

    for i in 0..nums.len() {
        let mut secret = nums[i];
        let mut sp = secret % 10;

        secret = iterate(secret);
        let ap = secret % 10;
        let mut d3 = (ap+9-sp)*19*19*19;

        secret = iterate(secret);
        let bp = secret % 10;
        let mut d2 = (bp+9-ap)*19*19;

        secret = iterate(secret);
        let mut cp = secret % 10;
        let mut d1 = (cp+9-bp)*19;

        let mut seen = [false;19*19*19*19];
        for _ in 4..=2000 {
            secret = iterate(secret);
            sp = secret % 10;
            let d0 = sp+9-cp;
            let index = (d3 + d2 + d1 + d0) as usize;
            if !seen[index] {
                seen[index] = true;
                sum[index] += sp as u16;
            }
            (d1,d2,d3) = (d0*19,d1*19,d2*19);     
            cp = sp;       
        }
        part1 += secret as i64;
        //println!("Secret 2000 = {}", secret);
    }
    (part1, sum)
}

fn process(inp:String) -> (i64,i64)
{
    let mut part1= 0;
    let mut sum:Vec<u16> = vec![0;19*19*19*19];
    let mut nums:Vec<u32> = Vec::new();

    for line in inp.lines() {
        let secret = line.parse::<u32>().unwrap();
        nums.push(secret);
    }

    let mut handles = vec![];
    //let mut results = vec![];
    let num_threads = 8;
    let chunk_size = (nums.len()+num_threads-1) / num_threads;
    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = if i == num_threads-1 { nums.len() } else { (i+1) * chunk_size };
        let mut numbers = Vec::<u32>::new();
        for j in start..end {
            numbers.push(nums[j]);
        }
        let handle = thread::spawn(move || inner(numbers));
        handles.push(handle);
    }
    for handle in handles {
        let result = handle.join().unwrap();
        part1 += result.0;
        for i in 0..(19*19*19*19) {
            sum[i] += result.1[i];
        }
    }
    let mut part2 = 0;
    for i in 0..(19*19*19*19) {
        let val = sum[i];
        if val > part2 { part2 = val; }
    }
    (part1, part2 as i64)
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut fname = "input.txt".to_string();

    if args.len() > 1 { fname = args[1].clone(); }
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == b'\n' {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}