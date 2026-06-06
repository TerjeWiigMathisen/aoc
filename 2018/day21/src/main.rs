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

fn process0(inp:String, reg0:usize) -> (i64,i64)
{
    let lines:Vec<String> = inp.split("\n").map(|s| s.to_string()).collect();
    let ipreg = lines[0].substring(4,5).parse::<usize>().unwrap();
    let p = parser!(opi:string(alpha+) " " ai:usize " " bi:usize " " ci:usize
            => Instruction{a:ai, b:bi, c:ci, op:opi,});
    
    let mut ip = 0;
//    let mut p1 = 0;

//    let mut rand:Vec<usize> = vec![usize::MAX;0x1000000];
    let mut count = 0;
    let mut maxcount = 0;
    let mut rcnt = 0;
    let mut twice = false;
//    for tp in [2118, 4348, 6423, 7215, 7257, 8804, 16729, 17264, 19194, 20605].iter() {
//       let t = *tp;
    let mut p1 = 0;
    let mut p2 = 0;
    let t = reg0;
//    for t in 0..1 { //1..=2118 {
//        if t & 0 == 0 { println!("Testing starting reg value = {t}"); }
        let mut r28:Vec<usize> = vec![];
        let mut rand:Vec<usize> = vec![usize::MAX;0x1000000];
        ip = 0;
        let mut cpu:Cpu = Cpu{regs:vec![0;6]};
        cpu.regs[0] = t;
        count = 0;
        let mut r = t;
//        rand[r] = 1;
        let mut prev = 0;
        let mut randfill = 0;
        rcnt = 0;
        let mut python = 0;
        let mut trace = false;
        while ip+1 < lines.len() {
            count += 1;
            if ip == 18 {
                r = cpu.regs[3];
//                println!("Start {t}, rand {r}");
                cpu.regs[5] = (cpu.regs[1]) >> 8;
                cpu.regs[4] = 1;
                count += 7*cpu.regs[5] - 2 + 7;
                ip = 26;
            }
            else if ip == 28 {
                rcnt += 1;
                if (rcnt & 0xff) == 0xff {
//                    println!("Tested {rcnt} values for {t}");
//                    break;
                }
                r = cpu.regs[3];
/*
                python = innerloop(python);
                if r != python {
                    println!("Diff found: {python} vs {r}, prev: {prev}");
                }
 */
                if rand[r] < rcnt {
//                    println!("Found r28 loop after {rcnt} values");
//                    println!("Duplicate: {r}, last value: {prev}");
//                    r28.sort();
//                    for i in 0..20 { print!("{}, ",r28[i])}
//                    println!("");
//                    if r != t {break;}
                    p2 = prev;
                    break;
                }
                if r28.len() == 0 {
                    p1 = r;
                }
                r28.push(r);
                rand[r] = rcnt;
                prev = r;
                if r == 4808703 {
//                    println!("Start tracing!");
//                    trace = true;
                }
            }
            let inst = p.parse(&lines[ip+1]).unwrap();
            cpu.regs[ipreg] = ip;
            if trace {
                println!("{:10} IP:{:2}, {:12}, {:?}", count, ip, &lines[ip+1], cpu.regs);
            }
            cpu.exec(&inst.op,inst.a,inst.b,inst.c);
            ip = cpu.regs[ipreg];
            ip += 1;
/*/            if count > 4100000000000 {
                println!("TImed out for t = {t}");
                break;
            }
 */
        }
/*
        if count > maxcount {
            println!("Found New max count for {t} at {r}, after {count} instructions and {rcnt} values, previous value was {prev}");
                maxcount = count;

        }
 */
//    }
    return (p1 as i64, p2 as i64);
}

fn process(inp:String) -> (i64,i64)
{
    let (part1,part2) = process0(inp.clone(), 0);
    return (part1,part2);
}

fn innerloop(input:usize) -> usize
{
    let mut bytes = input | 65536;
    let mut hash = 10373714;
    while bytes > 0 {
        hash += bytes & 255;
        hash &= 0xffffff;
        hash *= 65899;
        hash &= 0xffffff;
        bytes >>= 8;
    }
    return hash;
}

fn p2()->usize
{
    let mut i = 0;
    let mut count = 0;
    let mut counts:Vec<usize> = vec![0;0x1000000];
    let mut prev = 0;
    loop {
        count += i * (256+65536);
        i = innerloop(i);
        if counts[i] > 0 {break;}
        counts[i] = count;
        prev = i;
    }
    return prev;
}

fn main() {
/*     assert!(process(
"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]





".to_owned()) == (1,0));
 */
    let bench_result = run_benchmark(25, |_| {
        let p1 = innerloop(0);
        let p2 = p2();
    }); bench_result.print_stats();
    //println!("Part1 = {p1}");
    //println!("Part2 = {p2}");
//    panic!("");
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(25, |_| { process(input.clone()); }); bench_result.print_stats();

//    panic!("Run once only!");
    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}