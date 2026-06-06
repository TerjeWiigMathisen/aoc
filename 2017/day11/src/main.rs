//Fastest run: 26,9 us
//use std::io;
//use std::env;
use std::fs;
//use std::collections::HashMap;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

fn xy2dist(x:i32, y:i32)->i32
{
    let (x2, y2) = (x.abs()/2, y.abs());
    let p1:i32 = x2 + if y2 > x2 {(y2-x2+1)>>1} else {0};
    p1
}

fn process(inp:String) -> (i32,i32)
{
    let moves = inp.split(",");

    let mut x:i32 = 0;
    let mut y:i32 = 0;
    let mut p1 = 0;  
    let mut p2 = 0;
    for m in moves {
        let (dx, dy) =
        match m {
            "n" =>  (0,2),
            "ne" => (2,1),
            "nw" => (-2,1),
            "s" =>  (0,-2),
            "se" => (2,-1),
            "sw" => (-2,-1),
            _ => (0,0),
        };
        x += dx; y += dy;
        p1 = xy2dist(x,y);
        if p1 > p2 { p2 = p1; }
    }
    return (p1,p2)
}

fn hash(a:u8, b:u8) -> u8
{
    ((a - b'm')>>1) +  // n=0, s=3
    ((b - b',') >> 5)  // ',' = 0, e = 1, w = 2
}

const DELTA_AXIAL_SSE:[[i16;4];6] = [
    [2,0,-1,1],  // "n,"
    [3,1,-1,0],  // "ne"
    [3,-1,0,1],  // "nw"
    [2,0,1,-1],   // "s,"
    [3,1,0,-1],   // "se"
    [3,-1,1,0],  // "sw"
];

fn process_axial_sse(inp:&str) -> (i32,i32)
{
    let bytes = inp.as_bytes();

    let mut q:i16 = 0;
    let mut r:i16 = 0;
    let mut s:i16 = 0;
    let mut p1;  
    let mut p2 = 0;
    let mut i = 0;
    // let qrs_mask = __mm_setr_epi32(0,-1,-1,-1);
    loop {
        let a = bytes[i];
        let b = bytes[i+1];

        let dqrs = DELTA_AXIAL_SSE[hash(a,b) as usize];
        i += dqrs[0] as usize;
        q += dqrs[1];
        r += dqrs[2];
        s += dqrs[3];
        p1 = q.abs() + r.abs() + s.abs();
        if p1 > p2 { p2 = p1; }
    
        if i >= bytes.len() {break;}
    }
    (p1 as i32/2, p2 as i32/2)
}

const DELTA_AXIAL:[(usize,i32,i32);6] = [
    (2,0,-1),  // "n,"
    (3,1,-1),  // "ne"
    (3,-1,0),  // "nw"
    (2,0,1),   // "s,"
    (3,1,0),   // "se"
    (3,-1,1),  // "sw"
];

fn process_axial(inp:&str) -> (i32,i32)
{
    let bytes = inp.as_bytes();

    let mut q:i32 = 0;
    let mut r:i32 = 0;
    let mut p1;  
    let mut p2 = 0;
    let mut i = 0;
    loop {
        let a = bytes[i];
        let b = bytes[i+1];

        let (di, dq, dr) = DELTA_AXIAL[hash(a,b) as usize];
        i += di; q += dq; r += dr;
        p1 = (q.abs() + r.abs() + (q+r).abs());
        if p1 > p2 { p2 = p1; }
        if i >= bytes.len() {break;}
    }
    return (p1/2, p2/2)
}

const _DELTA:[(usize,i32,i32);6] = [
    (2,0,2),    // "n,"
    (3,2,1),    // "ne"
    (3,-2,1),   // "nw"
    (2,0,-2),   // "s,"
    (3,2,-1),   // "se"
    (3,-2,-1),  // "sw"
];

fn _process_hash(inp:String) -> (i32,i32)
{
    let bytes = inp.as_bytes();

    let mut x:i32 = 0;
    let mut y:i32 = 0;
    let mut p1;  
    let mut p2 = 0;
    let mut i = 0;
    loop {
        let a = bytes[i];
        let b = bytes[i+1];

        let (di, dx, dy) = _DELTA[hash(a,b) as usize];
        i += di; x += dx; y += dy;
        p1 = xy2dist(x,y);
        if p1 > p2 { p2 = p1; }
        if i >= bytes.len() {break;}
    }
    return (p1,p2)
}

fn main() {

    println!("n -> {}", hash(b'n', 0));
    println!("ne-> {}", hash(b'n', b'e'));
    println!("nw-> {}", hash(b'n', b'w'));
    println!("s -> {}", hash(b's', 0));
    println!("se-> {}", hash(b's', b'e'));
    println!("sw-> {}", hash(b's', b'w'));

    assert!(process("ne,ne,ne".to_owned()).0 == 3);
    assert!(process("ne,ne,sw,sw".to_owned()).0 == 0);
    assert!(process("ne,ne,s,s".to_owned()).0 == 2);
    assert!(process("se,sw,se,sw,sw".to_owned()).0 == 3);

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    input.push(',');
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| {
        process_axial_sse(&input);
    });
    bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process_axial_sse(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}