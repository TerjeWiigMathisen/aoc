//aoc 2022 day12
// Fastest run: Surface Pro 8 29.8 (u8), 35.3 (u32) us
//                       Acer 15.7 us

//use devtimer::DevTime;
use devtimer::run_benchmark;
//use std::arch::x86_64::*;

struct Grid {
    _width:usize,
    _height:usize,
    stride:usize,
    start:usize,
    end:usize,
    cells:Vec<u8>,
}

impl Grid {
    fn new(input:&[u8]) -> Grid {
        let mut width = 0;
        while width < input.len() && input[width] != b'\n' {
            width += 1;
        }
        let stride = width + 1;
        let height = (input.len()+1) / stride;
        let mut g = Grid {
            _width: width,
            _height: height,
            stride: stride,
            start: 0,
            end: 0,
            cells: vec![0;stride*(height+2)],
        };
        let mut i = 0;
        for y in 1..=height {
            for x in 0..width {
                let mut c = input[i];
                if c == b'S' {
                    g.start = y * stride + x;
                    c = b'a';
                } else if c == b'E' {
                    g.end = y * stride + x;
                    c = b'z';
                }
                g.cells[y*stride + x] = (c - b'a' + 2) << 3;
                i += 1;
            }
            i += 1; // skip newline
        }
        g
    }
}

#[inline(never)]
fn process(inp:&[u8]) -> (u32, u32)
{
    let mut grid = Grid::new(inp);
    let mut part2 = 0;
    let mut deq = std::collections::VecDeque::<[u32;2]>::new();
    deq.push_back([grid.end as u32, 0]);
    while let Some([id, stp]) = deq.pop_front() {
        let idx = id as usize;
        let cell = grid.cells[idx];
        if cell & 7 != 0 {
            continue;
        }
        //println!("idx={} stp={} height={} steps={}", idx, stp, height, steps);
        if idx == grid.start {
            return (stp >> 3, part2);
        }
        if cell == 16 && part2 == 0 {
            part2 = stp >> 3;
        }
        grid.cells[idx] = cell | (stp & 7) as u8;

        let d = (stp & !7) + 8;
        let step_limit = cell - 8;
        if grid.cells[idx+1] >= step_limit {
            deq.push_back([(idx+1) as u32, d+1]);
        }
        if grid.cells[idx-1] >= step_limit {
            deq.push_back([(idx-1) as u32, d+2]);
        }
        if grid.cells[idx-grid.stride] >= step_limit {
            deq.push_back([(idx-grid.stride) as u32, d+3]);
        }
        if grid.cells[idx+grid.stride] >= step_limit {
            deq.push_back([(idx+grid.stride) as u32, d+4]);
        }
    }
    (0,0)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // if input.as_bytes()[input.len() - 1] != b'\n' {
    //     input.push('\n');
    // }

    let bench_result = run_benchmark(1000, |_| {
        process(&input.as_bytes());
    });
    bench_result.print_stats();

    let display = process(&input.as_bytes());
    println!("part1={}", display.0);
    println!("part2={}", display.1);
}
