//use std::io;
//use std::env;
//use std::fs;
//use std::collections::HashMap;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;
use skiplist::SkipList;

fn process(steps:usize, iterations:usize) -> Vec<u32>
{
    let mut d:Vec<u32> = vec![0];
    let mut p1 = 0;
    for i in 0..iterations {
        let mut temp = d.clone();
        let s = steps % d.len();
        let mut j = 0;
        for i in s..d.len() {
            d[j] = temp[i];
            j += 1;
        }
        for i in 0..s {
            d[j] = temp[i];
            j += 1;
        }
        d.push(i as u32 +1);
        if i == 2016 {
            p1 = d[0];
        }
        if i & 0xffff == 0 {println!("{i} p1={p1}");}
    }
    return d;
}

fn process1(steps:usize) -> u32
{
    let d:Vec<u32> = process(steps, 2017);
    return d[0];
}

// TODO: Implement skip list by counting number of unused cells in each 256 number block
fn process2(steps:usize) -> u32
{
    let mut d:Vec<u32> = vec![0;50000001];
    let mut i = 50000000;
    let mut pos = d.len()-1;
    d[pos] = i;
    let mut disp = i - 1000000;
    while i > 0 {
        let mut s = steps+1;
//        if s > i as usize {s = s % i as usize;}
        if s > i as usize { s %= i as usize;}
        let mut j = s;
        while j > 0 {
            loop {
                pos = if pos > 0 {pos-1} else {d.len()-1};
                if d[pos] < i {break;}
            }
            j -= 1;
        }
        i -= 1;
        d[pos] = i;
        if i == disp {
            println!("{disp}");
            disp -= 1000000;
        }
    }
    return d[pos+1];
}

fn process3(steps:usize) -> u32
{
    let mut d:Vec<u32> = vec![0;50000001];
    let mut i = 50000000;
    let mut pos = d.len()-1;
    d[pos] = i;
    let mut skip32:Vec<u8> = vec![32;50000001/32];
    skip32.push(0);

    let mut disp = i - 1000000;
    while i > 0 {
        let mut s = steps+1;
//        if s > i as usize {s = s % i as usize;}
        if s > i as usize { s %= i as usize;}
        let mut j = s;
        'outer: while j > 0 {
            loop {
                pos = if pos > 0 {pos-1} else {d.len()-1};
                if pos & 31 == 31 { break;}
                if d[pos] < i {
                    j -= 1;
                    if j == 0 { break 'outer;}
                }
            }
            while (skip32[pos >> 5] as usize) < j { 
                j -= skip32[pos >> 5] as usize;
                pos = if pos >= 32 { pos-32} else {d.len()-1}
            }
        }
        i -= 1;
        d[pos] = i;
        skip32[pos >> 5] -= 1;
        if i == disp {
            println!("{disp}");
            disp -= 1000000;
        }
    }
    return d[pos+1];
}

fn process_skip(steps:usize) -> u32
{
    let mut d:SkipList<u32> = SkipList::new();
    d.insert(0,0);
    let mut pos = 0;
    let mut len:usize = 1;
    let mut disp = 1000000;
    for ins in 1..=50000000 {
        pos = (pos + steps) % len + 1;
        d.insert(ins,pos);
        len += 1;
        if ins == disp {
            println!("{ins}");
            disp += 1000000;
        }
    }
/*     let mut sl:Vec<u32> = vec![];
    for i in 0..len {
        sl.push(d[(i+pos) % len]);
    }
    println!("{:?}",sl);
 */    return d[pos+1];
}

fn main() {

   assert!(process1(3) == 638);
//    let fname = "input.txt";
//   let input = fs::read_to_string(fname).expect("Error readin input file");
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1, |_| {
        process1(345);
    });
    bench_result.print_stats();

    devtime.start();
    let part1 = process1(345);
    let part2 = process_skip(345);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}