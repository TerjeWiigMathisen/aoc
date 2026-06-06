//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
use substring::Substring;

struct Instruction {
    a:usize,
    b:usize,
    c:usize,
    op:String,
}

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
    fn exec(&mut self, o:&str, a:usize, b:usize, c:usize)
    {
        match o {
            "addr" => self.addr(a,b,c),
            "addi" => self.addi(a,b,c),
            "mulr" => self.mulr(a,b,c),
            "muli" => self.muli(a,b,c),
            "banr" => self.banr(a,b,c),
            "bani" => self.bani(a,b,c),
            "borr" => self.borr(a,b,c),
            "bori" => self.bori(a,b,c),
            "setr" => self.setr(a,b,c),
            "seti" => self.seti(a,b,c),
            "gtir" => self.gtir(a,b,c),
            "gtri" => self.gtri(a,b,c),
            "gtrr" => self.gtrr(a,b,c),
            "eqir" => self.eqir(a,b,c),
            "eqri" => self.eqri(a,b,c),
            "eqrr" => self.eqrr(a,b,c),
            _ => panic!("Impossible opcode!"),
        }
    }
/* 
    fn clone(&self) -> Cpu
    {
        return Cpu{regs:self.regs.to_owned()};
    }
 */
}

fn prime_factor_sum(n:usize) -> i64
{
    let mut factors:Vec<usize> = vec![1];
    let mut limit = n-1;
    if (n & 1) == 0 {
        factors.push(2);
        limit = n / 2;
    }
    let mut looking_for_smallest_factor = true;
    for f in (3..limit).step_by(2) {
        if n % f == 0 {
            factors.push(f);
            if looking_for_smallest_factor {
                limit = (n+f-1)/f;
                looking_for_smallest_factor = false;
            }
        }
        if n >= limit {break}
    }
    factors.push(n);
    return factors.iter().sum::<usize>() as i64;
}

fn process(inp:String, reg0:usize) -> (i64,i64)
{
    let lines:Vec<String> = inp.split("\n").map(|s| s.to_string()).collect();
    let ipreg = lines[0].substring(4,5).parse::<usize>().unwrap();
    let p = parser!(opi:string(alpha+) " " ai:usize " " bi:usize " " ci:usize
            => Instruction{a:ai, b:bi, c:ci, op:opi,});
    
    let mut ip = 0;
//    let mut p1 = 0;
    let mut cpu:Cpu = Cpu{regs:vec![0;6]};
    cpu.regs[0] = reg0;
//    let mut count = 0;
    while ip <= lines.len() {
//        count += 1;
        let inst = p.parse(&lines[ip+1]).unwrap();
        cpu.regs[ipreg] = ip;
//        println!("{:10} IP:{:2}, {:12}, {:?}", count, ip, &lines[ip+1], cpu.regs);
        cpu.exec(&inst.op,inst.a,inst.b,inst.c);
        ip = cpu.regs[ipreg];
        ip += 1;
//        println!(" {:?}", cpu.regs);
        if ip == 1 {break}
    }
    return (prime_factor_sum(cpu.regs[2]),0);
//    return (cpu.regs[0] as i64,0);
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

    let bench_result = run_benchmark(10, |_| { process(input.clone(),0); }); bench_result.print_stats();

    devtime.start();
    let (part1, _p1) = process(input.clone(),0);
    let (part2, _p2) = process(input.clone(),1);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}