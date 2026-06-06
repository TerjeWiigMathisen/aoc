// Fastest run (averaged over 1000 iterations): 53 ns

//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use std::result;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

//#[inline(never)]
fn expmod(n:usize, mu:usize, ex:usize, mo:usize) -> usize
{
    let mut res = 1;
    let mut mu = mu;
    let mut ex = ex;
    while ex != 0 {
        if ex & 1 != 0 {
            res = (res * mu) % mo;
        }
        ex >>= 1;
        mu = (mu * mu) % mo;
    }
    (n * res) % mo
}

fn get2(input: &[u8]) -> (usize, usize) {
    let mut res:[usize;2] = [0;2];
    let bytes = input;
    let mut i = 80; // Skip initial fixed text!
    let mut rpos = 0;
    while i < bytes.len() {
        let b = bytes[i];
        i += 1;
        if b >= b'0' && b <= b'9' {
            let mut num = (b - b'0') as usize;
            while i < bytes.len() {
                let b = bytes[i];
                if b < b'0' || b > b'9' {
                    break;
                }
                num = num * 10 + (b - b'0') as usize;
                i += 1;
            }
            res[rpos] = num;
            rpos += 1;
            if rpos == 2 {
                break;
            }
            i += 8; // skip "column"
        }
        i += 1;
    }
    assert!(rpos == 2);
    (res[0], res[1])
}

#[inline(never)]
fn process(input:&str) -> usize
{
    let bytes = input.as_bytes();
    let (row, col) = get2(&bytes);
    let start_diag = row + col - 1;
    let ex = start_diag * (start_diag-1) / 2 + col - 1;
    // assert!(ex == 18331559);
    let n = 20151125;
    let idx = expmod(n, 252533,ex, 33554393);
    // let mut r = 1;
    // let mut c = 1;
    // let mut cnt = 0;
    // while r < row || c < col {
    //     cnt += 1;
    //     idx  = (idx * 252533) % 33554393;
    //     if r > 1 {
    //         r -= 1;
    //         c += 1;
    //     } else {
    //         r = c+1;
    //         c = 1;
    //     }
    // }
    // println!("cnt = {cnt}");
    idx
}

fn pro1000(input:&str) -> usize
{
    let mut res:usize = 0;
    for _ in 0..1000 {
        res = process(input)
    }
    res
}
fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}
    
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { pro1000(&input); }); bench_result.print_stats();

    devtime.start();
    let part1 = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}