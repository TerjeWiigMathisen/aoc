//use std::io;
//use std::env;
use std::fs;
//use std::collections::HashMap;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

fn fill_reach(conn:&mut Vec<Vec<u16>>, reach:&mut Vec<bool>, idx:usize)->usize
{
    reach[idx] = true;
    let mut cnt = 1;
    let conns = conn[idx].clone();
    for i in conns {
        if reach[i as usize] { continue; }
        cnt += fill_reach(conn, reach, i as usize);
    }
    return cnt;
}

fn parseu16(line:&str) -> Vec<u16>
{
    let mut v:Vec<u16> = vec![];
    let bytes = line.as_bytes();
    let mut i = 0;
    let mut val:u16;
    while i < bytes.len() {
        let b = bytes[i]; i += 1;
        if b >= b'0' && b <= b'9'{
            val = (b - b'0') as u16;
            while i < bytes.len() {
                let b = bytes[i]; i += 1;
                if b < b'0' || b > b'9' { break; }
                val = val*10 + (b - b'0') as u16;
            }
            v.push(val);
        }
    }
    v
}

fn process(inp:String) -> (i32,i32)
{
    let p1;
    let mut p2;
    let lines = inp.lines();

    let mut conn:Vec<Vec<u16>> = Vec::with_capacity(2000);
    let mut v:usize = 0;
    for li in lines {
//        let parts:Vec<&str> = li.split(" <-> ").collect();
//        let curr = parts[0];
//        assert!(parts[0].parse::<usize>().unwrap() == v);
        conn.push(vec![]);
//        let connections = parts[1].trim().split(", ")
//                .map(|x| x.parse::<u16>().unwrap()).collect::<Vec<u16>>();
//        conn[v] = connections; //.clone();
        conn[v] = parseu16(li);
        v += 1;
    }
    let len = conn.len();
    let mut reach:Vec<bool> = vec![false;len];
    p1 = fill_reach(&mut conn, &mut reach, 0);
    p2 = 1;
    for c in 1..conn.len() {
        if reach[c] == false {
            fill_reach(&mut conn, &mut reach, c);
            p2 += 1;
        }
    }
    return (p1 as i32, p2 as i32);
}

fn main() {

    assert!(process("0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5".to_owned()).0 ==6);
    let fname = "input.txt"; // instead of args[1]
    let input = fs::read_to_string(fname).expect("Error readin input file");
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(25, |_| {
        process(input.clone());
    });
    bench_result.print_stats();

    devtime.start();
    let (part1,part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}