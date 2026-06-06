//use std::collections::VecDeque;
//use std::collections::HashMap;
//use hashbrown::hash_map::DefaultHashBuilder;
use hashbrown::DefaultHashBuilder;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;
use std::env::args;

struct Map {
    width:i32,
    height:i32,
    stride:i32,
    map:Vec<u8>,
}

impl Map {
    fn new(width:i32, height:i32, inp:String) -> Map {
        Map {
            width: width,
            height: height,
            stride: width+1,
            map: inp.as_bytes().to_vec(),
        }
    }

    fn get(&self, x:i32, y:i32) -> u8 {
        self.map[(y*self.stride + x) as usize]
    }

    fn set(&mut self, x:i32, y:i32, val:u8) {
        self.map[(y*self.stride + x) as usize] = val;
    }

    fn getp(&self, pos:i32) -> u8 {
        self.map[pos as usize]
    }

    fn setp(&mut self, pos:i32, val:u8) {
        self.map[pos as usize] = val;
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get(x, y));
            }
            println!();
        }
    }
}

fn pack(pos:i32, dir:u8, cost:i32) -> u64 {
    ((pos as u64) | (dir as u64) << 16 | (cost as u64) << 24)
}
fn unpack(p:u64) -> (i32, u8, i32) {
    let pos = (p & 0xffff) as i32;
    let dir = ((p >> 16) & 0x3) as u8;
    let cost = (p >> 24) as i32;
    (pos, dir, cost)
}

fn process(inp:String) -> (i32, i32)
{
    let mut part1 = 0;
    let mut part2 = 0;
    let w = inp.find('\n').unwrap() as i32;
    let s = w+1;
    let h = inp.len() as i32 / s;
    let m = Map::new(w, h, inp);
    let start = m.map.iter().position(|&x| x == b'S').unwrap() as i32;
    //println!("start={},{}", start % s, start / s);
    let end = m.map.iter().position(|&x| x == b'E').unwrap() as i32;
    //println!("end={},{}", end % s, end / s);

    let step = [1, m.stride, -1, -m.stride];

    //let mut pq: PriorityQueue<u64,u32> = PriorityQueue::new();
    let mut pq = PriorityQueue::<_, _, DefaultHashBuilder>::with_default_hasher();
    let mut seen:Vec<i32> = vec![i32::MAX; m.map.len()*2];
    pq.push(pack(start, 0, 0), Reverse(0));
    while !pq.is_empty() {
        let (p, _) = pq.pop().unwrap();
        let (pos, dir, cost) = unpack(p);
        let x = pos % m.stride;
        let y = pos / m.stride;
        //println!("pos={} (x={x},y={y}) dir={} cost={}", pos, dir, cost);

        let k = pos as usize * 2 + (dir as usize & 1);
        if seen[k] <= cost {continue;}
        seen[k] = cost;
        if pos == end {
            part1 = cost as i32;
            break;
        }
        let f = pos+step[dir as usize];
        let c = cost+1;
        if m.getp(f) != b'#' {
            pq.push(pack(f, dir, c), Reverse(c));
        }
        let d = (dir+3)&3;
        let l = pos+step[d as usize];
        let c = cost+1001;
        if m.getp(l) != b'#' {
            let k = pos as usize * 2 + (d as usize & 1);
            if seen[k] >= c-1 {
                seen[k] = c-1;
                pq.push(pack(l, d, c), Reverse(c));
            }
        }
        let d = (dir+1)&3;
        let r = pos+step[d as usize];
        if m.getp(r) != b'#' {
            let k = pos as usize * 2 + (d as usize & 1);
            if seen[k] >= c-1 {
                seen[k] = c-1;
                pq.push(pack(r, d, c), Reverse(c));
            }
        }
    }

    let mut pq = PriorityQueue::<_, _, DefaultHashBuilder>::with_default_hasher();
    let mut bseen:Vec<u8> = vec![0; m.map.len()*2];
    bseen[end as usize * 2] = 1;
    pq.push(pack(end-1, 0, part1-1), part1-1);
    pq.push(pack(end-s, 1, part1-1), part1-1);
    pq.push(pack(end+1, 2, part1-1), part1-1);
    pq.push(pack(end+s, 3, part1-1), part1-1);
    while !pq.is_empty() {
        let (p, _) = pq.pop().unwrap();
        let (pos, dir, cost) = unpack(p);

        if m.getp(pos) == b'#' {continue;}
        if cost < 0 {continue;}
        let k = pos as usize * 2 + (dir as usize & 1);
        if seen[k] != cost {continue;}
        if bseen[k] != 0 {continue;}
        bseen[k] = 1;
        if pos == start && dir == 0 {continue;}

        let p = pos-step[dir as usize];
        let k = p as usize * 2 + (dir as usize & 1);
        let c = cost-1;
        if seen[k] == c {
            pq.push(pack(p, dir, c), c);
        }
        let d = (dir+1)&3;
        let c = cost-1001;
        let p = pos-step[d as usize];
        let k = p as usize * 2 + (d as usize & 1);
        if seen[k] == c {
            pq.push(pack(p, d, c), c);
        }
        let d = (dir+3)&3;
        let p = pos-step[d as usize];
        let k = p as usize * 2 + (d as usize & 1);
        if seen[k] == c {
            pq.push(pack(p, d, c), c);
        }     
    }
    for i in 0..m.map.len() {
        if bseen[i*2]+bseen[i*2+1] != 0 {
            part2 += 1;
        }
    }

    (part1, part2)
}

fn main() {
    let args = args().collect::<Vec<String>>();
    let fname = if args.len() > 1 {&args[1]} else {"input.txt"}; // instead of args[1]
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