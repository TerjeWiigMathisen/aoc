// day17
// Surface: 
// Acer:    

use std::fs;
use devtimer::run_benchmark;
use std::collections::VecDeque;

struct BfsEntry{
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
    let mut bfs:VecDeque<BfsEntry> = VecDeque::new();
    let xmax = lines[0].len()-1;
    let ymax = lines.len()-1;
    let mut seen:Vec<Vec<u8>> = vec![vec![0;xmax+1];ymax+1];
    bfs.push_back(BfsEntry {loss:0,x:0,y:0,dir:0});
    bfs.push_back(BfsEntry {loss:0,x:0,y:0,dir:1});
    let mut best = u16::MAX;
    while bfs.len() > 0 {
        let e = bfs.pop_front().unwrap();
        let bit = 1 << e.dir;
        let (x, y) = (e.x as usize, e.y as usize);
        if seen[y][x] & bit != 0 {continue;}
        seen[y][x] |= bit;

        let loss = e.loss;
        if x == xmax && y == xmax {
            if loss < best { best = loss; }
            continue;
        }
        for turn in 0..2 {
            let nd = (e.dir+1+turn*2) & 3;
            let mut l = e.loss;
            let (dx, dy) = (DX[nd as usize], DY[nd as usize]);
            let (mut nx, mut ny) = (e.x, e.y);
            for r in 1..=lmax {
                nx += dx; ny += dy;
                if nx < 0 || nx > xmax as i16 || ny < 0 || ny > ymax as i16 {break;}
                l += (lines[ny as usize].as_bytes()[nx as usize] & 15) as u16;
                if r < lmin {continue;}
                bfs.push_back(BfsEntry {loss:l, x:nx, y:ny, dir:nd});
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
    
    let bench_result = run_benchmark(1000, |_| {
        process(&input);
    });
    bench_result.print_stats();

    let res = process(&input);
    println!("part1={}\npart2={}", res.0, res.1);
}
