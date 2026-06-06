//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn metasum(nr:&Vec<u8>, start:usize) -> (i64,usize)
{
    let mut idx = start;
    let mut sum = 0;
    let (children, metalen) = (nr[idx] as usize, nr[idx+1] as usize);
    idx += 2;
    for _ in 0..children {
        let (s, u) = metasum(nr, idx);
        sum += s;
        idx = u;
    }
    for _ in 0..metalen {
        sum += nr[idx] as i64;
        idx += 1;
    }
    return (sum, idx);
}

fn nodesum(nr:&Vec<u8>, start:usize) -> (i64,usize)
{
    let mut idx = start;
    let mut sum = 0;
    let (children, metalen) = (nr[idx] as usize, nr[idx+1] as usize);
    idx += 2;
    let mut child:Vec<i64> = vec![];
    for _ in 0..children {
        let (s, u) = nodesum(nr, idx);
        child.push(s);
        idx = u;
    }
    if children == 0 {
        for _ in 0..metalen {
            sum += nr[idx] as i64;
            idx += 1;
        }
    }
    else {
        for _ in 0..metalen {
            let i = nr[idx] as usize;
            if i > 0 && i <= children {
                sum += child[i-1];
            }
            idx += 1;
        }
    }
    return (sum, idx);
}

fn process(inp:&str) -> (i64,i64)
{
//    let nr:Vec<u8> = inp.split_ascii_whitespace().map(|s| s.parse::<u8>().unwrap()).collect();
    let mut nr:Vec<u8> = Vec::with_capacity(inp.len()/2);
    let ib = inp.as_bytes();
    let mut n;
    let mut i = 0;
    loop {
        n = ib[i];
        i += 1;
        if n >= b'0' {
            n &= 15;
            if i >= ib.len() {
                nr.push(n);
                break;
            }
            if ib[i] >= b'0' {
                n = n*10 + ib[i] & 15;
                i += 1;
                if i >= ib.len() {
                    nr.push(n);
                    break;
                }
            }
            else {
                nr.push(n);
            }
        }
    }
    let (p1, _) = metasum(&nr, 0);
    let (p2, _) = nodesum(&nr, 0);
    return (p1, p2);
    //(nr[0] as i64, nr.len() as i64)
}

fn parse1000(inp:&str)->(i64,i64)
{
    let mut part1 = 0;
    let mut part2 = 0;
    for _ in 0..1000 {
        let (p1,p2) = process(inp);
        part1 += p1;
        part2 += p2;
    }
    (part1,part2)
} 


fn main() {
//    assert!(process(
//"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2") == (138,66));

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error reading input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { parse1000(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}