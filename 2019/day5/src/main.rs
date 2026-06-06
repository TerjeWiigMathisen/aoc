use std::collections::VecDeque;
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
    inputs:VecDeque<i32>,
    outputs:Vec<i32>,
}

impl Cpu {
    fn run(mut self)->i32
    {
        let mut ip = self.ip;
        let mut ret = 0;
        loop {
            if ip >= self.opcodes.len() {break};
            let opcmode = self.opcodes[ip];
            let (opc, mode1, mode2, _mode3) = 
                    (opcmode % 100, (opcmode / 100) % 10, (opcmode / 1000) % 10, opcmode / 10000);

            if opc == 99 {break; }

            let s1 = self.opcodes[ip+1];
            if opc == 3 {
                if self.inputs.len() == 0 {break};
                self.opcodes[s1 as usize] = self.inputs.pop_front().unwrap(); 
                ip += 2;
                continue;
            }
            let src1 = if mode1 == 0 {self.opcodes[s1 as usize]}else{s1};
            if opc == 4 {  
                self.outputs.push(src1);
                //println!("{src1}");
                ret = src1;
                ip += 2; 
                continue;
            }

            let (s2, d) = (self.opcodes[ip+2], self.opcodes[ip+3]);
            let src2 = if mode2 == 0 {self.opcodes[s2 as usize]}else{s2};
//            let dst = if mode3 == 0 {self.opcodes[d as usize]}else{d};
            match opc {
                1 => {self.opcodes[d as usize] = src1 + src2; ip += 4; },
                2 => {self.opcodes[d as usize] = src1 * src2; ip += 4; },    
                5 => ip = if src1 != 0 {src2 as usize}else{ip + 3},
                6 => ip = if src1 == 0 {src2 as usize}else{ip + 3},
                7 => {self.opcodes[d as usize] = (src1 < src2) as i32; ip += 4},
                8 => {self.opcodes[d as usize] = (src1 == src2) as i32; ip += 4},
                _ => panic!("Invalid opcode!"),
            }
        }
        self.ip = ip;
        ret
    }
}

fn process(inp:&str, inputs:Vec<i32>) -> i64
{
    let mut cpu = Cpu{ip:0,opcodes:vec![],inputs:inputs.into(),outputs:vec![]};
    cpu.opcodes = inp.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    return cpu.run() as i64;
}


fn main() {
    assert!(process("3,3,1108,-1,8,3,4,3,99",vec![8]) == 1);
    assert!(process("3,3,1108,-1,8,3,4,3,99",vec![7]) == 0);
    assert!(process("3,3,1107,-1,8,3,4,3,99",vec![7]) == 1);

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { 
        process(&input,vec![1]);
        process(&input,vec![5]); 
    }); bench_result.print_stats();

    devtime.start();
    let part1 = process(&input,vec![1]);
    let part2 = process(&input,vec![5]);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}