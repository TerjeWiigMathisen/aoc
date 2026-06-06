//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn try_move(grid:&Vec<u8>, p:i32, dp:i32) -> bool
{
    let np = p + dp;
    let c = grid[np as usize];
    if c == b'#' { return false; }
    if c == b'.' { return true; }
    if c == b'[' {
        return try_move(grid, np, dp) && try_move(grid, np+1, dp);
    }
    if c == b']' {
        return try_move(grid, np-1, dp) && try_move(grid, np, dp);
    }
    if c == b'O' {
        return try_move(grid, np, dp);
    }
    return false;
}

fn do_move(grid:&mut Vec<u8>, p:i32, dp:i32)
{
    if grid[p as usize] == b'.' { return; }
    let np = p + dp;
    let c = grid[np as usize];
    if c == b'.' {
        grid[np as usize] = grid[p as usize];
        grid[p as usize] = b'.';
        return;
    }
    if c == b'[' {
        do_move(grid, np, dp);
        grid[np as usize] = grid[p as usize];
        grid[p as usize] = b'.';
        do_move(grid, np+1, dp);
        grid[np as usize+1] = grid[p as usize+1];
        grid[p as usize+1] = b'.';
        return;
    }
    if c == b']' {
        do_move(grid, np-1, dp);
        grid[np as usize-1] = grid[p as usize-1];
        grid[p as usize-1] = b'.';
        do_move(grid, np, dp);
        grid[np as usize] = grid[p as usize];
        grid[p as usize] = b'.';
        return;
    }
}

fn process(inp:String) -> (i32, i32)
{
    let mut part1 = 0;
    let mut part2 = 0;
    let mut bytes = inp.as_bytes().to_vec();
    let mut grid = bytes.clone();
    let mut width:i32 = 0;
    let mut height:i32 = 0;
    let mut sx = 0;
    let mut sy = 0;
    let mut x = 0;
    let mut y = 0;
    let mut move_start = 0;
    for i in 0..grid.len()
    {
        if grid[i] == b'\n'
        {
            if x == 0 {
                move_start = i+1;
                break;
            }

            if width == 0 {width = i as i32;}
            height += 1;
            y += 1;
            x = 0;
        }
        else {
            if grid[i] == b'@' { 
                sx = x; sy = y; 
                grid[i] = b'.'; // replace @ with .
            }
            x += 1;          
        }
    }
    // part1
    let mut p = sx + sy*(width+1);
    for i in move_start..grid.len()
    {
        let c = grid[i];
        let mut dp:i32 = 0;
        if c == b'>' { dp = 1; }
        else if c == b'<' { dp = -1; }
        else if c == b'^' { dp = -width-1; }
        else if c == b'v' { dp = width+1; }
        else { continue; }

        let mut np = p + dp;
        while grid[np as usize] == b'O' { np += dp; }
        if grid[np as usize] == b'.' {
            loop {
                let pp = np-dp;
                grid[np as usize] = grid[pp as usize];
                if pp == p { grid[p as usize] = b'.'; break; }
                np = pp;
            }
            p += dp;
        }
    }
    for y in 1..height-1
    {
        for x in 1..width-1
        {
            let p = x + y*(width+1);
            if grid[p as usize] == b'O' { 
                part1 += x + 100*y;
             }
        }
    }
    //part2
    let mut grid2 = vec![0; move_start*2];
    let mut p2 = 0;
    for p in 0..move_start
    {
        let c = bytes[p];
        grid2[p2] = c;
        p2 += 1;
        if c == b'\n' { continue; }
        if c == b'#' || c == b'.' { grid2[p2] = c; }
        else if c == b'O' { grid2[p2-1] = b'['; grid2[p2] = b']'; }
        else if c == b'@' { grid2[p2-1] = b'.'; grid2[p2] = b'.'; }
        p2 += 1;
    }
    let width = width*2;
    let sx = sx*2;

    let mut p = sx + sy*(width+1);
    for i in move_start..grid.len()
    {
        let c = grid[i];
        let mut dp:i32 = 0;
        if c == b'>' { dp = 1; }
        else if c == b'<' { dp = -1; }
        if dp != 0 {
            let mut np = p + dp;
            while grid2[np as usize] == b'[' || grid2[np as usize] == b']' { np += dp; }
            if grid[np as usize] == b'.' {
                loop {
                    let pp = np-dp;
                    grid[np as usize] = grid[pp as usize];
                    if pp == p { grid[p as usize] = b'.'; break; }
                    np = pp;
                }
                p += dp;
            }    
            continue;
        }
        if c == b'^' { dp = -width-1; }
        else if c == b'v' { dp = width+1; }
        else { continue; }
        if try_move(&grid2, p, dp) {
            do_move(&mut grid2, p, dp);
        }
    }
    for y in 1..height-1
    {
        for x in 1..width-1
        {
            let p = x + y*(width+1);
            if grid2[p as usize] == b'[' { 
                part2 += x + 100*y;
             }
        }
    }
    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}