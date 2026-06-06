// 67.5 ms

use std::env::args;
use once_cell::sync::Lazy; // 1.3.1
use std::fs;
use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

struct Point {
    x:i32,
    y:i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {x:x, y:y}
    }
}

static LR: Lazy<Point> = Lazy::new(|| Point::new(101, 103));

struct Robot {
    x:i32,
    y:i32,
    vx:i32,
    vy:i32,
}
impl Robot {
    /*
    fn new(x: i32, y: i32, vx: i32, vy: i32) -> Robot {
        Robot {x:x, y:y, vx:vx, vy:vy}
    }
    */

    fn newpos(&mut self, iter:i32) -> (i32, i32)
    {
        let x = (self.x + self.vx * iter) % LR.x;
        let y = (self.y + self.vy * iter) % LR.y;
        (x,y)
    }
    fn nextpos(&mut self) -> (i32, i32)
    {
        self.x += self.vx;
        if self.x >= LR.x {self.x -= LR.x;}
        self.y += self.vy;
        if self.y >= LR.y {self.y -= LR.y;}
        (self.x,self.y)
    }
}

fn process2(inp:String) -> (i32, i32)
{
    let mut robots:Vec<Robot> = Vec::new();

    let lp = parser!("p=" x:i32 "," y:i32 " v=" vx:i32 "," vy:i32 => Robot {x, y, vx, vy });
    for line in inp.lines() {
        let mut r = lp.parse(line).unwrap();
        /* Make all delta values positive, this simplifies and speeds up the % operations */
        if r.vx < 0 {r.vx += LR.x;}
        if r.vy < 0 {r.vy += LR.y;}
        robots.push(r);
    }
    // part1
    let mut quad:Vec<i32> = vec![0, 0, 0, 0];
    let x2 = LR.x/2;
    let y2 = LR.y/2;
    for r in &mut robots {
        let (x, y) = r.newpos(100);
        let xq = if x < x2 {0} else if x > x2 {1} else {continue};
        let yq = if y < y2 {0} else if y > y2 {2} else {continue};
        let idx = (xq + yq) as usize;
        quad[idx] += 1;
    }
    let part1 = quad.iter().product();

    // part2
    let mut part2 = 0;
    let mut max = 0;
    let size = (LR.x+2 + ((LR.y+2)*111)) as usize;
    for i in 1..=10403 { // 101*103, the pattern repeats after this point
        let mut rb:Vec<u8> = vec![0; size]; // With guard cells
        //generation(&mut robots); // Move all the robots
        for r in &mut robots {
            let (x,y) = r.nextpos();
            let o = ((x+1) + (y+1)*111) as usize;
            rb[o] += 4;
            rb[o-1] += 1;
            rb[o+1] += 1;
            rb[o-111] += 1;
            rb[o+111] += 1;
        }
        let mut neighbors = 0; //:[i32;20] = [0; 20];
        for n in rb {
            if n < 6 { continue; }
            neighbors += n as i32;
        }
        let many = neighbors; // = neighbors[8]*8 + neighbors[7]*7 + neighbors[6]*6;
        if many > max {
            part2 = i;
            max = many;
        }
    }

    (part1, part2)
}

fn process_collide(inp:String) -> (i32, i32)
{
    let mut robots:Vec<Robot> = Vec::new();

    let lp = parser!("p=" x:i32 "," y:i32 " v=" vx:i32 "," vy:i32 => Robot {x, y, vx, vy });
    for line in inp.lines() {
        let mut r = lp.parse(line).unwrap();
        /* Make all delta values positive, this simplifies and speeds up the % operations */
        if r.vx < 0 {r.vx += LR.x;}
        if r.vy < 0 {r.vy += LR.y;}
        robots.push(r);
    }
    // part1
    let mut quad:Vec<i32> = vec![0, 0, 0, 0];
    let x2 = LR.x/2;
    let y2 = LR.y/2;
    for r in &mut robots {
        let (x, y) = r.newpos(100);
        let xq = if x < x2 {0} else if x > x2 {1} else {continue};
        let yq = if y < y2 {0} else if y > y2 {2} else {continue};
        let idx = (xq + yq) as usize;
        quad[idx] += 1;
    }
    let part1 = quad.iter().product();

    // part2
    let mut part2 = 0;
    let size = (LR.x+2 + ((LR.y+2)*111)) as usize;
    for i in 1..=10403 { // 101*103, the pattern repeats after this point
        let mut rb:Vec<u8> = vec![0; size]; // With guard cells
        let mut collide = false;
        for r in &mut robots {
            let (x,y) = r.newpos(i);
            let o = ((x+1) + (y+1)*111) as usize;
            if rb[o] > 0 {collide = true; break;}
            rb[o] += 1;
        }
        if !collide {
            part2 = i;
            break;
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

    let bench_result = run_benchmark(100, |_| { process_collide(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process_collide(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}