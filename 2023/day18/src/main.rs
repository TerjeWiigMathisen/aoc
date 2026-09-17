// day18
// Surface:
// Acer:   

use std::fs;
use devtimer::run_benchmark;
use geo_types::{Polygon, LineString, Coord};
use geo::{Area};
use geo::algorithm::line_measures::{Euclidean, Length};

// directions = rt 0, dn 1, lt 2, up 3
const
  DX:[i64;4] = [1,0,-1,0];
const
  DY:[i64;4] = [0,1,0,-1];

// L 7 (#080f92)
// U 8 (#6345a3)
// L 9 (#8646f2)
// U 6 (#34e153)

fn hex2bin(h:u8) -> u32
{
    if h <= b'9' {return (h-b'0') as u32;}
    if h >= b'a' {return (h-b'a'+10) as u32;}
    (h-b'A'+10) as u32
}

fn d2d(h:u8) -> usize
{
    match h {b'R' => 0, b'D' => 1, b'L' => 2, b'U' => 3, _ => usize::MAX}
}

fn process(inp:&str) -> (usize, usize)
{
    let lines:Vec<&[u8]> = inp.lines().map(|s| s.as_bytes()).collect();
    let mut ext1:Vec<Coord> = Vec::new();
    let mut ext2:Vec<Coord> = Vec::new();
    ext1.push(Coord{ x:0.0, y:0.0});
    ext1.push(Coord{ x:0.0, y:0.0});
    let (mut x, mut y) = (0,0);
    for line in lines {
        let dir1 = d2d(line[0]);
        let mut i = 2;
        let mut n = (line[i] & 15) as i64; i += 1;
        while line[i] >= b'0' {
            n += (line[i] & 15) as i64; i += 1;
        }
        x += DX[dir1]*n; y += DY[dir1]*n;
        println!("{}{n} ({x},{y})", line[0] as char);
        ext1.push(Coord {x:x as f64, y:y as f64});
        i += 2; // skip '(#')
        let mut color = 0;
        for j in 0..5 {
            color = color *16 + hex2bin(line[i]); i += 1;
        }
        let dir2 = hex2bin(line[i]);
    }
    println!("Final (x,y) = ({x},{y})");
    ext1.push(Coord{ x:0.0, y:0.0});
    let ext1_ls = LineString::from(ext1);
    let poly1 = Polygon::new(ext1_ls, vec![]);
    let area = poly1.unsigned_area();
    let circ = Euclidean.length(poly1.exterior());

    let part1 = (area + circ*0.5 + 1.00001) as usize;

    (part1,0)
//    (part1, part2)
}

fn main() {
//    let fname = "test.txt"; // instead of args[1]
    let fname = "input.txt"; // instead of args[1]
    let mut input:String = fs::read_to_string(fname).expect("Error reading input file");
    if input.ends_with('\n') { input.pop(); }
    
    // let bench_result = run_benchmark(100, |_| {
    //     process(&input);
    // });
    // bench_result.print_stats();

    let res = process(&input);
    println!("part1={}\npart2={}", res.0, res.1);
}
