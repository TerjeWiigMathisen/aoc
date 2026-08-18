//aoc 2022 day14
// Fastest run: Surface Pro 8
//                       Acer

//use devtimer::DevTime;
use devtimer::run_benchmark;
//use std::arch::x86_64::*;
#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
struct Span {
    b:usize,
    e:usize,
}

impl Span {
    fn new(start, slutt) -> Span {
        Span { b: start, e: slutt }
    }
}

struct Spans {
    spans:Vec<Span>,
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
            if self.spans[i].e < self.spans[j].b { // no overlap, finished with this
                i += 1;
                self.spans[i] = self.spans[j];
                j += 1;
            }
            else {  // Partly or completely overlapping
                if self.spans[i].e < self.spans[j].e {
                    self.spans[i].e = self.spans[j].e;
                }
                j += 1;
            }
        }
        spans.truncate(j);
    }
    
    fn parse(inp:&str) -> Spans {
        let mut spans = Spans::new();
        for line in inp.lines() {
            let parts: Vec<&str> = line.split(' -> ').collect();
            let points: Vec<(usize, usize)> = parts.iter().filter_map(|p| {
                let coords: Vec<&str> = p.split(',').collect();
                if coords.len() == 2 {
                    if let (Ok(x), Ok(y)) = (coords[0].parse::<usize>(), coords[1].parse::<usize>()) {
                        return Some((x, y));
                    }
                }
                None
            }).collect();
            let mut prev_point = points[0];
            for &point in points.iter().skip(1) {
                let (x0, y0) = prev_point;
                let (x1, y1) = point;
                if x0 == x1 {
                    let (start_y, end_y) = if y0 < y1 { (y0, y1) } else { (y1, y0) };
                    for y in start_y..=end_y {
                        while y >= spans.spans.len() {
                            spans.spans.push(Vec::new());
                        }
                        spans.spans[y].add(Span::new(x0, x0+1));
                    }
                } else if y0 == y1 {
                    let (start_x, end_x) = if x0 < x1 { (x0, x1) } else { (x1, x0) };
                    while y0 >= spans.spans.len() {
                        spans.spans.push(Vec::new());
                    }
                    spans.spans[y0].add(Span::new(start_x, end_x+1));
                }
                prev_point = point;
            }
        }
        spans
    }
}

#[inline(never)]
fn process(inp:&str) -> usize
{
    let mut spans = Spans::new(inp);
    let mut part2 = 0;
    let mut left = 500;
    let mut right = 501;
    let prev = Spans{spans:vec![Span{left,right};1]}
    part2 += 1;
    for y in 1..spans.len() {
        let mut curr = spans[y].clone();
    }
    part2
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
