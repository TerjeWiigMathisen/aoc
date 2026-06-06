// Fastest run: 1 us
//use std::io;
//use std::env;
//use std::fs;
//use aoc_parse::{parser, prelude::*};
//use std::collections::HashMap;

use devtimer::DevTime;
use devtimer::run_benchmark;

fn process1(inp: i32) -> i32
{
    let (a,b) = index2xy(inp);
    return a.abs()+b.abs();
}

// fn key(x:i32, y:i32)->String
// {
//     let mut s = x.to_string();
//     s.push(',');
//     s.push_str(&y.to_string());
//     return s;
// }

fn index2xy(idx:i32)->(i32,i32)
{
    if idx <= 1 { return (0,0); }

    let mut r = 1;
    let mut round = r*r;
    while idx > round {
        r += 2;
        round = r*r;
    }
    let prev_round = (r-2)*(r-2);
    let rsize = round - prev_round;

    r -= 1;             // side == rsize >> 2;
    let r2 = r>>1; // half side
    let offs = (idx - prev_round) % rsize; // partial last round
    let side = offs / r;
    let s = offs % r;
    let (a,b) = (r2, r2-s);
    if side == 0 { return (a, b); }
    if side == 1 { return (b, -a); }
    if side == 2 { return (-a, -b); }
    return (-b, a);
}

// fn process2(target:i32)->i32
// {
//     let mut m = HashMap::new();
//     m.insert(key(0,0),1);
//     let mut i = 1;
//     loop {
//         i += 1;
//         let (x,y) = index2xy(i);
//         let mut s = 0;
//         for d in [(1,0),(1,1),(1,-1),(0,1),(0,-1),(-1,0),(-1,1),(-1,-1)] {
//             let dk = key(x+d.0,y+d.1);
//             if m.contains_key(&dk) {
//                 s += m[&dk];
//             }
//         }
//         m.insert(key(x,y),s);
//  //       println!("i:{i}, x:{x}, y:{y} -> {s}");
//         if s > target {
//             if false {
//             for a in -4..5 {
//                 for b in -4..5 {
//                     let k = key(b,a);
//                     if m.contains_key(&k) {
//                         print!("{:>7}",m[&k]);
//                     }
//                     else {
//                         print!("       ");
//                     }
//                 }
//                 println!("");
//             } 
//             }
//             return s; 
//         }
//     }
// }

const OFFS:i32 = 9;
const DX:[i32;4] = [1,0,-1,0];
const DY:[i32;4] = [0,-1,0,1];

fn process2flat(target:i32)->i32
{
    let mut m:Vec<Vec<i32>> = vec![vec![0;(OFFS+OFFS+1) as usize];(OFFS+OFFS+1) as usize];
    let mut dir = 0;
//    let mut i = 1;
    let mut x = OFFS+1;
    let mut y = OFFS+1;
    m[y as usize][x as usize] = 1;

    loop {
        x += DX[dir];
        y += DY[dir];
//        i += 1;
        let mut s = 0;
        // for d in [(1,0),(1,1),(1,-1),(0,1),(0,-1),(-1,0),(-1,1),(-1,-1)] {
        //     let (dx, dy) = (x+d.0,y+d.1);
        //     s += m[dy as usize][dx as usize];
        // }
        for dy in y-1..y+2 {
            // for x in x-1..x+2 {
            //     s += m[dy as usize][x as usize];
            // }
            s += m[dy as usize][x as usize-1] + m[dy as usize][x as usize] + m[dy as usize][x as usize+1];
        }
        m[y as usize][x as usize] = s;
        if s > target {
            return s; 
        }
        let leftdir = (dir+1) & 3;
        let (lx, ly) = (x+DX[leftdir],y+DY[leftdir]);
        if m[ly as usize][lx as usize] == 0 {
            dir = leftdir;
        }
    }
}


fn main() {
    //let args: Vec<String> = env::args().collect();

    assert!(index2xy(33) == (1,-3));
//    for i in 0..26 {
//        let (x,y) = index2xy(i);
//        println!("i:{i}, x:{x}, y:{y}");
//    }
    assert!(process1(12) == 3);
    assert!(process1(23) == 2);
    assert!(process1(1024) == 31);

//    let fname = "input.txt"; // instead of args[1]
//    let input = fs::read_to_string(fname).expect("Error readin input file");
    let input = 368078;
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| {
        process1(input);
        process2flat(input);
    });
    bench_result.print_stats();

    devtime.start();
    let part1 = process1(input);
    let part2 = process2flat(input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} ns",devtime.time_in_nanos().unwrap());
}