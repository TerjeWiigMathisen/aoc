//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use hex::encode;

fn process1(gen1:u64, gen2:u64, samples:usize)->usize
{
    let mut p1=0;
    let mut g1 = gen1;
    let mut g2 = gen2;
    for _ in 0..samples {
        g1 = g1 * 16807; //% 2147483647;
        g1 = (g1 & 0x7fffffff) + (g1 >> 31);
        if g1 >= 2147483647 { g1 -= 2147483647; }
        g2 = g2 * 48271; // % 2147483647;
        g2 = (g2 & 0x7fffffff) + (g2 >> 31);
        if g2 >= 2147483647 { g2 -= 2147483647; }
        if (g1 ^ g2) as u16 == 0 {p1 += 1; }
    }
    return p1
}

fn process2(gen1:u64, gen2:u64, samples:usize)->usize
{
    let mut p2=0;
    let mut g1 = gen1;
    let mut g2 = gen2;
    for _ in 0..samples {
        loop {
            g1 = g1 * 16807; //% 2147483647;
            g1 = (g1 & 0x7fffffff) + (g1 >> 31);
            if g1 >= 2147483647 { g1 -= 2147483647; }
            if g1 & 3 == 0 { break;}
        }
        loop {
            g2 = g2 * 48271; // % 2147483647;
            g2 = (g2 & 0x7fffffff) + (g2 >> 31);
            if g2 >= 2147483647 { g2 -= 2147483647; }
            if g2 & 7 == 0 { break;}
        }
        if (g1 ^ g2) as u16 == 0 {p2 += 1; }
    }
    return p2;
}

fn process(input:String) -> (usize,usize)
{
    let gen1:u64 = input[24..27].parse::<u64>().unwrap();
    let gen2:u64 = input[52..55].parse::<u64>().unwrap();
    let p1 = process1(gen1, gen2, 40000000);
    let p2 = process2(gen1, gen2, 5000000);
    (p1,p2)
}

fn main() {
    assert!(process1(65, 8921, 5) == 1);
    assert!(process1(65, 8921, 40000000) == 588);
    assert!(process2(65, 8921, 1055) == 0);
    assert!(process2(65, 8921, 5000000) == 309);

    let mut devtime = DevTime::new_simple();

    let fname = "input.txt"; // instead of args[1]
    let input = fs::read_to_string(fname).expect("Error readin input file");

    //let mut dummy = 0;
    let bench_result = run_benchmark(100, |_| { 
        process(input.clone());
    }); bench_result.print_stats();

//    println!("dummy = {dummy}"); 

    devtime.start();
    let (part1, part2) = process(input); 
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} ns",devtime.time_in_nanos().unwrap());
}