// Fastest run 19573700 ns

//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;
//use rustc_hash::FxHashMap;
//use std::f64;

//const EXPANDBYTE:u64 = (1 + 8 + 64 + 512 + 4096 + 32768 + 262144 + 2097152) as u64;
//const EXPANDNYBBLE:[u64] = [0,1,8,9,64,65,72,73,512,513,520,521,576,577,584,585];


fn _bitinterleave(x:i64, y:i64, z:i64) -> i64 {
    let mut res = 0;
    let mut obit = 0;
    for ibit in 0..21 {
        res += ((x >> ibit) & 1) << obit; obit += 1;
        res += ((y >> ibit) & 1) << obit; obit += 1;
        res += ((z >> ibit) & 1) << obit; obit += 1;
    }
    res
}


use std::arch::asm;
//use core::arch::x86_64;

#[target_feature(enable = "bmi2")]
fn asm_pdep64(src:u64, mask:u64) -> u64 {
    // SAFETY
    unsafe {
        let dst:u64;
        asm!(
            "pdep {}, {}, {}",
            out(reg) dst,
            in(reg) src,
            in(reg) mask,
        );
        dst
    }
}

//use core::arch::x86_64;
//use crate::x86_64::_pdep_u64;

//unsafe fn my_pdep64(src:u64, mask:u64) -> u64 {
//    let r = _pdep_u64(src, mask);
//    r
//}

const FIRST_PDEP:u64  = 0o1111111111111111111111;
const SECOND_PDEP:u64 = 0o222222222222222222222;
const THIRD_PDEP:u64  = 0o444444444444444444444;

fn bitinterleave_bmi2(x:u64, y:u64, z:u64) -> u64 {
    unsafe {
        let res = asm_pdep64(x, FIRST_PDEP) + 
                    asm_pdep64(y, SECOND_PDEP) + 
                    asm_pdep64(z, THIRD_PDEP);
        return res;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3D {
    fn new(x: i64, y: i64, z: i64) -> Point3D {
        Point3D { x, y, z }
    }
    fn dist2(&self, other: &Point3D) -> i64 {
        let (dx,dy,dz) = (self.x-other.x, self.y-other.y, self.z-other.z);
        dx*dx + dy*dy + dz*dz
    }
}

struct Circuit {  
    len: u16, 
    parentid: u16,
    next: u16,
}

impl Circuit {
    fn new(id: u16) -> Circuit {
        Circuit { len:1, parentid: id, next: 0}
    }
}

struct Circuits {
    circs: Vec<Circuit>,   
}
impl Circuits {
    fn new() -> Circuits {
        Circuits { circs: Vec::new() }
    }
    fn merge(&mut self, t: u16, o:u16) -> u16 {
        let this = self.circs[t as usize].parentid as usize;
        let other = self.circs[o as usize].parentid as usize;
        if this == other {
            return self.circs[this].len;
        }
        if this > other {
            return self.merge(other as u16,this as u16);
        }
        self.circs[this].len += self.circs[other].len;
        self.circs[other].len = 0;

        let parent = self.circs[this].parentid;  // Always equal to this!
        let mut curr = other;
        let topnext = self.circs[this].next;
        self.circs[this].next = other as u16;
        loop {
            self.circs[curr].parentid = parent;
            self.circs[curr].len = 0;
            let next = self.circs[curr].next as usize;
            if next == 0 {
                self.circs[curr].next = topnext;
                break;
            }
            curr = next;
        }
        self.circs[this].len
    }

    fn _dump_circuits(&self, sizelimit:u16) {
        for i in 1..self.circs.len() {
            if self.circs[i].len >= sizelimit {
                println!("Circuit {i}, size: {}, parent: {}", self.circs[i].len, self.circs[i].parentid);
                let mut curr = self.circs[i].next as usize;
                while curr != 0 {
                    print!(" {curr}");
                    curr = self.circs[curr].next as usize;
                }
                println!("");
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
struct Distance {
    dist2: i64,
    id: u16,
    next: u16,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
struct Vert {
    inter:u64,
    index:usize,
}

fn processdist(inp:String) -> (i64, i64)
{
    let mut part1:i64 = 0;
    let mut part2:i64 = 0;
    let mut junctions:Vec<Point3D> = Vec::new();
    junctions.push(Point3D::new(0,0,0)); // Dummy bottom element for index 0

    let mut v:Vec<Vert> = Vec::new();

    let mut circs = Circuits::new();
    circs.circs.push(Circuit { len:0, parentid: 0, next: 0} );

    for line in inp.lines() {
        let coords = line.split(",").collect::<Vec<&str>>();
        let p1 = Point3D::new(coords[0].parse().unwrap(), coords[1].parse().unwrap(), coords[2].parse().unwrap());
            v.push(Vert { inter:bitinterleave_bmi2(p1.x as u64, p1.y as u64, p1.z as u64),
                index:junctions.len() });
        circs.circs.push(Circuit::new(junctions.len() as u16));
        junctions.push(p1);
    }
    
    v.sort();
    let mut distances:Vec<Distance> = Vec::with_capacity(6000);

    const LIMIT:i64 = 40000;

    for i in 0..v.len()-1 {
        let a = v[i].index;
        for j in i+1..v.len() {
            let b = v[j].index;
            let d2 = junctions[a].dist2(&junctions[b]);
            if d2 > LIMIT*LIMIT { break; }
            distances.push( Distance {dist2:d2, id:a as u16, next:b as u16 } );
        }
    }
    distances.sort();
    println!("{} distances, range: {} to {}", distances.len(), 
        f64::sqrt(distances[0].dist2 as f64), f64::sqrt(distances[distances.len()-1].dist2 as f64));
    println!("dist(1000) = {}", f64::sqrt(distances[1000].dist2 as f64));

    // part1
    let part1cnt = if junctions.len() > 100 { 1000 } else { 10 };
    for i in 0..part1cnt {
        let d = distances[i];
        circs.merge(d.id, d.next);
    }
    let mut circlen:Vec<u16> = Vec::new();
    for i in 0..junctions.len() {
        if circs.circs[i].len > 0 {
            circlen.push(circs.circs[i].len);
        }
    }
    circlen.sort();
    circlen.reverse();
    let part1 = circlen[0] as i64 * circlen[1] as i64 * circlen[2] as i64;
    for i in part1cnt.. {
        let d = distances[i];
        let clen = circs.merge(d.id, d.next);
        if clen+1 >= junctions.len() as u16 {
            part2 = junctions[d.id as usize].x * junctions[d.next as usize].x;
            break;
        }
    }
   
    (part1, part2)
}

fn process(inp:String) -> (i64, i64)
{
    //let mut part1:i64 = 0;
    let mut part2:i64 = 0;
    let mut junctions:Vec<Point3D> = Vec::new();
    junctions.push(Point3D::new(0,0,0)); // Dummy bottom element for index 0
    let mut circs = Circuits::new();
    circs.circs.push(Circuit { len:0, parentid: 0, next: 0} );

    for line in inp.lines() {
        let coords = line.split(",").collect::<Vec<&str>>();
        let p1 = Point3D::new(coords[0].parse().unwrap(), coords[1].parse().unwrap(), coords[2].parse().unwrap());
        circs.circs.push(Circuit::new(junctions.len() as u16));
        junctions.push(p1);
    }
    //println!("{} junctions and {} circuits", junctions.len(), circs.circs.len());
    let mut distances:Vec<Distance> = Vec::with_capacity((junctions.len()-2)*(junctions.len()-1) >> 1);
    let mut pi:Point3D;
    for i in 1..junctions.len()-1 {
        pi = junctions[i];
        for j in i+1..junctions.len() {
            distances.push( Distance { dist2:pi.dist2(&junctions[j]), id:i as u16, next:j as u16 } );
        }
    }
    distances.sort();
    println!("{} distances, range: {} to {}", distances.len(), 
        f64::sqrt(distances[0].dist2 as f64), f64::sqrt(distances[distances.len()-1].dist2 as f64));
    println!("dist(1000) = {}", f64::sqrt(distances[1000].dist2 as f64));
    println!("dist(5000) = {}", f64::sqrt(distances[5000].dist2 as f64));

    // part1
    let part1cnt = if junctions.len() > 100 { 1000 } else { 10 };
    for i in 0..part1cnt {
        let d = distances[i];
        // if i+2 >= part1cnt {
        //     println!("{},{}  {:?} {:?}", d.id-1, d.next-1, junctions[d.id as usize], junctions[d.next as usize]);
        //     circs.merge(d.id, d.next);
        //     circs.dump_circuits(7);
        // }
        // else {
        circs.merge(d.id, d.next);
        //}
    }
    let mut circlen:Vec<u16> = Vec::new();
    for i in 0..junctions.len() {
        if circs.circs[i].len > 0 {
            circlen.push(circs.circs[i].len);
        }
    }
    circlen.sort();
    circlen.reverse();
    // let cl = 10; //if circlen.len() > 10 { 10 } else { circlen.len() };
    // for i in 0..cl {
    //     print!("{} ", circlen[i]);
    // }
    // println!("");
    let part1 = circlen[0] as i64 * circlen[1] as i64 * circlen[2] as i64;
    for i in part1cnt.. {
        let d = distances[i];
        let clen = circs.merge(d.id, d.next);
        if clen+1 >= junctions.len() as u16 {
            println!("Minimum distance: {}", f64::sqrt(distances[0].dist2 as f64));
            println!("Single network after {i} links, distance {}", f64::sqrt(d.dist2 as f64));
            println!("Maximum distance: {}", f64::sqrt(distances[distances.len()-1].dist2 as f64));
            part2 = junctions[d.id as usize].x * junctions[d.next as usize].x;
            break;
        }
    }
   
    (part1, part2)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut fname = "input.txt".to_string();
    if args.len() > 1 { fname = args[1].clone(); }

    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

//    let (part1, part2) = process(input.clone());
//    let (p1, p2) = processdist(input.clone());
//    println!("interleave: {}, {}", p1, p2 as f64);

//    let bench_result = run_benchmark(100, |_| { process(input.clone()); }); bench_result.print_stats();

    let mut devtime = DevTime::new_simple();
    process(input.clone());
    devtime.start();
    let (part1, part2) = processdist(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}