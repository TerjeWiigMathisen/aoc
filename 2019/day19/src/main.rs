use std::collections::VecDeque;
use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use priority_queue::PriorityQueue;
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

fn show(grid:&HashMap<String,i32>)
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
    let blocks = ['#',' ','O']; // Intcode return values
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            let key = format!("{x},{y}");
            let mut b = 0;
            if grid.contains_key(&key) {
                b = *grid.get(&key).unwrap();
                if b > 2 {b = 1;}
            }
            print!("{}", blocks[b as usize]);
        }
        println!();
    }
}

fn discover(cpu:&mut Cpu, grid:&mut HashMap<String,i32>, x:i32, y:i32, t:i32)->(i32,i32)
{
    let key = format!("{x},{y}").to_owned();
    let mut oxy = (0,0);
    grid.insert(key, t);
//    show(&grid);
    if t == 2 {oxy = (x,y);}
    // Try north, south, east, west:
    for (dx,dy, dir) in [(0,-1,1),(0,1,2),(1,0,3),(-1,0,4)] {
        let nkey = format!("{},{}",x+dx,y+dy).to_owned();
        if !grid.contains_key(&nkey) {
            
            cpu.inputs.push_back(dir);
            cpu.run();
            let r = cpu.outputs.pop().unwrap();
            if r == 0 {
                grid.insert(nkey, 0);
                continue;
            }

            let o = discover(cpu, grid, x+dx,y+dy, r as i32);
            if o != (0,0) {
                oxy = o;
            }
            // Move back!
            cpu.inputs.push_back(((dir-1)^1)+1);
            cpu.run();
            cpu.outputs.pop().unwrap();
        }
    }
    oxy
}
    
fn process(inp:String) -> i64
{
    let mut cpu = Cpu{ip:0,opcodes:vec![], inputs:vec![].into(), outputs:vec![], relative_base:0};

    let tract:[char;2] = ['.','#'];
    let mut p1 = 0;
    for y in 0..50 {
        for x in 0..50 {
            cpu.opcodes = inp.split(",").map(|s| s.parse::<i64>().unwrap()).collect();
            cpu.inputs.push_back(x);
            cpu.inputs.push_back(y);
            cpu.ip = 0;
            cpu.run();
            let res = cpu.outputs.pop().expect("Bad result");
//            print!("{}",tract[res as usize]);
            p1 += res;
        }
//        println!("");
    }
    p1
}

fn process2(inp:String) -> i64
{
    let mut cpu = Cpu{ip:0,opcodes:vec![], inputs:vec![].into(), outputs:vec![], relative_base:0};

    let mut p2 = 0;
    let mut left = 4;
    let mut right = 5;
    let mut start:Vec<usize> = vec![0;1000];
    let mut end:Vec<usize> = vec![0;1000];
    for y in 3..1000 {
        let mut line:Vec<u8> = vec![0;3000];
        let mut x = left;
        let mut first_edge = true;
        while x < line.len() {
            cpu.opcodes = inp.split(",").map(|s| s.parse::<i64>().unwrap()).collect();
            cpu.inputs.push_back(x as i64);
            cpu.inputs.push_back(y as i64);
            cpu.ip = 0;
            cpu.run();
            let res = cpu.outputs.pop().expect("Bad result");
//            print!("{}",tract[res as usize]);
            p2 += res;
            line[x] = res as u8;
            if first_edge {
                if res == 1 && line[x-1] == 0 {
//                    print!("  {y}:({x}");
                    left = x;
                    start[y] = x;
                    first_edge = false;
                    while x < right-1 {
                        line[x] = 1;
                        x += 1;
                    }
                }
            }
            else {
                if res == 0 {
//                    print!("-{x})");
                    right = x;
                    end[y] = x;
                    break;
                }
            }
            x += 1;
        }
//        println!("");
    }
    for y in 200..1000 {
        let down_left = start[y];
        let top_right = end[y-99];
        if top_right >= down_left + 100 {
            let top_left_x = down_left;
            let top_left_y = y-99;
            p2 = (top_left_x * 10000 + top_left_y) as i64;
//            println!("top left = ({top_left_x},{top_left_y}) p2={p2}");
            break;
        }
    }
    p2
}


fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    //let bench_result = run_benchmark(10, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let part1 = process(input.clone());
    let part2 = process2(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} ms",devtime.time_in_millis().unwrap());
}