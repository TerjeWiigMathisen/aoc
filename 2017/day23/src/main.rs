//use std::io;
//use std::env;
use std::fs;
//use std::collections::VecDeque;
//use std::collections::HashMap;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

fn process(inp:String) -> i64
{
    let mut reg:Vec<i64> = vec![0;26];

    let lines:Vec<&str> = inp.split("\n").collect();
    let mut ip:i64 = 0;
    let mut muls:i64 = 0;
    loop {
        let li = lines[ip as usize];
        if li.len() == 0 {break; }
        let words:Vec<&str> = li.split(" ").collect();
        let mut r:usize = 0;
        let mut val = 0;
        let w1 = words[1].as_bytes()[0] as i64 - 'a' as i64;
        let mut r1imm = false;
        let mut r1i = 0;
        if w1 >= 0 {
            r = w1 as usize;
//            print!("reg {r} = {}", reg[r]);
        }
        else {
            r1i = words[1].parse::<i64>().unwrap();
//            print!("r1 imm = {}", r1i);
            r1imm = true;
        }

        if words.len() > 2 {
            let w2 = words[2].as_bytes()[0] as i64 - 'a' as i64;
            if w2 >= 0 {
                val = reg[w2 as usize];
            }
            else {
                val = words[2].parse::<i64>().unwrap();
            }
        }
        match words[0] {
            "set" => reg[r] = val,
            "sub" => reg[r] -= val,
            "mul" => { reg[r] *= val; muls += 1; },
            "jnz" => if (r1imm && r1i != 0) || (!r1imm && reg[r] != 0) {ip += val - 1;},
            _ => println!("Bad instruction {}",li.clone()),
        }
        ip += 1;
        if ip < 0 || ip as usize >= lines.len() {
            println!("Bad IP: {ip}");
            break;
        }
    }
    return muls;
}

fn is_prime(i:usize)->bool
{
    if (i & 1) == 0 {return false;}
    for n in (3..i).step_by(2) {
        let r = i / n;
        if r*n == i { return false; }
        if n > r { break;}
    }
    return true;
}

fn process2() -> i64
{
    let mut primes:i64 = 0;
    for i in (108400..=125400).step_by(17) {
        if !is_prime(i) { primes += 1; }
    }
    return primes;
}

fn process3() -> i64
{
    let mut primes:i64 = 0;
    for b in (108400..=125400).step_by(17) {
        let mut f:i64 = 1;
        for d in 2..=(b/2) {
            let e:usize = b / d;
            if d*e == b {
                f = 0;
                break;
            }
        }
        primes += (1-f);
    }
    return primes;
}

fn main() {

    let fname = "input.txt"; // instead of args[1]
    let input = fs::read_to_string(fname).expect("Error readin input file");
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1, |_| {
        process(input.clone());
    });
    bench_result.print_stats();

    devtime.start();
    let part1 = process(input.clone());
    let part2 = process2();
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}