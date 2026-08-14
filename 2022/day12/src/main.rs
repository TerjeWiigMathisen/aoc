//aoc 2022 day12
// Fastest run: Surface Pro 8 29.2 (u8), 35.3 (u32) us
//                       Acer 15.7 us

//use devtimer::DevTime;
use devtimer::run_benchmark;
//use std::arch::x86_64::*;
#[derive(Debug,Clone)]
struct Grid {
    _width:usize,
    _height:usize,
    stride:usize,
    start:usize,
    end:usize,
    next:[i16;8],
    next_dirs:[u8;8],
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
            next: [-(stride as i16), 1, stride as i16, -1, -(stride as i16), 1, stride as i16, -1],
            next_dirs: [4, 1, 2, 3, 4, 1, 2, 3],
            cells: vec![0; stride * (height + 2)],
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
    fn _show(&self) {
        let dirs = ['.', '>', 'v', 'v', '^', 'S', 'E', 'Z'];
        let mut sf = self.cells.clone();
        sf[self.start] = 5;
        sf[self.end] = 6;
        for y in 1..=self._height {
            for x in 0..self._width {
                let c = sf[y*self.stride + x];
                print!("{}", dirs[(c & 7) as usize]);
            }
            println!();
        }
    }
}

#[inline(never)]
fn process(inp:&[u8]) -> (u16, u16, Grid)
{
    let mut grid = Grid::new(inp);
    let mut part2 = 0;
    let mut deq = std::collections::VecDeque::<[u16;2]>::new();
    deq.push_back([grid.end as u16, 0]);
    while let Some([id, steps_and_dir]) = deq.pop_front() {
        let idx = id as usize;
        let cell = grid.cells[idx];
        if cell & 7 != 0 {
            continue;
        }
        let dir = (steps_and_dir & 7) as usize;
        assert!(dir > 0 && dir <= 4);
        //println!("idx={} steps_and_dir={} height={} steps={}", idx, steps_and_dir, height, steps);
        if idx == grid.start {
            return (steps_and_dir >> 3, part2, grid);
        }
        if cell == 16 && part2 == 0 {
            part2 = steps_and_dir >> 3;
        }
        grid.cells[idx] = cell | dir as u8;

        let d = (steps_and_dir & !7) + 8;
        let step_limit = cell - 8;
        for next_dir in 0..3 {
            let neighbor_idx = idx as i16 + grid.next[(dir+next_dir-1) as usize];
            let next_cell = grid.cells[neighbor_idx as usize];
            if next_cell & 7 == 0 && next_cell >= step_limit {
                deq.push_back([neighbor_idx as u16, d + grid.next_dirs[(dir+next_dir-1) as usize] as u16]);
            }
        }
    }
    (0,0,grid)
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
    display.2._show();
}
