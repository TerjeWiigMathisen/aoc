// Acer 248 us

use std::collections::VecDeque;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

const STRIDE:usize = 72;
const TOP_LT:usize = STRIDE+1;
const BOT_RT:usize = TOP_LT + STRIDE*70 + 70;
const SIZE:usize = BOT_RT + STRIDE + 1;

fn xy2o(x:usize, y:usize) -> usize
{
    (y+1)*STRIDE + x + 1
}

fn _o2xy(o:usize) -> (usize, usize)
{
    (((o-1) % STRIDE), ((o-1) / STRIDE) - 1)
}

fn _show(space:&Vec<u16>)
{
    let mut l = 0;
    for i in 0..space.len() {
        print!("{}", if space[i] == 0 {'#'} else {'.'});
        l += 1;
        if l == STRIDE {
            l = 0;
            println!();
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct PosSteps {
    pos:u16,
    steps:u16,
}

fn bfs(sp:&Vec<u16>) -> usize
{
    let mut q:VecDeque<PosSteps> = VecDeque::new();
    let mut space = sp.clone();
    q.push_back(PosSteps {pos:TOP_LT as u16, steps:0} );

    while let Some(p) = q.pop_front() {
        let ps = p.pos as usize;
        let steps = p.steps;
        if space[ps] <= steps {continue; }
//        let (x,y) = o2xy(ps);
//        let o = _xy2o(x,y);
//        println!("{:?} ({x},{y} -> {o})", p);
//        assert!(x+y <= steps as usize, "{x},{y} - >{steps}");
//        assert!(o == ps);
        if ps == BOT_RT { return steps as usize; }

        space[ps] = p.steps;
        let steps = p.steps + 1;
        q.push_back(PosSteps{pos:(ps-1) as u16, steps:steps});
        q.push_back(PosSteps{pos:(ps-STRIDE) as u16, steps:steps});
        q.push_back(PosSteps{pos:(ps+1) as u16, steps:steps});
        q.push_back(PosSteps{pos:(ps+STRIDE) as u16, steps:steps});
    }
    usize::MAX
}

fn process(inp:&str) -> (usize, String)
{
    let mut space:Vec<u16> = vec![u16::MAX;SIZE];
    for i in 0..STRIDE { space[i] = 0; }
    for i in 1..=72 {space[STRIDE*i] = 0; }
    for i in BOT_RT+1..SIZE { space[i] = 0; }

//    show(&space);

    let input = inp.as_bytes();
    let mut pairs = 0;
    let mut i = 0;
    let mut part1 = 0;
    let mut newbytes:Vec<u8> = Vec::with_capacity(8000);
    while i < input.len() {
        let mut x = 0;
        while input[i] != b',' { x = x*10 + (input[i]-b'0') as usize; i += 1; }
        i += 1; // skip ','
        let mut y = 0;
        while input[i] != b'\n' { y = y*10 + (input[i]-b'0') as usize; i += 1; }
        i += 1; // skip '\n'
//        println!("byte at ({x},{y}) -> {}", xy2o(x,y));
        space[xy2o(x,y)] = 0;
        newbytes.push(x as u8);
        newbytes.push(y as u8);
        pairs += 1;
        if pairs == 1024 {
//            show(&space);
            part1 = bfs(&space);
            break;
        }
    }
    while i < input.len() {
        let mut x = 0;
        while input[i] != b',' { x = x*10 + (input[i]-b'0') as usize; i += 1; }
        i += 1; // skip ','
        let mut y = 0;
        while input[i] != b'\n' { y = y*10 + (input[i]-b'0') as usize; i += 1; }
        i += 1; // skip '\n'
        newbytes.push(x as u8);
        newbytes.push(y as u8);
    }
//    println!("Loaded {} bytes", newbytes.len()/2);

//  Binary search
    let mut left = 1024;
    let mut right = newbytes.len()/2;
    loop {
        let middle = (right+left+1)>>1;
//        println!("Trying {left}..{right} -> {middle}");
        let save = space.clone();
        for i in left..middle {
            space[xy2o(newbytes[i*2] as usize, newbytes[i*2+1] as usize)] = 0;
        }
        let p = bfs(&space);
        if p < 5000 {
            left = middle;
        }
        else {
            space = save.clone();
            right = middle;
        }
        if left+5 >= right { break; }
    }
    loop {
        for i in left..=right {
            space[xy2o(newbytes[i*2] as usize, newbytes[i*2+1] as usize)] = 0;
            let p = bfs(&space);
            if p > 5000 {
                return (part1, format!("{},{}", newbytes[i*2], newbytes[i*2+1]));
            }
        }
    }
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] != '\n' as u8 {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}