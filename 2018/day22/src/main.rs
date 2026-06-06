//use std::collections::HashMap;
//use std::io;
//use std::env;
//use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;
use priority_queue::PriorityQueue;

fn combine(x:usize, y:usize, eq:usize, dist:usize, fra:usize, peq:usize) -> usize
{
    return eq + (x << 2) + (y << 18) + (dist << 34) + (fra << 50) + (peq << 53);
}

fn split(n:usize) -> (usize, usize, usize, usize, usize, usize)
{
    return ((n >>2) & 0xffff, (n >>18) & 0xffff, n & 3, (n >> 34) & 0xffff, (n >> 50) & 7, n>>53);
}

fn enque(pq:&mut PriorityQueue<usize,usize>, x:usize, y:usize, tx:usize, ty:usize, eq:usize,dist:usize,fra:usize,peq:usize)
{
    pq.push(combine(x,y,eq,dist, fra,peq), 0x10000000 - dist - ((eq != 1) as usize * 7) - ((x as i64 - tx as i64).abs()+(y as i64-ty as i64).abs()) as usize);
}

fn dequeue(pq:&mut PriorityQueue<usize,usize>) -> (usize, usize, usize, usize, usize, usize)
{
    let (i,_pri) = pq.pop().unwrap();
    return split(i);
}

fn show_route(dist:&Vec<Vec<Vec<usize>>>,tx:usize,ty:usize)
{
    let mut x = tx; let mut y = ty; let mut e = 1; let mut t = 0;
    while x != 0 || y != 0 {
        println!("x={x:3}, y={y:3} e={e} t={t}");
        let d:usize = dist[e][y][x];
        let (f, pe) = ((d >> 16) & 7, d >> 19);
        if pe != e {
            t += 7; // Equipment change
        }    
        else {
            t += 1;     // Move
            match f {
                0 => x -= 1,
                1 => y += 1,
                2 => x += 1,
                3 => y -= 1,
                _ => panic!(),
            }
        }
        e = pe;
    }
    println!("x={x:3}, y={y:3} e={e} t={t}");
}

fn process(depth:usize, tx:usize, ty:usize, show:bool) -> (i64,i64)
{
    let (xsize, ysize) = (tx*2,ty*3/2);
    let mut grid:Vec<Vec<usize>> = vec![vec![0;xsize];ysize];
    for y in 0..ysize {
        for x in 0..xsize {
            let mut geo = if y == 0 {x*16807}else if x == 0 {y*48271}else {grid[y][x-1]*grid[y-1][x]};
            if x == tx && y == ty {geo = 0;}
            let ero = (geo + depth) % 20183;
            grid[y][x] = ero;
        }
    }

    let mut risk = 0;
    let p:Vec<char> = vec!['.','=','|'];
    for y in 0..ysize {
        for x in 0..xsize {
            let erosion = grid[y][x] % 3;
            grid[y][x] = erosion;
            if x <= tx && y <= ty { risk += erosion as i64; }
        }
    }
    // A*! routing
    // Equipment: Neither, Torch, Climbing (0,1,2)
    let mut dist:Vec<Vec<Vec<usize>>> = vec![vec![vec![usize::MAX;grid[0].len()];grid.len()];3];
    let mut pq:PriorityQueue<usize, usize> = PriorityQueue::new();
    let mut enq = 1;
    let mut deq = 0;
    enque(&mut pq,0,0,tx,ty,1,0,3,1);
    while pq.len() > 0 {
        let (x,y,e,d,fra,pe) = dequeue(&mut pq);
        deq += 1;
        if (dist[e][y][x] & 0xffff) <= d {continue}
        let g = grid[y][x];
        if e == g {continue;}

        dist[e][y][x] = d + (fra << 16) + (pe << 19);

        if x == tx && y == ty && e == 1 {
            if show {
                show_route(&dist, tx, ty);
                println!("pushed {enq} entries, popped {deq}");
            }
            return (risk, d as i64);
        }

        // Try to move in all 4 directions, but stay within the playing field:
        if x > 0 {
            enq += 1;
            enque(&mut pq,x-1,y,tx,ty,e,d+1,2,e);
        }
        if x+1 < grid[y].len() {
            enq += 1;
            enque(&mut pq,x+1,y,tx,ty,e,d+1,0,e);
        }
        if y > 0 {
            enq += 1;
            enque(&mut pq,x,y-1,tx,ty,e,d+1,1,e);
        }
        if y+1 < grid.len() {
            enq += 1;
            enque(&mut pq,x,y+1,tx,ty,e,d+1,3,e);
        }
        // As a last resort, switch equipment:
        enq += 1;
        let e1 = if e < 2 {e+1}else{0};
        if e1 != g {
            enque(&mut pq,x,y,tx,ty,e1,d+7,3,e);
        }
        else {
            let e2 = if e == 0 {2}else{e-1};
            enque(&mut pq,x,y,tx,ty,e2,d+7,3,e);
        }
    }
    println!("Target not found! Extend the playing field?");
    return (0,0);
}

fn main() {
    assert!(process(510,10,10,false) == (114,45));
    process(510,10,10,true);

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { process(7305, 13, 734, false); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(7305, 13, 734,false);
    devtime.stop();

//    process(7305, 13, 734,true);
    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}