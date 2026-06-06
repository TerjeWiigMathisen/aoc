// Fastest run: 282 us on Acer

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
    opcodes:Vec<i32>,
    inputs:VecDeque<i32>,
    outputs:Vec<i32>,
}

fn splitopcode(opcmode:i32) -> (u16, u8, u8, u8)
{
    let mut scaled_opc:u64 = (opcmode as u64) * 0x100000000/10000 + 1;
    let m3 = (scaled_opc >> 32) as u8; 
    scaled_opc &= 0xffffffff;
    scaled_opc *= 5;
    let m2 = (scaled_opc >> 31) as u8;
    scaled_opc &= 0x7fffffff;
    scaled_opc *= 5;
    let m1 = (scaled_opc >> 30) as u8;
    scaled_opc &= 0x3fffffff;
    scaled_opc *= 25;
    let opc = (scaled_opc >> 28) as u16;
    (opc, m1, m2, m3)
}

fn test_split()
{
    for m3 in 0..10 {
        for m2 in 0..10 {
            for m1 in 0..10 {
                for op in 0..99 {
                    let opcmode = op + m1 as i32*100 + m2 as i32*1000 + m3 as i32*10000;
                    let (a, a1, a2, a3) = splitopcode(opcmode);
                    assert!(a == op as u16 && a1 == m1 && a2 == m2 && a3 == m3);
                }
            }
        }
    }
}
impl Cpu {
    fn run(&mut self)->i32
    {
        let mut ip = self.ip;
        let mut ret = i32::MIN;
        loop {
            if ip >= self.opcodes.len() {break};
            let opcmode = self.opcodes[ip];
            let (opc, mode1, mode2, _mode3) = 
                //splitopcode(opcmode);
                    (opcmode % 100, (opcmode / 100) % 10, (opcmode / 1000) % 10, opcmode / 10000);

            if opc == 99 {break; }

            if ip+1 >= self.opcodes.len() {break};
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
//                println!("{src1}");
                ret = src1;
                ip += 2; 
                continue;
            }

            if ip+3 >= self.opcodes.len() {break};
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

fn _bench_permute()->i32
{
    let mut a:Vec<i32> = vec![0,1,2,3,4]; //,5,6,7,8,9];
    let mut cnt = 1;
    while permute(&mut a) {cnt += 1;}
    print!("{cnt} ");
    cnt
}

fn process(inp:&str) -> i64
{
    let mut cpu = Cpu{ip:0,opcodes:vec![],inputs:vec![].into(),outputs:vec![]};
    cpu.opcodes = inp.split(",").map(|s| s.parse::<i32>().unwrap()).collect();

    let mut phase_settings:Vec<i32> = vec![0,1,2,3,4];
    let mut _max_settings = vec![];
    let mut maxs = 0;
    loop {
        let mut cpus:Vec<Cpu> = vec![];
        for c in 0..5 {
            let mut cp = cpu.clone();
//            cp.opcodes = cpu.opcodes.clone();
            cp.inputs.push_back(phase_settings[c]);
            cpus.push(cp);
        }
        let mut s = 0;
        for c in 0..5 {
            cpus[c].inputs.push_back(s);
            s = cpus[c].run();
        }
        if s > maxs {
            maxs = s;
            _max_settings = phase_settings.clone();
        }
        if permute(&mut phase_settings) == false {break;}
    }

    //println!("Max phase {maxs} from settings {:?}", max_settings);
    return maxs as i64;
}

fn process2(inp:&str) -> i64
{
    let mut cpu = Cpu{ip:0,opcodes:vec![],inputs:vec![].into(),outputs:vec![]};
    cpu.opcodes = inp.split(",").map(|s| s.parse::<i32>().unwrap()).collect();

    let mut phase_settings:Vec<i32> = vec![5,6,7,8,9];
    let mut _max_settings = vec![];
    let mut maxs = 0;
    loop {
        let mut cpus:Vec<Cpu> = vec![];
        for c in 0..5 {
            let mut cp = cpu.clone();
//            cp.opcodes = cpu.opcodes.clone();
            cp.inputs.push_back(phase_settings[c]);
            cpus.push(cp);
        }
        let mut s = 0;
        loop {
            for c in 0..5 {
                cpus[c].inputs.push_back(s);
                s = cpus[c].run();
            }
            if cpus[4].opcodes[cpus[4].ip] == 99 {break;}
        }
            
        if s > maxs {
            maxs = s;
            _max_settings = phase_settings.clone();
        }
        if permute(&mut phase_settings) == false {break;}
    }

    //println!("Max phase {maxs} from settings {:?}", max_settings);
    return maxs as i64;
}

fn main() {
    test_split();
//    assert!(process("3,3,1108,-1,8,3,4,3,99",vec![8]) == 1);
//    assert!(process("3,3,1108,-1,8,3,4,3,99",vec![7]) == 0);
//    assert!(process("3,3,1107,-1,8,3,4,3,99",vec![7]) == 1);

//    let bench_result = run_benchmark(10, |_| {_bench_permute();}); bench_result.print_stats();

    //panic!("Stop now");

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { 
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