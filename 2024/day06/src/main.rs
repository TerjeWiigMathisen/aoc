//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;
//use core::intrinsics::wrapping_add;

struct Board {
    board:Vec<u8>,
    width:usize,
    dp:[usize;4],
    //uplt:usize,
    //uprt:usize,
    //dnlt:usize,
    //dnrt:usize,
}

impl Board {
    pub fn new(inp:String) -> Board {
        let mut width = 0;
        while inp.as_bytes()[width] != b'\n' {
            width += 1;
        }
        width += 1;
        
        let lt = usize::MAX;
        let rt = 1;
        let up = (0 - width as isize) as usize;
        let dn = width;
        let dp:[usize;4] = [up,rt,dn,lt];
        //let uprt = (up as isize + rt as isize) as usize;
        //let uplt = (up as isize + lt as isize) as usize;
        //let dnrt = dn + 1;
        //let dnlt = dn - 1;
    
        let mut board:Vec<u8> = Vec::<u8>::with_capacity(inp.as_bytes().len()+width*2+2);
        for _ in 0..width+1 { board.push(b' '); }
        board.append(&mut Vec::from(inp.as_bytes()));
        for _ in 0..width+1 { board.push(b' '); }
        Board {board, width, dp } //, uplt, uprt, dnlt, dnrt}
    }

    pub fn _show(&self) {
        for y in 0..self.board.len() {
            let u = self.board[y];
            if u == b'\n' {
                println!();
            }
            else {
                print!("{}",u as char);
            }
        }
        println!();
    }
    pub fn startpos(&self) -> usize {
        let mut pos = 0;
        while self.board[pos] != b'^' {
            pos += 1;
        }
        pos
    }
    pub fn _pos2xy(&self, pos:usize) -> (i32,i32) {
        let y = (pos / self.width) as i32;
        let x = pos as i32 - y * self.width as i32;
        (x,y)
    }
    pub fn _xy2pos(&self, x:i32, y:i32) -> usize {
        x as usize + y as usize * self.width
    }
    pub fn part2loop(&self, spos:usize, sdir:usize) -> bool{
        let mut dir = sdir;
        let mut seen:Vec<u8> = vec![0;self.board.len()];
        let mut p = spos;

        let mut dp = self.dp[dir] as i64;
        
        loop {
            let mut np = (p as i64 + dp) as usize;
            //let mut np = wrapping_add(p, self.dp[dir]);
            if self.board[np] <= b' ' { return false; } // SPACE or LF guard
    
            if self.board[np] == b'#' {
                dir = (dir+1)&3;
                //if dir == 0 {
                    if seen[p] != 0 { return true; }
                    seen[p] = 1;
                //}
                np = p;
                dp = self.dp[dir] as i64;
            }
            p = np;
        }
    }
    pub fn part1path(&self, spos:usize, sdir:usize) -> Vec<u8> {
        let mut dir = sdir;
        let mut dirb = 1 << dir;
        let mut seen:Vec<u8> = vec![0;self.board.len()];
        let mut p = spos;
        loop {
            seen[p] |= dirb;
            let mut np = (p as i64 + self.dp[dir] as i64) as usize;
            if self.board[np] <= b' ' { return seen; }
    
            if self.board[np] == b'#' {
                dir = (dir+1)&3;
                dirb = 1 << dir;
                if seen[p] & dirb != 0 {
                    // exitboard = false;
                    break; // Inf loop!
                }
                np = p;
            }
            p = np;
        }
        seen
    }
}

fn process(inp:String) -> (usize, usize)
{
    let mut b = Board::new(inp);

    let spos = b.startpos();
    let seen = b.part1path(spos, 0);
    let mut part1 = 0;
    let mut part2 = 0;
    for s in 0..seen.len() {
        if seen[s] != 0 {
            part1 += 1;
            if s != spos {
                b.board[s] = b'#';
                if b.part2loop(spos,0) {part2 += 1;}
                b.board[s] = b'.';
            }
        }
    }
    (part1, part2)
}

fn main() {
    //let fname = "input.txt"; // instead of args[1]
    let fname = std::env::args().nth(1).expect("Please provide a filename as an argument");
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}