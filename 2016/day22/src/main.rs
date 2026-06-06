//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;
use priority_queue::PriorityQueue;


#[derive(Debug, Copy, Clone,Eq,PartialEq,Ord,PartialOrd, Hash)]
struct P2 {x:i8,y:i8,}
impl P2 {
    fn manhattan(self, other:&P2)->i32
    { ((self.x-other.x).abs() + (self.y-other.y).abs()) as i32 }
}
struct Move {
    fra:P2,
    til:P2,
}
impl Move { 
    fn move_len(self)->i32 {self.fra.manhattan(&self.til)}
}

#[derive(Debug, Copy, Clone,Eq,PartialEq,Ord,PartialOrd, Hash)]
struct Node {
    x:i32,
    y:i32,
    size:i32,
    used:i32,
    avail:i32,
}

fn pack(x:usize, y:usize, len:usize)->usize { len*65536+y*256+x }

fn unpack(i:usize)->(usize,usize, usize) { (i & 255, (i >> 8) & 255, i >> 16) }

fn manhattan(x0:i32,y0:i32,x1:i32,y1:i32) -> i32 {(x0-x1).abs() + (y0-y1).abs() }

fn clean(grid:&mut Vec<Vec<Node>>)
{
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            grid[y][x].x = i32::MAX;
            grid[y][x].y = i32::MAX;
        }
    }
}

fn search(grid:&mut Vec<Vec<Node>>, mv:Move, target:P2, steps:i32)->i32
{
    if mv.til.manhattan(&target) == 0 {return steps};

    if mv.til.y < 0 || mv.til.y >= grid.len() as i8 {return usize::MAX};
    if mv.til.x < 0 || mv.til.y >= grid[0].len() as i8 {return usize::MAX};
    let x = mv.til.x as usize; let y = mv.til.y as usize;
    let fx = mv.fra.x as usize; let fy = mv.fra.y as usize;
    if grid[y][x].x <= steps {return usize::MAX};
    if grid[fy][fx].used * grid[y][x].used > 0 || grid[y][x].avail < grid[fy][fx].used {return usize::MAX};

    let mut moves:Vec<Move> = vec![];
    let savesteps = grid[y][x].x;
    grid[y][x].x = steps;

    clean(&mut grid);

}

fn process(inp:String) -> (i64,i64)
{
    let lines:Vec<String> = inp.split("\n").map(|s| s.to_string()).collect();
    let p = parser!("/dev/grid/node-x" x:i32 "-y" y:i32 "\s+" size:i32 "T\s+" used:i32 "T\s+" avail:i32 "T.+" 
            => Node{x,y,size,used,avail});

    let mut grid:Vec<Vec<Node>> = vec![vec![]];
    let mut ex = 0; let mut ey = 0;
    for i in 2..lines.len() {
        let mut n = p.parse(&lines[i]).expect("Parsing error!");
        let x = n.x as usize; n.x = 0;
        let y = n.y as usize; n.y = 0;
        if y >= grid.len() {
            grid.push(vec![]);
        }
        grid[y].push(n);
        if n.used == 0 { ex = x; ey = y; }
    }
    let mut tx = grid[0].len()-1; let mut ty = 0;

    // Routing: Alternate picking a new square for the target payload, moving the zero block there and then swap the payload and zero
    let mut pq:PriorityQueue<usize,usize> = PriorityQueue::new();
    pq.push(pack(tx,ty),0);
    while !pq.is_empty() {
        let (i,p) = pq.pop().expect("Should get an item from the pq here!");
        let (x,y) = unpack(i);
        if x > 0 && grid[y][x-1].avail >= grid[y][x].used { pq.push(pack(x-1,y),x-1+y)}
        if y > 0 && grid[y-1][x].avail >= grid[y][x].used { pq.push(pack(x-1,y),x-1+y)}

    }
    return (0,0);
}


fn main() {
    assert!(process(
"Filesystem            Size  Used  Avail  Use%
/dev/grid/node-x0-y0   10T    8T     2T   80%
/dev/grid/node-x0-y1   11T    6T     5T   54%
/dev/grid/node-x0-y2   32T   28T     4T   87%
/dev/grid/node-x1-y0    9T    7T     2T   77%
/dev/grid/node-x1-y1    8T    0T     8T    0%
/dev/grid/node-x1-y2   11T    7T     4T   63%
/dev/grid/node-x2-y0   10T    6T     4T   60%
/dev/grid/node-x2-y1    9T    8T     1T   88%
/dev/grid/node-x2-y2    9T    6T     3T   66%".to_owned()) == (7,0));

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