use std::collections::VecDeque;
use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;
//use tcod::Console;
//use tcod::console::Root;
//use win32console::console::WinConsole;


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

fn show(grid:&HashMap<String,u8>)
{
    let mut xmin = i32::MAX;
    let mut ymin = i32::MAX;
    let mut xmax = i32::MIN;
    let mut ymax = i32::MIN;
    let keys:Vec<&String> = grid.keys().collect();
    for k in keys {
        let xy:Vec<i32> = k.split(",").map(|m| m.parse::<i32>().unwrap()).collect();
        if xy[0] < xmin {xmin = xy[0];}
        if xy[0] > xmax {xmax = xy[0];}
        if xy[1] < ymin {ymin = xy[1];}
        if xy[1] > ymax {ymax = xy[1];}
    }
    println!("({xmin},{ymin}) -> ({xmax},{ymax})");
    let blocks = [' ','#','-','=','O'];
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            let key = format!("{x},{y}");
            let mut b = 0;
            if grid.contains_key(&key) {
                b = *grid.get(&key).unwrap();
            }
            print!("{}", blocks[b as usize]);
        }
        println!();
    }
}

fn process(inp:&str) -> i64
{
    let mut cpu = Cpu{ip:0,opcodes:vec![], inputs:vec![].into(), outputs:vec![], relative_base:0};
    cpu.opcodes = inp.split(",").map(|s| s.parse::<i64>().unwrap()).collect();

    let mut grid:HashMap<String,u8> = HashMap::new();
    cpu.run();
//    println!("{:?}\n{}", cpu.outputs, cpu.outputs.len());
    for o in (0..cpu.outputs.len()).step_by(3) {
        let (x, y, b) = (cpu.outputs[o],cpu.outputs[o+1],cpu.outputs[o+2]);
        let key = format!("{x},{y}");
        grid.insert(key, b as u8);
    }
    let mut blocks = 0;
    let values:Vec<u8> = grid.values().map(|b| {*b}).collect();
    for v in values {
        if v == 2 {blocks += 1;}
    }
    //show(&grid);

    blocks
}

fn process2(inp:&str)->i64
{
    let mut cpu = Cpu{ip:0,opcodes:vec![], inputs:vec![].into(), outputs:vec![], relative_base:0};
    cpu.opcodes = inp.split(",").map(|s| s.parse::<i64>().unwrap()).collect();
    cpu.opcodes[0] = 2;

    let mut grid:HashMap<String,u8> = HashMap::new();
    let mut score = 0;
//    let mut root = Root::initializer().size(40,20).init();
//    cpu.inputs = vec![0,0,0,0,0,0,0,0,0,-1,-1,-1,-1,-1,-1,-1,-1,-1].into();
    let blocks = [' ','#','-','=','O'];
    let mut bx = 0;
    let mut by = 0;
    let mut bdx = 0;
    let mut bdy = 0;
    let mut px = 0;
    loop {
        cpu.run();
    //    println!("{:?}\n{}", cpu.outputs, cpu.outputs.len());
        let out = cpu.outputs.clone();
        cpu.outputs = vec![];
        for o in (0..out.len()).step_by(3) {
            let (x, y, b) = (out[o],out[o+1],out[o+2]);
            if x == -1 && y == 0 {
//                println!("Score = {b}");
                score = b;
                continue;
            }
            let key = format!("{x},{y}");
            grid.insert(key, b as u8);
//           root.set_char(x as i32, y as i32, blocks[b as usize]);
            if b == 3 { // paddle
                px = x;
            }
            else if b == 4 {
                bdx = x - bx;
                bdy = y - by;
                bx = x;
                by = y;
                let s = (bx-px).signum();
                if cpu.inputs.len() == 0 {
                    cpu.inputs.push_back(s);
                }
//                show(&grid);
            }
            //if b >= 3 { show(&grid);}
        }
//        show(&grid);
        if cpu.opcodes[cpu.ip] % 100 != 3 {break;}
    }
    score
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

    let bench_result = run_benchmark(25, |_| { 
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