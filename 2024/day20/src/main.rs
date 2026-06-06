use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

struct Board {
    board: Vec<u8>,
    width: i32,
    height: i32,
    stride: i32,
    xguard:i32,
    yguard:i32,
}

impl Board {
    fn new(width:i32 , height:i32) -> Self {
        let xguard = 19;
        let yguard = 19;
        let stride = width + xguard;
        let board = vec![b'#'; (stride * (height + yguard + yguard)) as usize];
        Self { board, width, height, stride, xguard, yguard }
    }

    fn xy2pos(&self, x: i32, y: i32) -> usize {
        ((y+self.yguard) * self.stride + x + self.xguard) as usize
    }

    //fn get(&self, x: usize, y: usize) -> u8 {
    //    self.board[self.xy2pos(x, y)]
    //}

    fn set(&mut self, x: i32, y: i32, val: u8) {
        let p = self.xy2pos(x, y);
        self.board[p] = val;
    }

    fn bfs(&self, sx:i32,sy:i32,ex:i32,ey:i32) -> Vec<u16> {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; (self.stride * (self.height +self. yguard + self.yguard)) as usize];
        let mut dist:Vec<u16> = vec![u16::MAX; (self.stride * (self.height + self.yguard + self.yguard)) as usize];
        let start = self.xy2pos(sx, sy);
        let end = self.xy2pos(ex, ey);
        dist[start] = 1;
        queue.push_back(start);
        visited[start] = true;
        while !queue.is_empty() {
            let p = queue.pop_front().unwrap();
            let di = dist[p];
            if p == end { continue;}
            for np in [p as i32 - self.stride, p as i32 - 1, p as i32 + 1, p as i32 + self.stride].iter() {
                let n = *np as usize;
                if visited[n] || self.board[n] == b'#' {
                    continue;
                }
                visited[n] = true;
                dist[n] = di + 1;
                queue.push_back(n);
            }
        }
        dist
    }

}

fn process(inp:String) -> (i32, i32)
{
    let mut part1 = 0;
    let mut part2 = 0;
    let byt = inp.as_bytes();
    let mut width = 0;
    let mut height = 0;
    let mut x = 0;
    let mut y = 0;
    let mut sx = 0;
    let mut sy = 0;
    let mut ex = 0;
    let mut ey = 0;
    for i in 0..byt.len() {
        if byt[i] == b'\n' {
            width = x;
            y += 1;
            x = 0;
        }
        else {
            if byt[i] == b'S' {
                sx = x;
                sy = y;
            }
            else if byt[i] == b'E' {
                ex = x;
                ey = y;
            }
            x += 1;
        }
    }
    if byt[byt.len()-1] == b'\n' { height = y;} else {height = y+1;}

    let mut board = Board::new(width, height);
    (x,y) = (0,0);
    for i in 0..byt.len() {
        if byt[i] == b'\n' {
            y += 1;
            x = 0;
        }
        else {
            board.set(x, y, byt[i]);
            x += 1;
        }
    }
    let forw = board.bfs(sx, sy, ex, ey);
    let back = board.bfs(ex, ey, sx, sy);

    //print!("Forw: {}\n", forw[board.xy2pos(ex, ey)]);
    //print!("Back: {}\n", back[board.xy2pos(sx, sy)]);

    let distance = forw[board.xy2pos(ex, ey)] as i32;

    for y in 0..height {
        for x in 0..width {
            let p = board.xy2pos(x, y);
            let f = forw[p] as i32;
            if f != u16::MAX as i32 {
                let mut dy:i32 = -2;
                while dy <= 2 {
                    let xlen = 2 - dy.abs();
                    let mut dx:i32 = -xlen;
                    while dx <= xlen {
                        let xp = board.xy2pos(x + dx, y + dy);
                        let dist = f + back[xp] as i32 + dy.abs() + dx.abs() - 1;
                        let save = distance - dist;
                        if save >= 100 {
                            part1 += 1;
                        }
                        dx += 2;
                    }
                    dy += 2;
                }                
            }
        }
    }
    //println!("Part1 = {}", part1);

    for y in 0..height {
        for x in 0..width {
            let p = board.xy2pos(x, y);
            let f = forw[p] as i32;
            if f != u16::MAX as i32 {
                let mut dy:i32 = -20;
                while dy <= 20 {
                    let xlen = 20 - dy.abs();
                    let mut dx:i32 = -xlen;
                    while dx <= xlen {
                        let xp = board.xy2pos(x + dx, y + dy);
                        let dist = f + back[xp] as i32 + dy.abs() + dx.abs() - 1;
                        let save = distance - dist;
                        if save >= 100 {
                            part2 += 1;
                        }
                        dx += 1;
                    }
                    dy += 1;
                }                
            }
        }
    }
    //println!("Part2 = {}", part2);

    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    //let fname = "test.txt"; // instead of args[1]
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