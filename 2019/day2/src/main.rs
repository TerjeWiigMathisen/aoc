// Fastest 3.2 us on Surface

//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

/*
enum Opcodes {
    add,
    mul,
}
*/

#[derive(Clone,Debug)]
struct Cpu {
    ip:usize,
    opcodes:Vec<i32>,
}

impl Cpu {
    fn run(mut self)->i32
    {
        let mut ip = self.ip;
        loop {
            if ip >= self.opcodes.len() {break};
            if self.opcodes[ip] == 99 {break; }
            if ip+3 >= self.opcodes.len() {break};
            let (opc, s1, s2, d) = (self.opcodes[ip], self.opcodes[ip+1], self.opcodes[ip+2], self.opcodes[ip+3]);
            let (src1, src2) = (self.opcodes[s1 as usize], self.opcodes[s2 as usize]);
            match opc {
                1 => self.opcodes[d as usize] = src1 + src2,
                2 => self.opcodes[d as usize] = src1 * src2,
                _ => panic!("Invalid opcode!"),
            }
            ip += 4;
        }
        self.ip = ip;
        self.opcodes[0]    
    }
    fn step(self)
    {
    }
}

fn process_test(inp:&str) -> (i64,i64)
{
    let mut cpu = Cpu{ip:0,opcodes:vec![]};
    cpu.opcodes = inp.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    return (cpu.run() as i64, 0);
}

fn process(inp:&str, noun:i32, verb:i32) -> i64
{
    let mut cpu = Cpu{ip:0,opcodes:vec![]};
    cpu.opcodes = inp.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    cpu.opcodes[1] = noun;
    cpu.opcodes[2] = verb;
    return cpu.run() as i64;
}

fn process2(inp:&str) -> i32
{
    let opc:Vec<i32> = inp.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    for noun in 0..100 {
        for verb in 0..100 {
            let mut cpu = Cpu{ip:0,opcodes:opc.clone()};
            cpu.opcodes[1] = noun;
            cpu.opcodes[2] = verb;
            if cpu.run() == 19690720 { return 100*noun + verb; }
        }
    }
    return 0;
}

fn process3(inp:&str) -> (i32,i32)
{
    let opc:Vec<i32> = inp.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    let mut cpu = Cpu{ip:0, opcodes:opc.clone()};
    cpu.opcodes[1] = 12;
    cpu.opcodes[2] = 1;
    let p1 = cpu.clone().run();
    cpu.opcodes = opc.clone();
    cpu.opcodes[1] = 13;
    cpu.opcodes[2] = 13;
    let ptop = cpu.run();

    // A*12 + B + 1 = p1
    // A*13 + B + 13 = ptop
    // A = ptop-p1-12
    let a = ptop-p1-12;
    // B = p1-12*A
    let b = p1-12*a-1;
    //println!("A = {a}, B = {b}");

    for noun in 19690720/a..100 {
        let fastverb = 19690720 - noun*a - b;
        if fastverb < 0 || fastverb > 99 {continue;}
        return (p1, 100*noun+fastverb);
        // println!("noun={noun}, fastverb = {fastverb}");
        // for verb in 0..100 {
        //     let mut cpu = Cpu{ip:0,opcodes:opc.clone()};
        //     cpu.opcodes[1] = noun;
        //     cpu.opcodes[2] = verb;
        //     if cpu.run() == 19690720 { return 100*noun + verb; }
        // }
    }
    (0,0)
}


fn main() {
    assert!(process_test(
"1,9,10,3,2,3,11,0,99,30,40,50") == (3500,0));

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    //process3(&input);
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { 
        //process(&input,12,2);
        process3(&input);
    }); bench_result.print_stats();

    devtime.start();
    //let part1 = process(&input,12,2);
    let (part1,part2) = process3(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}