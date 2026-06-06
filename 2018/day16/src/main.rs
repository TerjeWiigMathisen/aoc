//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

#[derive(PartialEq)]
struct Cpu {
    regs:Vec<usize>,
}
impl Cpu {
    fn addr(&mut self, a:usize, b:usize, c:usize)
    {
        self.regs[c] = self.regs[a] + self.regs[b];
    }
    fn addi(&mut self, a:usize, b:usize, c:usize)
    {
        self.regs[c] = self.regs[a] + b;
    }
    fn mulr(&mut self, a:usize, b:usize, c:usize)
    {
        self.regs[c] = self.regs[a] * self.regs[b];
    }
    fn muli(&mut self, a:usize, b:usize, c:usize)
    {
        self.regs[c] = self.regs[a] * b;
    }
    fn banr(&mut self, a:usize, b:usize, c:usize)
    {
        self.regs[c] = self.regs[a] & self.regs[b];
    }
    fn bani(&mut self, a:usize, b:usize, c:usize)
    {
        self.regs[c] = self.regs[a] & b;
    }
    fn borr(&mut self, a:usize, b:usize, c:usize)
    {
        self.regs[c] = self.regs[a] | self.regs[b];
    }
    fn bori(&mut self, a:usize, b:usize, c:usize)
    {
        self.regs[c] = self.regs[a] | b;
    }
    fn setr(&mut self, a:usize, _b:usize, c:usize)
    {
        self.regs[c] = self.regs[a];
    }
    fn seti(&mut self, a:usize, _b:usize, c:usize)
    {
        self.regs[c] = a;
    }
    fn gtir(&mut self, a:usize, b:usize, c:usize)
    {
        self.regs[c] = (a > self.regs[b]) as usize;
    }
    fn gtri(&mut self, a:usize, b:usize, c:usize)
    {
        self.regs[c] = (self.regs[a] > b) as usize;
    }
    fn gtrr(&mut self, a:usize, b:usize, c:usize)
    {
        self.regs[c] = (self.regs[a] > self.regs[b]) as usize;
    }
    fn eqir(&mut self, a:usize, b:usize, c:usize)
    {
        self.regs[c] = (a == self.regs[b]) as usize;
    }
    fn eqri(&mut self, a:usize, b:usize, c:usize)
    {
        self.regs[c] = (self.regs[a] == b) as usize;
    }
    fn eqrr(&mut self, a:usize, b:usize, c:usize)
    {
        self.regs[c] = (self.regs[a] == self.regs[b]) as usize;
    }
    fn exec(&mut self, o:usize, a:usize, b:usize, c:usize)
    {
        assert!(o < 16);
        match o {
            0 => self.addr(a,b,c),
            1 => self.addi(a,b,c),
            2 => self.mulr(a,b,c),
            3 => self.muli(a,b,c),
            4 => self.banr(a,b,c),
            5 => self.bani(a,b,c),
            6 => self.borr(a,b,c),
            7 => self.bori(a,b,c),
            8 => self.setr(a,b,c),
            9 => self.seti(a,b,c),
            10=> self.gtir(a,b,c),
            11=> self.gtri(a,b,c),
            12=> self.gtrr(a,b,c),
            13=> self.eqir(a,b,c),
            14=> self.eqri(a,b,c),
            15=> self.eqrr(a,b,c),
            _ => panic!("Impossible opcode!"),
        }
    }
    fn clone(&self) -> Cpu
    {
        return Cpu{regs:self.regs.to_owned()};
    }
}


fn process(inp:String) -> (i64,i64)
{
    let lines:Vec<String> = inp.split("\n").map(|s| s.to_string()).collect();
    let pb = parser!("Before: [" a:usize ", " b:usize ", " c:usize ", " d:usize "]" 
            => Cpu{regs:vec![a,b,c,d]});
    let pa = parser!("After:  [" a:usize ", " b:usize ", " c:usize ", " d:usize "]" 
            => Cpu{regs:vec![a,b,c,d]});
    
    let mut lnr = 0;
    let mut p1 = 0;
    let mut fits:Vec<usize> = vec![0;16];
    loop {
        let before = lines[lnr].to_owned();
        let instruction = lines[lnr+1].to_owned();
        let after = lines[lnr+2].to_owned();

        if before.len() == 0 {break}

        let cb = pb.parse(&before).unwrap();
        let opcode:Vec<usize> = instruction.split_ascii_whitespace()
            .map(|x| x.parse::<usize>().expect("Wanted a number!")).collect();
        let ca = pa.parse(&after).unwrap();

        let (o,a,b,c) = (opcode[0],opcode[1],opcode[2],opcode[3]);

        let mut candidate = 0;
        for instruction in 0..16 {
//            let mut ci = Cpu{regs:vec![cb.regs[0],cb.regs[1],cb.regs[2],cb.regs[3]]};
            let mut ci = cb.clone();
            ci.exec(instruction,a,b,c);
            if ci == ca {
                candidate += 1;
                fits[o] |= 1 << instruction;
            }
        }
/*
        let mut ci = Cpu{regs:vec![cb.regs[0],cb.regs[1],cb.regs[2],cb.regs[3]]};
        ci.addr(a,b,c);
        if ci == ca {candidate += 1; fits[o] |= 1<<0;}
    
        ci.regs = cb.regs.clone();
        ci.addi(a,b,c);
        if ci == ca {candidate += 1;  fits[o] |= 1<<1;}
    
        ci.regs = cb.regs.clone();
        ci.mulr(a,b,c);
        if ci == ca {candidate += 1;  fits[o] |= 1<<2;}
    
        ci.regs = cb.regs.clone();
        ci.muli(a,b,c);
        if ci == ca {candidate += 1;  fits[o] |= 1<<3;}
    
        ci.regs = cb.regs.clone();
        ci.banr(a,b,c);
        if ci == ca {candidate += 1;  fits[o] |= 1<<4;}
    
        ci.regs = cb.regs.clone();
        ci.bani(a,b,c);
        if ci == ca {candidate += 1;  fits[o] |= 1<<5;}
    
        ci.regs = cb.regs.clone();
        ci.borr(a,b,c);
        if ci == ca {candidate += 1;  fits[o] |= 1<<6;}
    
        ci.regs = cb.regs.clone();
        ci.bori(a,b,c);
        if ci == ca {candidate += 1;  fits[o] |= 1<<7;}
    
        ci.regs = cb.regs.clone();
        ci.setr(a,b,c);
        if ci == ca {candidate += 1;  fits[o] |= 1<<8;}
    
        ci.regs = cb.regs.clone();
        ci.seti(a,b,c);
        if ci == ca {candidate += 1;  fits[o] |= 1<<9;}
    
        ci.regs = cb.regs.clone();
        ci.gtir(a,b,c);
        if ci == ca {candidate += 1;  fits[o] |= 1<<10;}
    
        ci.regs = cb.regs.clone();
        ci.gtri(a,b,c);
        if ci == ca {candidate += 1;  fits[o] |= 1<<11;}
    
        ci.regs = cb.regs.clone();
        ci.gtrr(a,b,c);
        if ci == ca {candidate += 1;  fits[o] |= 1<<12;}
    
        ci.regs = cb.regs.clone();
        ci.eqir(a,b,c);
        if ci == ca {candidate += 1;  fits[o] |= 1<<13;}
    
        ci.regs = cb.regs.clone();
        ci.eqri(a,b,c);
        if ci == ca {candidate += 1;  fits[o] |= 1<<14;}
    
        ci.regs = cb.regs.clone();
        ci.eqrr(a,b,c);
        if ci == ca {candidate += 1;  fits[o] |= 1<<15;}
*/
        if candidate >= 3 {
            p1 += 1;
        }

        lnr += 4;
    }
    let mut mapping:Vec<usize> = vec![0;16];
    let mut revmap:Vec<usize> = vec![0;16];
/*  for i in 0..16 {
        println!("{i:2} {:x}", fits[i]);
    }
*/
    let mut mapped = 0;
    while mapped < 16 {
        for i in 0..16 {
            for b in 0..16 {
                if fits[i] == 1 << b {
//                    println!("Found single match for operation {i} -> instruction # {b}");
                    mapped += 1;
                    mapping[b] = i;
                    revmap[i] = b;
                    for j in 0..16 {
                        fits[j] &= !(1 << b);
                    }
                }
            }
        }
    }
/*  for i in 0..16 {
        println!("i:{i} -> {}", revmap[i]);
    }
*/
    let mut cpu:Cpu = Cpu{regs:[0,0,0,0].to_vec()};
    while lnr < lines.len() {
        let line = lines[lnr].to_owned();
        lnr += 1;
        if line.len() == 0 {continue}
        let opcode:Vec<usize> = line.split_ascii_whitespace()
            .map(|x| x.parse::<usize>().expect("Wanted a number!")).collect();
        cpu.exec(revmap[opcode[0]],opcode[1],opcode[2],opcode[3]);
    }
    return (p1,cpu.regs[0] as i64);
}



fn main() {
/*     assert!(process(
"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]





".to_owned()) == (1,0));
 */
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}