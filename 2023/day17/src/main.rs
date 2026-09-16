// day17
// Surface: 272331 us
// Acer:    

use std::fs;
use devtimer::run_benchmark;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct BfsEntry{
    pri:u16,
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

fn solve(lines:&Vec<&str>, lmin:usize, lmax:usize) -> usize
{
    let mut bfs:BinaryHeap<BfsEntry> = BinaryHeap::new();
    let xmax = lines[0].len()-1;
    let ymax = lines.len()-1;
//    println!("xmax:{xmax}, ymax:{ymax}");
    let mut seen:Vec<Vec<u8>> = vec![vec![0;xmax+1];ymax+1];
    bfs.push(BfsEntry {pri:u16::MAX, loss:0,x:0,y:0,dir:0});
    bfs.push(BfsEntry {pri:u16::MAX, loss:0,x:0,y:0,dir:1});
    let mut best = u16::MAX;
//    let mut xy = 0;
    while let Some(e) = bfs.pop() {
        let bit = 1 << e.dir;
        let (x,y) = (e.x as usize, e.y as usize);
        // if x+y > xy { 
        //     xy = x+y;
        //     println!("{:?}", e); 
        // }
        let prev = seen[y][x];
        let loss = e.loss;
        if prev & bit != 0 /* && ((prev >> (4+12*(e.dir&1))) & 4095) < loss */ {continue;}
        let (x, y) = (e.x as usize, e.y as usize);
        seen[y][x] |= bit;

        if x == xmax && y == xmax {
//            println!("BFS = {loss}");
            if loss < best { 
                best = loss; 
            }
            return best as usize;
//            break;
        }
        for turn in 0..2 {
//            let mut pri = e.pri;
            let nd = (e.dir+1+turn*2) & 3;
            let mut l = loss;
            let (dx, dy) = (DX[nd as usize], DY[nd as usize]);
            let (mut nx, mut ny) = (e.x, e.y);
            for r in 1..=lmax {
//                pri -= 1;
                nx += dx; ny += dy;
//                println!("{r},{nx},{ny}");
                if nx < 0 || nx as usize > xmax || ny < 0 || ny as usize > ymax {break;}
                l += (lines[ny as usize].as_bytes()[nx as usize] & 15) as u16;
                if r < lmin {continue;}
//                println!("push({nx},{ny},{nd}");
                bfs.push(BfsEntry {pri:u16::MAX-l, loss:l, x:nx, y:ny, dir:nd});
            }
        }
    }
    best as usize
}

fn process(inp:&str) -> (usize, usize)
{
    let lines:Vec<&str> = inp.lines().collect();
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
