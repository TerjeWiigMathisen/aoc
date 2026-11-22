// Surface Fastest: 366978400 ns
// HP Fastest: 1630600 ns
// HP with samply: Fastest: 1668700 ns
// Acer Fastest: 1809500 ns

use std::collections::VecDeque;
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

#[derive(Clone,Debug)]
struct Cpu {
    ip:usize,
    opcodes:Vec<i32>,
    cache:Vec<u8>,
    inputs:VecDeque<i32>,
    outputs:Vec<i32>,
    relative_base:i32,
}

impl Cpu {
    fn lea(&mut self, s1:i32, mode:i32)->i32
    {
        let mode = mode as u32;
        if mode == 1 {return s1;}
        let addr = if mode == 0 {s1} else {s1+self.relative_base};
        addr
    }
    fn load(&mut self, s1:i32, mode:i32)->i32
    {
        let mode = mode as u32;
        if mode == 1 {return s1;}
        let addr = if mode == 0 {s1} else {s1+self.relative_base};
//        while addr >= self.opcodes.len() as i32 { self.opcodes.push(0); }
        self.opcodes[addr as usize]
    }
    fn unpack(&self, opcmode:i32) -> (i32, i32, i32, i32)
    {
        let opcmode = opcmode as u32;
        let (opc, m1, m2, m3) = 
            (opcmode % 100, (opcmode / 100) % 10, (opcmode / 1000) % 10, opcmode / 10000);

        (opc as i32, m1 as i32, m2 as i32, m3 as i32)
    }
    fn _mem(&self, ip:usize, mode:i32)->String
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
    fn run(&mut self)->i32
    {
        let mut ip = self.ip;
        let mut ret = i32::MIN;
        loop {
            let mut o = self.cache[ip];
            if o == 255 {
                let (mut opc, mut mode1, mut mode2, mut mode3) = self.unpack(self.opcodes[ip]);
                if opc == 99 { opc = 243;} 
                else { 
                    if opc == 9 {opc = 0};
                }
                if mode1 > 2 { mode1 = 2;}
                if mode2 > 2 { mode2 = 2;}
                if mode3 > 2 { mode3 = 2;}
                o = (opc + mode1*9 + mode2*27 + mode3*81) as u8;
                self.cache[ip] = o;
            }
            match o {
  0 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[s1 as usize];  ret = src1; ip += 2;}
  1 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[d  as usize];
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
  2 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
  3 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = self.opcodes[s1 as usize]; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
  4 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[s1 as usize]; self.outputs.push(src1); ret = src1; ip += 2;}
  5 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[s2 as usize]; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
  6 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[s2 as usize]; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
  7 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
  8 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
  9 => { let s1 = self.opcodes[ip+1]; let src1 = s1;  ret = src1; ip += 2;}
 10 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = s1; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[d  as usize];
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
 11 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
 12 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = s1; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
 13 => { let s1 = self.opcodes[ip+1]; let src1 = s1; self.outputs.push(src1); ret = src1; ip += 2;}
 14 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = s1; let src2 = self.opcodes[s2 as usize]; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
 15 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = s1; let src2 = self.opcodes[s2 as usize]; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
 16 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
 17 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
 18 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[(s1+self.relative_base) as usize];  ret = src1; ip += 2;}
 19 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[d  as usize];
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
 20 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
 21 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = self.opcodes[(s1+self.relative_base) as usize]; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
 22 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; self.outputs.push(src1); ret = src1; ip += 2;}
 23 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[s2 as usize]; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
 24 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[s2 as usize]; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
 25 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
 26 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
 27 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[s1 as usize];  ret = src1; ip += 2;}
 28 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = self.opcodes[s1 as usize]; let src2 = s2; let dst = self.opcodes[d  as usize];
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
 29 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = s2; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
 30 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = self.opcodes[s1 as usize]; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
 31 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[s1 as usize]; self.outputs.push(src1); ret = src1; ip += 2;}
 32 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[s1 as usize]; let src2 = s2; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
 33 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[s1 as usize]; let src2 = s2; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
 34 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = s2; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
 35 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = s2; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
 36 => { let s1 = self.opcodes[ip+1]; let src1 = s1;  ret = src1; ip += 2;}
 37 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = s1; let src2 = s2; let dst = self.opcodes[d  as usize];
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
 38 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = s2; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
 39 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = s1; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
 40 => { let s1 = self.opcodes[ip+1]; let src1 = s1; self.outputs.push(src1); ret = src1; ip += 2;}
 41 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = s1; let src2 = s2; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
 42 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = s1; let src2 = s2; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
 43 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = s2; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
 44 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = s2; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
 45 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[(s1+self.relative_base) as usize];  ret = src1; ip += 2;}
 46 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = s2; let dst = self.opcodes[d  as usize];
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
 47 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = s2; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
 48 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = self.opcodes[(s1+self.relative_base) as usize]; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
 49 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; self.outputs.push(src1); ret = src1; ip += 2;}
 50 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = s2; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
 51 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = s2; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
 52 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = s2; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
 53 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = s2; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
 54 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[s1 as usize];  ret = src1; ip += 2;}
 55 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[d  as usize];
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
 56 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
 57 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = self.opcodes[s1 as usize]; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
 58 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[s1 as usize]; self.outputs.push(src1); ret = src1; ip += 2;}
 59 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
 60 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
 61 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
 62 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
 63 => { let s1 = self.opcodes[ip+1]; let src1 = s1;  ret = src1; ip += 2;}
 64 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = s1; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[d  as usize];
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
 65 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
 66 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = s1; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
 67 => { let s1 = self.opcodes[ip+1]; let src1 = s1; self.outputs.push(src1); ret = src1; ip += 2;}
 68 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = s1; let src2 = self.opcodes[(s2+self.relative_base) as usize]; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
 69 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = s1; let src2 = self.opcodes[(s2+self.relative_base) as usize]; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
 70 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
 71 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
 72 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[(s1+self.relative_base) as usize];  ret = src1; ip += 2;}
 73 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[d  as usize];
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
 74 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
 75 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = self.opcodes[(s1+self.relative_base) as usize]; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
 76 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; self.outputs.push(src1); ret = src1; ip += 2;}
 77 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
 78 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
 79 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
 80 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[d  as usize]; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
 81 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[s1 as usize];  ret = src1; ip += 2;}
 82 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[s2 as usize]; let dst = d;
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
 83 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[s2 as usize]; let dst = d; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
 84 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = self.opcodes[s1 as usize]; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
 85 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[s1 as usize]; self.outputs.push(src1); ret = src1; ip += 2;}
 86 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[s2 as usize]; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
 87 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[s2 as usize]; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
 88 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[s2 as usize]; let dst = d; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
 89 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[s2 as usize]; let dst = d; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
 90 => { let s1 = self.opcodes[ip+1]; let src1 = s1;  ret = src1; ip += 2;}
 91 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = s1; let src2 = self.opcodes[s2 as usize]; let dst = d;
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
 92 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = self.opcodes[s2 as usize]; let dst = d; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
 93 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = s1; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
 94 => { let s1 = self.opcodes[ip+1]; let src1 = s1; self.outputs.push(src1); ret = src1; ip += 2;}
 95 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = s1; let src2 = self.opcodes[s2 as usize]; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
 96 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = s1; let src2 = self.opcodes[s2 as usize]; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
 97 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = self.opcodes[s2 as usize]; let dst = d; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
 98 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = self.opcodes[s2 as usize]; let dst = d; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
 99 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[(s1+self.relative_base) as usize];  ret = src1; ip += 2;}
100 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[s2 as usize]; let dst = d;
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
101 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[s2 as usize]; let dst = d; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
102 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = self.opcodes[(s1+self.relative_base) as usize]; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
103 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; self.outputs.push(src1); ret = src1; ip += 2;}
104 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[s2 as usize]; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
105 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[s2 as usize]; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
106 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[s2 as usize]; let dst = d; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
107 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[s2 as usize]; let dst = d; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
108 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[s1 as usize];  ret = src1; ip += 2;}
109 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = self.opcodes[s1 as usize]; let src2 = s2; let dst = d;
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
110 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = s2; let dst = d; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
111 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = self.opcodes[s1 as usize]; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
112 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[s1 as usize]; self.outputs.push(src1); ret = src1; ip += 2;}
113 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[s1 as usize]; let src2 = s2; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
114 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[s1 as usize]; let src2 = s2; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
115 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = s2; let dst = d; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
116 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = s2; let dst = d; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
117 => { let s1 = self.opcodes[ip+1]; let src1 = s1;  ret = src1; ip += 2;}
118 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = s1; let src2 = s2; let dst = d;
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
119 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = s2; let dst = d; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
120 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = s1; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
121 => { let s1 = self.opcodes[ip+1]; let src1 = s1; self.outputs.push(src1); ret = src1; ip += 2;}
122 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = s1; let src2 = s2; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
123 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = s1; let src2 = s2; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
124 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = s2; let dst = d; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
125 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = s2; let dst = d; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
126 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[(s1+self.relative_base) as usize];  ret = src1; ip += 2;}
127 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = s2; let dst = d;
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
128 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = s2; let dst = d; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
129 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = self.opcodes[(s1+self.relative_base) as usize]; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
130 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; self.outputs.push(src1); ret = src1; ip += 2;}
131 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = s2; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
132 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = s2; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
133 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = s2; let dst = d; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
134 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = s2; let dst = d; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
135 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[s1 as usize];  ret = src1; ip += 2;}
136 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = d;
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
137 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = d; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
138 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = self.opcodes[s1 as usize]; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
139 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[s1 as usize]; self.outputs.push(src1); ret = src1; ip += 2;}
140 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
141 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
142 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = d; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
143 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = d; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
144 => { let s1 = self.opcodes[ip+1]; let src1 = s1;  ret = src1; ip += 2;}
145 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = s1; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = d;
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
146 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = d; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
147 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = s1; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
148 => { let s1 = self.opcodes[ip+1]; let src1 = s1; self.outputs.push(src1); ret = src1; ip += 2;}
149 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = s1; let src2 = self.opcodes[(s2+self.relative_base) as usize]; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
150 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = s1; let src2 = self.opcodes[(s2+self.relative_base) as usize]; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
151 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = d; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
152 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = d; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
153 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[(s1+self.relative_base) as usize];  ret = src1; ip += 2;}
154 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = d;
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
155 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = d; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
156 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = self.opcodes[(s1+self.relative_base) as usize]; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
157 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; self.outputs.push(src1); ret = src1; ip += 2;}
158 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
159 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
160 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = d; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
161 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = d; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
162 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[s1 as usize];  ret = src1; ip += 2;}
163 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[(d +self.relative_base) as usize];
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
164 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
165 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = self.opcodes[s1 as usize]; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
166 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[s1 as usize]; self.outputs.push(src1); ret = src1; ip += 2;}
167 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[s2 as usize]; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
168 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[s2 as usize]; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
169 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
170 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
171 => { let s1 = self.opcodes[ip+1]; let src1 = s1;  ret = src1; ip += 2;}
172 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = s1; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[(d +self.relative_base) as usize];
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
173 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
174 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = s1; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
175 => { let s1 = self.opcodes[ip+1]; let src1 = s1; self.outputs.push(src1); ret = src1; ip += 2;}
176 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = s1; let src2 = self.opcodes[s2 as usize]; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
177 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = s1; let src2 = self.opcodes[s2 as usize]; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
178 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
179 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
180 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[(s1+self.relative_base) as usize];  ret = src1; ip += 2;}
181 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[(d +self.relative_base) as usize];
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
182 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
183 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = self.opcodes[(s1+self.relative_base) as usize]; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
184 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; self.outputs.push(src1); ret = src1; ip += 2;}
185 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[s2 as usize]; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
186 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[s2 as usize]; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
187 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
188 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[s2 as usize]; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
189 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[s1 as usize];  ret = src1; ip += 2;}
190 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = self.opcodes[s1 as usize]; let src2 = s2; let dst = self.opcodes[(d +self.relative_base) as usize];
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
191 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = s2; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
192 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = self.opcodes[s1 as usize]; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
193 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[s1 as usize]; self.outputs.push(src1); ret = src1; ip += 2;}
194 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[s1 as usize]; let src2 = s2; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
195 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[s1 as usize]; let src2 = s2; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
196 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = s2; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
197 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = s2; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
198 => { let s1 = self.opcodes[ip+1]; let src1 = s1;  ret = src1; ip += 2;}
199 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = s1; let src2 = s2; let dst = self.opcodes[(d +self.relative_base) as usize];
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
200 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = s2; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
201 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = s1; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
202 => { let s1 = self.opcodes[ip+1]; let src1 = s1; self.outputs.push(src1); ret = src1; ip += 2;}
203 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = s1; let src2 = s2; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
204 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = s1; let src2 = s2; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
205 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = s2; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
206 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = s2; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
207 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[(s1+self.relative_base) as usize];  ret = src1; ip += 2;}
208 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = s2; let dst = self.opcodes[(d +self.relative_base) as usize];
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
209 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = s2; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
210 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = self.opcodes[(s1+self.relative_base) as usize]; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
211 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; self.outputs.push(src1); ret = src1; ip += 2;}
212 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = s2; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
213 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = s2; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
214 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = s2; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
215 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = s2; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
216 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[s1 as usize];  ret = src1; ip += 2;}
217 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[(d +self.relative_base) as usize];
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
218 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
219 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = self.opcodes[s1 as usize]; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
220 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[s1 as usize]; self.outputs.push(src1); ret = src1; ip += 2;}
221 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
222 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
223 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
224 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[s1 as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
225 => { let s1 = self.opcodes[ip+1]; let src1 = s1;  ret = src1; ip += 2;}
226 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = s1; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[(d +self.relative_base) as usize];
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
227 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
228 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = s1; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
229 => { let s1 = self.opcodes[ip+1]; let src1 = s1; self.outputs.push(src1); ret = src1; ip += 2;}
230 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = s1; let src2 = self.opcodes[(s2+self.relative_base) as usize]; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
231 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = s1; let src2 = self.opcodes[(s2+self.relative_base) as usize]; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
232 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
233 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = s1; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
234 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[(s1+self.relative_base) as usize];  ret = src1; ip += 2;}
235 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];
	 let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[(d +self.relative_base) as usize];
	self.opcodes[dst as usize] = src1 + src2; ip += 4; },
236 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = src1 * src2; ip += 4; },
237 => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = self.opcodes[(s1+self.relative_base) as usize]; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }
238 => { let s1 = self.opcodes[ip+1]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; self.outputs.push(src1); ret = src1; ip += 2;}
239 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; ip = if src1 != 0 {src2 as usize}else{ip + 3} }
240 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; ip = if src1 == 0 {src2 as usize}else{ip + 3} }
241 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },
242 => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = self.opcodes[(s1+self.relative_base) as usize]; let src2 = self.opcodes[(s2+self.relative_base) as usize]; let dst = self.opcodes[(d +self.relative_base) as usize]; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },
243 => break,
_ => panic!("Impossible opcode!"),
            }
        }
        self.ip = ip;
        ret
    }
}

fn _show(grid:&Vec<Vec<u8>>)
{
    println!("(0,0) -> ({},{})", grid[0].len()-1, grid.len()-1);
    let blocks = [' ','#','-','=','O'];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let b = grid[y][x];
            print!("{}", blocks[b as usize]);
        }
        println!();
    }
}

fn process(inp:&str) -> i32
{
    let mut cpu = Cpu{ip:0,opcodes:vec![], cache:vec![], inputs:vec![].into(), outputs:vec![], relative_base:0};
    cpu.opcodes = inp.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    cpu.cache = vec![255;cpu.opcodes.len()];
    cpu.opcodes.resize(23000,0);

    let mut grid:Vec<Vec<u8>> = vec![vec![0;37];20];
    cpu.run();
//    println!("{:?}\n{}", cpu.outputs, cpu.outputs.len());
    for o in (0..cpu.outputs.len()).step_by(3) {
        let (x, y, b) = (cpu.outputs[o],cpu.outputs[o+1],cpu.outputs[o+2]);
        grid[y as usize][x as usize] = b as u8;
    }
//    show(&grid);
    let mut blocks = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 2 { blocks += 1;}
        }
    }

    blocks
}

fn process2(inp:&str)->i32
{
    let mut cpu = Cpu{ip:0,opcodes:vec![], cache:vec![], inputs:vec![].into(), outputs:vec![], relative_base:0};
    cpu.opcodes = inp.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    cpu.cache = vec![255;cpu.opcodes.len()];
    cpu.opcodes[0] = 2;
    cpu.opcodes.resize(23000,0);

//    let mut grid:Vec<Vec<u8>> = vec![vec![0;37];20];
    let mut score = 0;
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
//            grid[y as usize][x as usize] = b as u8;
            if b == 3 { // paddle
                px = x;
            }
            else if b == 4 {
                let s = (x-px).signum();
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
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { 
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