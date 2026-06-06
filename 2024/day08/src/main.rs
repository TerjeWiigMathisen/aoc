// fastest version without HashSet runs in 8.4 us

//use std::collections::VecDeque;
use std::collections::HashSet;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

#[derive(Debug,PartialEq,Eq,Hash,Clone)]
struct Point {
    x:i32,
    y:i32,
}

fn _gcd(a:i32, b:i32) -> i32 {
    let mut a = a.abs();
    let mut b = b.abs();
    if a < b { (a,b) = (b,a); }
    while b != 0 {
        (a,b) = (b,a%b);
    }
    a
}

fn _process(inp:String) -> (i32, i32)
{
    let mut antennas:Vec<Vec<Point>> = vec![Vec::<Point>::new();80];
    let mut anti:HashSet<Point> = HashSet::new();
    let mut anti2:HashSet<Point> = HashSet::new();

    let mut width:usize = 0;
    let mut height:usize = 0;

    for line in inp.lines() {
        let lineb = line.as_bytes();
        width = lineb.len();

        for i in 0..width {
            let c = lineb[i];
            if c != b'.' {
                let index = (c - b'0') as usize;
                let p = Point{x:i as i32, y:height as i32};
                antennas[index].push(p);
            }
        }
        height += 1;

    }
    for ant in &antennas {
        if ant.len() == 0 {continue;}
        for j in 1..ant.len() {
            let p2 = &ant[j];
            for i in 0..j {
                let p = &ant[i];
                let mut dx = p2.x - p.x;
                let mut dy = p2.y - p.y;
                let mut x = p2.x + dx;
                let mut y = p2.y + dy;
                if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    let p3 = Point{x:x, y:y};
                    anti.insert(p3);
                }
                x = p.x - dx;
                y = p.y - dy;
                if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    let p3 = Point{x:x, y:y};
                    anti.insert(p3);
                }
                // part2
                let gcd = _gcd(dx, dy);
                if gcd > 1 {
                    dx = dx / gcd;
                    dy = dy / gcd;
                }
                let mut x = p.x + dx;
                let mut y = p.y + dy;
                while x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    let p3 = Point{x:x, y:y};
                    anti2.insert(p3);
                    x += dx;
                    y += dy;
                }
                (x,y) = (p.x, p.y);
                while x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    let p3 = Point{x:x, y:y};
                    anti2.insert(p3);
                    x -= dx;
                    y -= dy;
                }
            }
        }        
    }
    let part1 = anti.len() as i32;
    let part2 = anti2.len() as i32;
    (part1, part2)
}

fn process_noset(inp:String) -> (i32, i32)
{
    let mut antennas:Vec<Vec<Point>> = vec![Vec::<Point>::new();(b'z'-b'0'+1) as usize];

    let mut width:usize = 0;
    let mut height:usize = 0;

    for line in inp.lines() {
        let lineb = line.as_bytes();
        width = lineb.len();

        for i in 0..width {
            let c = lineb[i];
            if c != b'.' {
                let index = (c - b'0') as usize;
                let p = Point{x:i as i32, y:height as i32};
                antennas[index].push(p);
            }
        }
        height += 1;
    }

    let mut anti:Vec<u8> = vec![0; width*height];

    let mut part1 = 0;
    let mut part2 = 0;
    for ant in &antennas {
        if ant.len() == 0 {continue;}
        for j in 1..ant.len() {
            let p2 = &ant[j];
            for i in 0..j {
                let p = &ant[i];
                let dx = p2.x - p.x;
                let dy = p2.y - p.y;
                let mut x = p2.x + dx;
                let mut y = p2.y + dy;
                if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    anti[(y as usize)*width + x as usize] |= 1;
                }
                x = p.x - dx;
                y = p.y - dy;
                if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    anti[(y as usize)*width + x as usize] |= 1;
                }
                let mut x = p.x + dx;
                let mut y = p.y + dy;
                while x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    anti[(y as usize)*width + x as usize] |= 2;
                    x += dx;
                    y += dy;
                }
                (x,y) = (p.x, p.y);
                while x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    anti[(y as usize)*width + x as usize] |= 2;
                    x -= dx;
                    y -= dy;
                }
            }
        }
    }
    for i in 0..anti.len() {
        let a = anti[i] as i32;
        part1 += a & 1;
        part2 += a;
    }
    part2 = (part2-part1)>>1;      
    (part1, part2)
}

pub fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process_noset(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process_noset(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}