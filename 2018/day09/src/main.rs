//use std::collections::HashMap;
//use std::io;
//use std::env;
//use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn process(players:usize, lastmarble:usize) -> i64
{
    let mut playersum:Vec<i64> = vec![0;players];
    let mut ring:Vec<i32> = vec![-1;lastmarble+1];
    ring[0] = 0;
    let mut m23 = 23;
    let mut pl = 1;
    let mut pos = 0;
    let mut len = 1;
    let mut disp = 100000;
    for r in 1..=lastmarble {
        pos += 1; if pos >= len {pos = 0}
        pos += 1; // if pos > len {pos = 0}
        if r == m23 {
            let cut = if pos >= 9 {pos-9} else {len-(9-pos)};
            playersum[pl-1] += r as i64 + ring[cut] as i64;
            for i in cut+1..len {
                ring[i-1] = ring[i];
            }
            pos = cut;
            len -= 1;
            m23 += 23;
        }
        else {
            let mut i = len;
            while i > pos {
                ring[i] = ring[i-1];
                i -= 1;
            }
            ring[pos] = r as i32;
            len += 1;
        }
        if r == disp {
            println!("{r}");
            disp += 100000;
        }
//        println!("{:?}",ring);
        pl = if pl < players {pl+1} else {1};
    }
    let mut win = playersum[0];
    for p in 1..players {
        if win < playersum[p] {win = playersum[p]}
    }
    return win;
}

fn process_fast(players:usize, lastmarble:usize) -> i64
{
    let mut playersum:Vec<i64> = vec![0;players];
    let mut ring:Vec<i32> = vec![-1;lastmarble+1];
    ring[0] = 0;
    let mut m23 = 23;
    let mut pl = 1;
    let mut pos = 0;
    let mut len = 1;
    let mut disp = 100000;
    let mut r = 1;
    while r <= lastmarble {
        pos += 1; if pos >= len {pos = 0}
        pos += 1; // if pos > len {pos = 0}
        if r == m23 {
            let cut = if pos >= 9 {pos-9} else {len-(9-pos)};
            playersum[pl-1] += r as i64 + ring[cut] as i64;
            for i in cut+1..len {
                ring[i-1] = ring[i];
            }
            pos = cut;
            len -= 1;
            m23 += 23;
            r += 1;
        }
        else {
            // Place as many as possible:
            let mut max = len - pos + 1;
            if r+max >= m23 {max = m23-r;}
            let mut i = len+max-1;
            while i >= pos+max {
                ring[i] = ring[i-max*2];
                i -= 1;
            }
            let mut m = max-1;
            while i > pos {
                ring[i] = (r + m) as i32;
                i -= 1;
                m -= 1;
                ring[i] = ring[i-m*2];
            }
            ring[i] = r as i32;
            r += max; 
            pos += max*2 - 1;
            len += max;
        }
        if r == disp {
            println!("{r}");
            disp += 100000;
        }
        println!("{:?}",ring);
        pl = if pl < players {pl+1} else {1};
    }
    let mut win = playersum[0];
    for p in 1..players {
        if win < playersum[p] {win = playersum[p]}
    }
    return win;
}

#[derive(Clone)]
struct Marble {
    next:u32,
    prev:u32,
}

fn disp_marble(ring:&Vec<Marble>, pos:usize)
{
    let mut m:usize = 0;
    while ring[m].next == u32::MAX {m += 1;}
    let start = m;
    let mut max = 40;
    loop {
        if m == pos { print!("({})",m)}
        else {print!("{},",m);}
        m = ring[m].next as usize;
        if m == start {break;}
        max -= 1;
        if max == 0 {break;}
    }
    println!("");
}

fn process_double_linked(players:usize, lastmarble:usize) -> i64
{
    let mut playersum:Vec<i64> = vec![0;players];
    let marble_head = Marble{next:0,prev:0};
    let mut ring:Vec<Marble> = vec![marble_head;lastmarble+1];
    let mut m23 = 23;
    let mut player = 1;
    let mut pos = 0;
//    let mut disp = 100000;
    for r in 1..=lastmarble {
        pos = ring[pos].next as usize;
        if r == m23 {
            for _ in 0..8 {
                pos = ring[pos].prev as usize;
            }
            playersum[player-1] += r as i64 + pos as i64;
            let prev = ring[pos].prev; let next = ring[pos].next;
            ring[prev as usize].next = next;
            ring[next as usize].prev = prev;
            ring[pos].next = u32::MAX;
            ring[pos].prev = u32::MAX;
            pos = next as usize;
            m23 += 23;
        }
        else {
            let n = ring[pos].next;
            ring[r as usize] = Marble{prev:pos as u32,next:n};
            ring[pos].next = r as u32;
            ring[n as usize].prev = r as u32;
            pos = r as usize;
        }
/*         if r == disp {
            println!("{r}");
            disp += 100000;
        }
 *///        disp_marble(&ring,pos);
        player = if player < players {player+1} else {1};
    }
    let mut win = playersum[0];
    for p in 1..players {
        if win < playersum[p] {win = playersum[p]}
    }
    return win;
}

fn main() {
    assert!(process_double_linked(9,25) == 32);
    assert!(process_double_linked(13,7999) == 146373);

//    let fname = "input.txt"; // instead of args[1]
//    let mut input = fs::read_to_string(fname).expect("Error readin input file");
//    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1, |_| { process_double_linked(470,72170); }); bench_result.print_stats();

    devtime.start();
    let part1 = process_double_linked(470,72170);
    let part2 = process_double_linked(470,72170*100);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}