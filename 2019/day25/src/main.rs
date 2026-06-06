// This is Collosal Cave style map, the correct sequence for my input is to take everything except the klein bottle,
/*
east
take antenna
west
north
take weather machine
north
east
take spool of cat6
east
south
take mug
north
west
south
south
east
*/


use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
use std::io;
use std::char;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
//use devtimer::run_benchmark;
//use substring::Substring;

#[derive(Clone,Debug)]
struct Cpu {
    ip:usize,
    opcodes:Vec<i64>,
    inputs:VecDeque<i64>,
    outputs:VecDeque<i64>,
    relative_base:i64,
}

impl Cpu {
    fn lea(&mut self, s1:i64, mode:i64)->i64
    {
        if mode == 1 {return s1;}
        let addr = if mode == 0 {s1} else {s1+self.relative_base};
        addr
    }
    fn load(&mut self, s1:i64, mode:i64)->i64
    {
        if mode == 1 {return s1;}
        let addr = if mode == 0 {s1} else {s1+self.relative_base};
        while addr >= self.opcodes.len() as i64 {
            self.opcodes.push(0);
        }
        self.opcodes[addr as usize]
    }
    fn unpack(&self, opcmode:i64) -> (i64, i64, i64, i64)
    {
        let (opc, m1, m2, m3) = 
            (opcmode % 100, (opcmode / 100) % 10, (opcmode / 1000) % 10, opcmode / 10000);
        (opc, m1, m2, m3)
    }
    fn _mem(&self, ip:usize, mode:i64)->String
    {
        let addr = self.opcodes[ip];
        if mode == 0 {return format!("[{addr}]")};
        if mode == 1 {return format!["{addr}"]};
        return format!("[{addr} + rb]");
    }
    fn _disasm(&self, startip:usize, instructions:usize)
    {
        let mut ip = startip;
        for _ in 0..instructions {
            let (opc,m1,m2,m3) = self.unpack(self.opcodes[ip]);
            match opc {
                1 => { println!("{ip:4} {:5} add {},{} -> {}", self.relative_base,
                    self._mem(ip+1, m1),
                    self._mem(ip+2, m2),
                    self._mem(ip+3, m3)); ip += 4; },
                2 => { println!("{ip:4} {:5} mul {},{} -> {}",  self.relative_base,
                    self._mem(ip+1, m1),
                    self._mem(ip+2, m2),
                    self._mem(ip+3, m3)); ip += 4; },
                3 => { println!("{ip:4} {:5} input -> {}",  self.relative_base,
                    self._mem(ip+1, m1)); ip += 2; },
                4 => { println!("{ip:4} {:5} {} -> output",  self.relative_base,
                    self._mem(ip+1, m1)); ip += 2; },
                5 => { println!("{ip:4} {:5} jnz {} -> {}",  self.relative_base,
                    self._mem(ip+1, m1),
                    self._mem(ip+2, m2)); ip += 3; },
                6 => { println!("{ip:4} {:5} jz {} -> {}",  self.relative_base,
                    self._mem(ip+1, m1),
                    self._mem(ip+2, m2)); ip += 3; },
                7 => { println!("{ip:4} {:5} setl {},{} -> {}",  self.relative_base,
                    self._mem(ip+1, m1),
                    self._mem(ip+2, m2),
                    self._mem(ip+3, m3)); ip += 4; },
                8 => { println!("{ip:4} {:5} sete {},{} -> {}",  self.relative_base,
                    self._mem(ip+1, m1),
                    self._mem(ip+2, m2),
                    self._mem(ip+3, m3)); ip += 4; },
                9 => { println!("{ip:4} {:5} rb += {}",  self.relative_base,
                    self._mem(ip+1, m1)); ip += 2; },
                99 => println!("EXIT"),
                _ => panic!("Illegal opcode {opc}"),
            }
        }

    }
    fn run(&mut self)->i64
    {
        let mut ip = self.ip;
        let mut ret = i64::MIN;
        loop {
            while ip > self.opcodes.len()+3 {self.opcodes.push(0);}
//            self.disasm(ip,1);

            let (opc, mode1, mode2, mode3) = self.unpack(self.opcodes[ip]);
                    

            if opc == 99 {break; }

            let s1 = self.opcodes[ip+1];
            if opc == 3 {
                if self.inputs.len() == 0 {break};
                let addr = self.lea(s1,mode1);
                while (addr as usize) + 2 > self.opcodes.len() { 
                    self.opcodes.push(0);
                }
                self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); 
                ip += 2;
                continue;
            }
            let src1 = self.load(s1, mode1);
            if opc == 4 {  
                self.outputs.push_back(src1);
//                println!("{src1}");
                ret = src1;
                ip += 2; 
                continue;
            }
            if opc == 9 { // Modify relative_base
                self.relative_base += src1;
                ip += 2;
                continue;
            }

            let (s2, d) = (self.opcodes[ip+2], self.opcodes[ip+3]);
            let src2 = self.load(s2, mode2);
//            let dst = if mode3 == 0 {self.opcodes[d as usize]}else{d};
            let dst = self.lea(d,mode3);
            while self.opcodes.len() as i64 <= dst { self.opcodes.push(0);}
            match opc {
                1 => {self.opcodes[dst as usize] = src1 + src2; ip += 4; },
                2 => {self.opcodes[dst as usize] = src1 * src2; ip += 4; },    
                5 => ip = if src1 != 0 {src2 as usize}else{ip + 3},
                6 => ip = if src1 == 0 {src2 as usize}else{ip + 3},
                7 => {self.opcodes[dst as usize] = (src1 < src2) as i64; ip += 4},
                8 => {self.opcodes[dst as usize] = (src1 == src2) as i64; ip += 4},
                _ => panic!("Invalid opcode!"),
            }
        }
        self.ip = ip;
        ret
    }
}

fn process(inp:&str) -> i64
{
    let mut cpu = Cpu{ip:0,opcodes:vec![],inputs:vec![].into(),outputs:vec![].into(),relative_base:0};
    cpu.opcodes = inp.split(",").map(|s| s.parse::<i64>().unwrap()).collect();

    loop {
        let mut cmd:String = "".into();
        let stdin = io::stdin();
        if stdin.read_line(&mut cmd).is_ok() {
            let cm = cmd.trim_end();
            for c in cm.as_bytes() {
                cpu.inputs.push_back(*c as i64);
            }
            cpu.inputs.push_back(10 as i64);
            println!("{:?}",cpu.inputs)
        }
        cpu.run();
        while cpu.outputs.len() > 0 {
            let c = cpu.outputs.pop_front().unwrap();
            print!("{}",c as u8 as char);
        }
    }
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == b'\n' {input.pop();}

    let mut devtime = DevTime::new_simple();

//    let bench_result = run_benchmark(0, |_| { process(&input); }); bench_result.print_stats();

    devtime.start();
    let part1 = process(&input);
//    let part2 = process2(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
//    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}