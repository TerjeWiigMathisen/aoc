use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

#[derive(Clone,Debug)]
struct Cpu {
    ip:usize,
    opcodes:Vec<i64>,
    inputs:VecDeque<i64>,
    outputs:Vec<i64>,
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
                self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); 
                ip += 2;
                continue;
            }
            let src1 = self.load(s1, mode1);
            if opc == 4 {  
                self.outputs.push(src1);
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
            while self.opcodes.len() as i64 <= d { self.opcodes.push(0);}
            let src2 = self.load(s2, mode2);
//            let dst = if mode3 == 0 {self.opcodes[d as usize]}else{d};
            let dst = self.lea(d,mode3);
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

struct Grid{
    xsize:usize,
    ysize:usize,
    pos:usize,
    dir:usize,
    grid:Vec<i8>,
}
impl Grid {
    fn get(&self)->i8
    {
        self.grid[self.pos] & 1
    }
    fn set(&mut self, color:i8)->i8
    {
        let b = self.get();
        self.grid[self.pos] = color | 2;
        b
    }
    fn step(&mut self)
    {
        match self.dir {
            0 => self.pos -= self.xsize,
            1 => self.pos += 1,
            2 => self.pos += self.xsize,
            3 => self.pos -= 1,
            _ => panic!("Invalid direction1"),
        }
    }
    fn left(&mut self)
    {
        self.dir = (self.dir+3) & 3;
    }
    fn right(&mut self)
    {
        self.dir = (self.dir+1) & 3;
    }
    fn touched(&self)->i64
    {
        let mut t = 0;
        let size = self.xsize*self.ysize;
        for b in 0..size {
            if self.grid[b] != 0 {t += 1;}
        }
        t
    }
    fn dump(&self)
    {
        let mut p = 0;
        let dirs = ['^','>','v','<'];
        for _y in 0..self.ysize {
            for _x in 0..self.xsize {
                if p == self.pos {
                    let ds = dirs[self.dir];
                    print!("{ds}");
                }
                else {
                    print!("{}", if self.grid[p] & 1 == 0 {'.'}else {'#'});
                }
                p += 1;
            }
            println!();
        }
        println!();
    }
}

fn process(inp:String) -> i64
{
    let mut cpu = Cpu{ip:0,opcodes:vec![], inputs:vec![].into(), outputs:vec![], relative_base:0};
    cpu.opcodes = inp.split(",").map(|s| s.parse::<i64>().unwrap()).collect();

    let mut grid = Grid{xsize:120,ysize:80,pos:40*120+60,dir:0,grid:vec![0;120*80]};
    loop {
        cpu.inputs.push_back(grid.get() as i64);
        cpu.run();
        if cpu.outputs.len() != 2 {break}

        let color = cpu.outputs[0];
        let turnright = cpu.outputs[1];
        cpu.outputs = vec![];
        grid.set(color as i8 | 2);
        if turnright != 0 {
            grid.right();
        }
        else {
            grid.left();
        }
        grid.step();
//        grid.dump();
    }
//    grid.dump();
    grid.touched()
}

fn process2(inp:String)
{
    let mut cpu = Cpu{ip:0,opcodes:vec![], inputs:vec![].into(), outputs:vec![], relative_base:0};
    cpu.opcodes = inp.split(",").map(|s| s.parse::<i64>().unwrap()).collect();

    let mut grid = Grid{xsize:80,ysize:20,pos:10*80+40,dir:0,grid:vec![0;80*20]};
    grid.set(1);
    loop {
        cpu.inputs.push_back(grid.get() as i64);
        cpu.run();
        if cpu.outputs.len() != 2 {break}

        let color = cpu.outputs[0];
        let turnright = cpu.outputs[1];
        cpu.outputs = vec![];
        grid.set(color as i8 | 2);
        if turnright != 0 {
            grid.right();
        }
        else {
            grid.left();
        }
        grid.step();
//        grid.dump();
    }
    grid.dump();
//    grid.touched()
}

fn main() {
/*
    process("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_owned());
    process("1102,34915192,34915192,7,4,7,99,0".to_owned());
    process("104,1125899906842624,99".to_owned());
//    assert!(process("3,3,1107,-1,8,3,4,3,99".to_owned(),vec![7]) == 1);

//    let bench_result = run_benchmark(10, |_| {bench_permute();}); bench_result.print_stats();

    panic!("Stop now");
*/
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let part1 = process(input.clone());
    process2(input.clone());
    let part2 = 0; //process(input.clone(),&[2]);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}