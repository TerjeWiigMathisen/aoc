//aoc 2022 day12
// Fastest run: Surface Pro 8 29.2 (u8), 35.3 (u32) us
//                       Acer 14.8 us

//use devtimer::DevTime;
use devtimer::run_benchmark;
//use std::arch::x86_64::*;
#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
struct Span {
    b:usize,
    e:usize,
}

struct Spans {
    spans:Vec<Span>,
}

impl Span {
    fn new(start, slutt) -> Span {
        Span { b: start, e: slutt }
    }
}

impl Spans {
    fn new() -> Spans {
        Spans { spans: Vec::new() }
    }
    fn add(&mut self, span: Span) {
        self.spans.push(span);
    }
    fn merge(&mut self) {
        self.spans.sort_by_key(|s| s.b);
        let mut i = 0;
        let mut j = 1;
        while j < self.spans.len() {
            if self.spans[i].e < self.spans[j].b {
                i += 1;
                self.spans[i] = self.spans[j];
                j += 1;
            }
            else {
                if self.spans[i].e < self.spans[j].e {
                    self.spans[i].e = self.spans[j].e;
                }
                j += 1;
            }
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
        //println!("idx={} steps_and_dir={} height={} steps={}", idx, steps_and_dir, height, steps);
        if idx == grid.start {
            return (steps_and_dir >> 3, part2, grid);
        }
        if cell == 16 && part2 == 0 {
            part2 = steps_and_dir >> 3;
        }
        grid.cells[idx] = cell | (steps_and_dir & 7) as u8;

        let d = (steps_and_dir & !7) + 8;
        let step_limit = cell - 8;
        if grid.cells[idx+1] >= step_limit {
            deq.push_back([(idx+1) as u16, d+1]);
        }
        if grid.cells[idx-1] >= step_limit {
            deq.push_back([(idx-1) as u16, d+2]);
        }
        if grid.cells[idx-grid.stride] >= step_limit {
            deq.push_back([(idx-grid.stride) as u16, d+3]);
        }
        if grid.cells[idx+grid.stride] >= step_limit {
            deq.push_back([(idx+grid.stride) as u16, d+4]);
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
//    display.2.show();
}
