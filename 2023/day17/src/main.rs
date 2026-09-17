// day17
// Surface: 151010 us
// Acer:    39937 us

use std::fs;
use devtimer::run_benchmark;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Entry{
    pri:u64,
    loss:u16,
    x:i16,
    y:i16,
    dir:u16,
}

// directions = rt 0, dn 1, lt 2, up 3
const
  DX:[i16;4] = [1,0,-1,0];
const
  DY:[i16;4] = [0,1,0,-1];

fn solve(lines:&Vec<&[u8]>, lmin:usize, lmax:usize) -> usize
{
    let mut bfs:BinaryHeap<Entry> = BinaryHeap::new();
    let xmax = lines[0].len()-1;
    let ymax = lines.len()-1;
//    println!("xmax:{xmax}, ymax:{ymax}");
    let mut seen:Vec<Vec<u8>> = vec![vec![0;xmax+1];ymax+1];
    bfs.push(Entry {pri:u64::MAX, loss:0,x:0,y:0,dir:0});
    bfs.push(Entry {pri:u64::MAX, loss:0,x:0,y:0,dir:1});
    while let Some(e) = bfs.pop() {
        let (x,y) = (e.x as usize, e.y as usize);
        let dir = e.dir as usize;
        let bit = 1 << (dir & 1);
        let prev = seen[y][x];
        if prev & bit != 0 {continue;}
        seen[y][x] = prev | bit;
        let loss = e.loss;

        if x == xmax && y == xmax {
            return loss as usize;
        }

        let mut nd = e.dir + 3;
        for _turn in 0..2 {
//            let mut pri = e.pri;
            nd = (nd + 2) & 3;
            let mut l = loss;
            let (dx, dy) = (DX[nd as usize], DY[nd as usize]);
            let (mut nx, mut ny) = (e.x, e.y);
            for r in 1..=lmax {
                nx += dx; ny += dy;
                if nx < 0 || nx as usize > xmax || ny < 0 || ny as usize > ymax {break;}

                l += (lines[ny as usize][nx as usize] & 15) as u16;
                if r < lmin {continue;}

                let pri = u64::MAX - 256  * l as u64 - (xmax-nx as usize) as u64 - (ymax-ny as usize) as u64;
                bfs.push(Entry {pri:pri, loss:l, x:nx, y:ny, dir:nd});
            }
        }
    }
    0
}

fn process(inp:&str) -> (usize, usize)
{
    let lines:Vec<&[u8]> = inp.lines().map(|s| s.as_bytes()).collect();
    let part1 = solve(&lines, 1, 3);
    let part2 = solve(&lines, 4, 10);
    (part1, part2)
}

fn main() {
//    let fname = "test.txt"; // instead of args[1]
    let fname = "input.txt"; // instead of args[1]
    let mut input:String = fs::read_to_string(fname).expect("Error reading input file");
    if input.ends_with('\n') { input.pop(); }
    
    let bench_result = run_benchmark(100, |_| {
        process(&input);
    });
    bench_result.print_stats();

    let res = process(&input);
    println!("part1={}\npart2={}", res.0, res.1);
}
