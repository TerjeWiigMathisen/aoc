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
                while (addr as usize) + 2 > self.opcodes.len() { 
                    self.opcodes.push(0);
                }
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

fn permute(arr:&mut [i32]) -> bool
{
    let l = arr.len();
    if l <= 1 {return false;} // Need at least two elements to permute!

    let mut si = l-1;
    let mut bx = l-1;
    let mut ret = false;
    let mut p = arr[si];
    while si > 0 {
        si -= 1;
        let t = arr[si];
        if t < p {          // Found a pair in sorted order, permute from here
            while t >= arr[bx] { // Find last entry greater than the current
                bx -= 1;
            }
            arr[si] = arr[bx];  // Swap these two
            arr[bx] = t;
            si += 1;            // and start with the next entry
            ret = true;
            break;
        }
        p = t;
    }
    bx = l-1;                   // Reverse the tail
    while si < bx {
        let u = arr[si];
        arr[si] = arr[bx];
        arr[bx] = u;
        si += 1;
        bx -= 1;
    }
    ret
}

fn bench_permute()->i32
{
    let mut a:Vec<i32> = vec![0,1,2,3,4,5,6,7,8,9];
    let mut cnt = 1;
    while permute(&mut a) {cnt += 1;}
    print!("{cnt} ");
    cnt
}


fn encode(i:&mut VecDeque<i64>, s:&str)->i64
{
    let b = s.as_bytes();
    for c in b {
        i.push_back(*c as i64);
    }
    return b.len() as i64;
}

fn process(inp:String) -> i64
{
    let mut cpu = Cpu{ip:0,opcodes:vec![],inputs:vec![].into(),outputs:vec![],relative_base:0};
    cpu.opcodes = inp.split(",").map(|s| s.parse::<i64>().unwrap()).collect();

    encode(&mut cpu.inputs, 
"NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK\n");
    cpu.run();

    for o in cpu.outputs {
        if o > 255 {return o as i64;}
        print!("{}",o as u8 as char);
    }

    return -1;
}

fn process2(inp:String) -> i64
{
    let mut cpu = Cpu{ip:0,opcodes:vec![],inputs:vec![].into(),outputs:vec![],relative_base:0};
    cpu.opcodes = inp.split(",").map(|s| s.parse::<i64>().unwrap()).collect();

// Don't jump if d | eh | ABC
    encode(&mut cpu.inputs, 
"OR E J
OR H J
NOT J J
OR A T
AND B T
AND C T
OR T J
NOT D T
OR T J
NOT J J
RUN
");
    cpu.run();

    for o in cpu.outputs {
        if o > 255 {return o as i64;}
        print!("{}",o as u8 as char);
    }

    return -1;
}

fn main() {
//    assert!(process("3,3,1108,-1,8,3,4,3,99".to_owned(),vec![8]) == 1);
//    assert!(process("3,3,1108,-1,8,3,4,3,99".to_owned(),vec![7]) == 0);
//    assert!(process("3,3,1107,-1,8,3,4,3,99".to_owned(),vec![7]) == 1);

//    let bench_result = run_benchmark(10, |_| {bench_permute();}); bench_result.print_stats();

    //panic!("Stop now");

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == b'\n' {input.pop();}

    let mut devtime = DevTime::new_simple();

//    let bench_result = run_benchmark(1, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let part1 = process(input.clone());
    let part2 = process2(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}