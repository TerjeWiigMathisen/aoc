// day18
// Surface:  25.2 us
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

const
  HEX:[u8;256] = [
0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0,
0, 10, 11, 12, 13, 14, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
0, 10, 11, 12, 13, 14, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

fn hex2bin(h:u8) -> u32
{
    // if h >= b'0' && h <= b'9' {return (h-b'0') as u32;}
    // if h >= b'a' && h <= b'f' {return (h-b'a'+10) as u32;}
    // if h >= b'A' && h <= b'F' {return (h-b'A'+10) as u32;}
    // 0
    HEX[h as usize] as u32
}

const
  D2D:[u8;256] = [
0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

fn d2d(h:u8) -> usize
{
//    match h {b'R' => 0, b'D' => 1, b'L' => 2, b'U' => 3, _ => 0}
    D2D[h as usize] as usize
}

fn process(inp:&str) -> (usize, usize)
{
    let lines:Vec<&[u8]> = inp.lines().map(|s| s.as_bytes()).collect();
    let mut ext1:Vec<Coord> = Vec::new();
    let mut ext2:Vec<Coord> = Vec::new();
    ext1.push(Coord{ x:0.0, y:0.0});
    ext1.push(Coord{ x:0.0, y:0.0});
    let (mut x, mut y) = (0,0);
    let (mut x2,mut y2) = (0,0);
    for line in lines {
        let dir1 = d2d(line[0]);
        let mut i = 2;
        let mut n = (line[i] & 15) as i64; i += 1;
        while line[i] >= b'0' {
            n = n*10 + (line[i] & 15) as i64; i += 1;
        }
        x += DX[dir1]*n; y += DY[dir1]*n;
        ext1.push(Coord {x:x as f64, y:y as f64});
        i += 3; // skip ' (#')
        let mut len2 = 0;
        for _ in 0..5 {
            len2 = len2 * 16 + hex2bin(line[i]) as i64; i += 1;
        }
        let dir2 = hex2bin(line[i]) as usize;
        x2 += DX[dir2]*len2; y2 += DY[dir2]*len2;
        ext2.push(Coord {x:x2 as f64, y:y2 as f64});
    }
    let ls = LineString::from(ext1);
    let poly = Polygon::new(ls, vec![]);
    let area = poly.unsigned_area();
    let circ = Euclidean.length(poly.exterior());

    let part1 = (area + circ*0.5 + 1.00001) as usize;

    let ls = LineString::from(ext2);
    let poly = Polygon::new(ls, vec![]);
    let area = poly.unsigned_area();
    let circ = Euclidean.length(poly.exterior());

    let part2 = (area + circ*0.5 + 1.00001) as usize;

    (part1, part2)
}

fn main() {
    // print!("const\n  D2D:[u8;256] = [\n");
    // for i in 0..256 {
    //     let n = d2d(i as u8);
    //     print!("{n}, ");
    //     if i & 31 == 31 {println!();}
    // }
    // println!("];");
    // return;
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
