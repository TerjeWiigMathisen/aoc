//use std::collections::VecDeque;
use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

#[derive(Clone,Debug)]
struct World {
    xsize:usize,
    ysize:usize,
    grid:Vec<u8>,
}

impl World {
    fn xy2pos(&self, x:usize, y:usize)->usize
    {
        self.xsize * y + x
    }
    fn pos2xy(&self, pos:usize)->(usize,usize)
    {
        (pos % self.xsize, pos / self.xsize)
    }
}

fn parse(inp:&str)->World
{
    let lines:Vec<&str> = inp.split("\n").collect();
    let mut w = World{xsize:lines[0].as_bytes().len(), ysize:lines.len(),grid:vec![]};
    for l in lines {
        w.grid.extend(l.as_bytes().iter());
    }
    w
}

fn reduce(dx:i32, dy:i32)->(i32,i32)
{
    let mut udx = dx.abs();
    let mut udy = dy.abs();
    for n in 2..40 {
        while udx % n == 0 && udy % n == 0 {
            udx = udx / n;
            udy = udy / n;
        }
    }
    (dx.signum()*udx, dy.signum()*udy)
}

fn direction(dx:i32,dy:i32)->f64
{
    let (udx,udy) = (dx.abs(),dy.abs());
    let mut a:f64 = 0.0;
    if udy >= udx {
        a = udx as f64 / udy as f64 * 0.125;
    }
    else {
        a = 0.25 - udy as f64 / udx as f64 * 0.125;
    }
    if dy < 0 {
        if dx < 0 { // Bottom left quadrant
            a = a + 0.5
        }
        else { // Bottom right
            a = 0.5 - a;
        }
    }
    else if dx < 0 { // Top left
        a = 1.0 - a;
    }
    a
}

fn direction_fixed(dx:i32, dy:i32)->u64
{
    let a = direction(dx, dy);
    return (a * 65536.0 * 65536.0 * 65536.0 * 65536.0) as u64;
}

fn process(inp:&str) -> String
{
    let w = parse(inp);
    let len = w.grid.len();
    let mut max_visible = 0;
    let mut max_pos:String = "0,0".to_owned();
    for m in 0..len {
        if w.grid[m] == b'#' {
            let (x0,y0) = w.pos2xy(m);
            let mut visible:HashMap<String,u32> = HashMap::new();
            for n in 0..len {
                if n != m && w.grid[n] == b'#' {
                    let (x1,y1) = w.pos2xy(n);
                    let (dx,dy) = reduce(x0 as i32 - x1 as i32, y0 as i32 - y1 as i32);
                    let key = format!("{dx},{dy}");
                    *visible.entry(key).or_insert(0) += 1;
                }
            }
            let total_visible_from_here = visible.len();
            if total_visible_from_here > max_visible {
                max_visible = total_visible_from_here;
                max_pos = format!("{x0},{y0}");
//                println!("Can see {max_visible} stations from {max_pos}");
//                for k in visible.keys() {
//                    println!("{k} -> {}",visible.get(k).unwrap());
//                }
            }
        }
    }
    return max_pos;
}

fn process2(inp:&str, x0:usize, y0:usize) -> String
{
    let w = parse(inp);
    let len = w.grid.len();
    let mut directions:HashMap<String,Vec<String>> = HashMap::new();
    for n in 0..len {
        let (x1,y1) = w.pos2xy(n);
        if (x1 != x0 || y1 != y0) && w.grid[n] == b'#' {
            let (dx,dy) = (x1 as i32 - x0 as i32, y0 as i32 - y1 as i32);
            let manh = dx.abs()+dy.abs();
            let dir = direction(dx,dy);
            let key = format!("{:10.8}",dir);
            let payload = format!("{manh:4},{x1},{y1}");
//            println!("{x1},{y1} -> {dx},{dy} -> {key} {payload}");
            let mut points: Vec<String> = vec![];
            if directions.contains_key(&key) {points = directions.get(&key).unwrap().to_vec()};

            points.push(payload);
            points.sort();
            points.reverse();
            directions.insert(key, points);
        }
    }
    let mut keys:Vec<String> = vec![];
    for (k,v) in directions.iter() {
        let key = k.clone();
        keys.push(key);
    }
    keys.sort();
    let mut k = 0;
    let mut i = 0;
    loop {
        let key = keys[i].clone();
        i += 1;
        if i >= keys.len() { i = 0;}

        let mut points:Vec<String> = directions.get(&key).unwrap().to_vec();
        let l = points.len();
        if l == 0 {continue}
    
        k += 1;
        let p = points.pop().unwrap();
//        println!("{k:3} {key} -> {} (+{})", p, l-1);
        if k == 200 {return p;}
        directions.insert(key, points.to_owned());
    }
    return "".to_string();
}

fn main() {

assert!(process("......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####") == "5,8");
    
assert!(process("#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.") == "1,2");

assert!(process(".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..") == "6,3");

assert!(process(".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##") == "11,13".to_owned());
//    let bench_result = run_benchmark(10, |_| {bench_permute();}); bench_result.print_stats();

    //panic!("Stop now");

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1, |_| { process(&input); }); bench_result.print_stats();

    devtime.start();
    let part1 = process(&input);
    let part2 = process2(&input,30,34);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}