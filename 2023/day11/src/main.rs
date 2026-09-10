// day11
// Surface:
// Acer:    127 us
use std::fs;
use devtimer::run_benchmark;

struct Point {
    x: u32,
    y: u32,
}

pub fn process(inp:&str) -> (usize, usize)
{
    let input = inp.as_bytes();
    let mut i = 0;
    while input[i] != b'\n' { i += 1; } // Measure line length
    let line_len = i;
    i = 0;
    let vert_len = input.len() / line_len;

    let mut x = 0;
    let mut y = 0;
    let mut hblank = true;
    let mut hblank_cnt:Vec<usize> = vec![0; vert_len];
    let mut vblank = vec![true; line_len];
    let mut vblank_cnt:Vec<usize> = vec![0; line_len];
    let mut prev_hblank_cnt = 0;
    let mut points:Vec<Point> = Vec::new();
    while i < input.len() {
        let c = input[i]; i += 1;
        if c == b'\n' {
            if hblank == true { 
                prev_hblank_cnt += 1;
//                println!("hblank_cnt[{}]={}", y, prev_hblank_cnt);
            }
            hblank_cnt[y] = prev_hblank_cnt;
            y += 1;
            x = 0;
            hblank = true;
        } else {
            if c == b'#' { 
                points.push(Point { x: x as u32, y: y as u32 });
                vblank[x as usize] = false;
                hblank = false;
            }
            x += 1;
        }
    }
    let mut prev_vblank_cnt = 0;
    for x in 0..line_len {
        if vblank[x] { 
            prev_vblank_cnt += 1; 
//            println!("vblank_cnt[{}]={}", x, prev_vblank_cnt);
        }
        vblank_cnt[x] = prev_vblank_cnt;
    }
    let mut part1 = 0;
    let mut part2 = 0;
    for second in 1..points.len() {
        let p2 = &points[second];
        for first in 0..second {
            let p1 = &points[first];
            let mut x0 = p1.x as usize;
            let mut x1 = p2.x as usize;
            let mut y0 = p1.y as usize;
            let mut y1 = p2.y as usize;
            if x0 > x1 {
                let t = x0; x0 = x1; x1 = t;
            }
            if y0 > y1 {
                let t = y0; y0 = y1; y1 = t;
            }
            let dx = x1 - x0 + ((vblank_cnt[x1] - vblank_cnt[x0])) * 1;
            let dy = y1 - y0 + ((hblank_cnt[y1] - hblank_cnt[y0])) * 1;
            part1 += dx + dy;
            let dx = x1 - x0 + ((vblank_cnt[x1] - vblank_cnt[x0])) * 999999;
            let dy = y1 - y0 + ((hblank_cnt[y1] - hblank_cnt[y0])) * 999999;
            part2 += dx + dy;
        }
    }
    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
//    let fname = "test.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] != b'\n' as u8 {input.push('\n');}

    let bench_result = run_benchmark(1000, |_| {
        process(&input);
    });
    bench_result.print_stats();

    let res = process(&input);
    println!("part1={}\npart2={}", res.0, res.1);
}
