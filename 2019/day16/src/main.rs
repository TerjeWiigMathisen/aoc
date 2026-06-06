//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn rep_pattern(line:usize, minlen:usize)->Vec<i8>
{
    let master:[i8;4] = [0, 1, 0, -1];
    let mut p:Vec<i8> = vec![];
    while p.len() <= minlen {
        for i in master {
            for _ in 0..line {
                p.push(i);
                if p.len() >= minlen {
                    return p;
                }
            }
        }
    }
    p
}

fn process(inp:&str) -> String
{
    let mut signal:Vec<i8> = inp.as_bytes().into_iter().map(|n| {(n & 15) as i8}).collect();
    let len = signal.len();
    let mut patterns:Vec<Vec<i8>> = vec![vec![];len];
    for line in 1..=len {
        patterns[line-1] = rep_pattern(line, len+1);
    }
    for _phase in 0..100 {
        let mut o:Vec<i8> = vec![0;len];
        for line in 0..len {
            let p = &patterns[line];
            let mut sum:i32 = 0;
            for i in line..len {
                sum += (signal[i] * p[i+1]) as i32;
            }
            sum = sum.abs();
            sum %= 10;
            o[line] = sum as i8;
        }
        signal = o;
//        println!("After {_phase} phase: {:?}", signal);
    }
    let res = signal.into_iter().take(8).map(|n| {(n as u8 + b'0') as char}).collect();
//    println!("After 100 phases: {res}");
    res
}

fn process2(inp:&str) -> i32
{
    let mut signal:Vec<i8> = inp.as_bytes().into_iter().map(|n| {(n & 15) as i8}).collect();
    let cs = signal.clone();
    let offs = cs.clone().into_iter().
        take(7).
        map(|n| {n as u8 + b'0'}).
        collect::<Vec<u8>>();
    let off = String::from_utf8(offs).unwrap().
            parse::<usize>().unwrap();
    for _ in 0..9999 {
        signal.extend(cs.iter());
    }

    let len = signal.len();
    /*
    let mut patterns:Vec<Vec<i8>> = vec![vec![0;len];100];
    for line in 1..=len {
        patterns[line-1] = rep_pattern(line, len+len);
    }
    */
    for _phase in 0..100 {
        let mut o:Vec<i8> = vec![0;len];
//            let p = &patterns[line];
//        let mut sum:i32 = 0;
        let mut p = len-1;
        o[p] = signal[p];
        while p > off {
            p -= 1;
            o[p] = (o[p+1]+signal[p]) % 10;
        }
        signal = o;
//        let tail:[i8;80] = &o[(len-80)..len];
        let mut tail:Vec<i8> = vec![];
        for i in (off-10)..(off+70) {
            tail.push(signal[i]);
        }
//        println!("After {_phase} phase: {:?}", tail);
    }
    let mut res:i32 = 0;
    for i in off..(off+8) {
        res = res*10 + signal[i] as i32;
    }
//    println!("After 100 phases: {res}");
    res
}

fn main() {
    assert!(process("80871224585914546619083218645595") == "24176176");
    assert!(process2("03036732577212944063491565474664") == 84462026);
    assert!(process2("02935109699940807407585447034323") == 78725270);
    assert!(process2("03081770884921959731165446850517") == 53553731);
//    assert!(process("3,3,1107,-1,8,3,4,3,99".to_owned(),vec![7]) == 1);

//    let bench_result = run_benchmark(10, |_| {bench_permute();}); bench_result.print_stats();

    //panic!("Stop now");

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { 
        process(&input); 
        process2(&input);
    }); bench_result.print_stats();

    devtime.start();
    let part1 = process(&input);
    let part2 = process2(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}