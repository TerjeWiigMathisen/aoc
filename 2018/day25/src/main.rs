//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use std::rt::panic_count::set_always_abort;
use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

#[derive(Debug,Clone,Copy)]
struct P4 {
    x:i8,
    y:i8,
    z:i8,
    t:i8,
    set_number:usize,
}
impl P4 {
    fn manhattan_dist4(self, p:&P4)->usize
    {
        ((self.x-p.x).abs() + (self.y-p.y).abs() + (self.z-p.z).abs() + (self.t-p.t).abs()) as usize
    }
}

fn flatten(points:&mut [P4], idx:usize)->usize
{
    let mut i = points[idx].set_number;
    let mut parent = idx;
    if i != idx {
        let mut ip = points[i].set_number;
        while ip != i {
            i = ip;
            ip = points[i].set_number;
        }
        parent = ip;
        i = idx;
        while i != parent {
            ip = points[i].set_number;
            points[i].set_number = parent;
            i = ip;
        }
    }
    parent
}

fn flatten_to(points:&mut [P4], idx:usize, parent:usize)->usize
{
    if idx != parent {
        let mut i = idx;
        while i != parent {
            let ip = points[i].set_number;
            points[i].set_number = parent;
            i = ip;
        }
    }
    parent
}

fn discrete_sets(inp:&str) -> usize
{
//    let mut lines:Vec<String> = inp.split("\n").map(|s| s.to_string()).collect();
    let p = parser!(lines(x:i8 "," y:i8 "," z:i8 "," t:i8 => P4{x,y,z,t,set_number:usize::MAX}));
    let mut points = p.parse(&inp).unwrap();
    let mut sets = 0;
    for i in 0..points.len() {
        let mut parent = i;
        sets += 1;
        if points[i].set_number != usize::MAX {
            parent = flatten(&mut points,i);
            sets -= 1;
        }
        points[i].set_number = parent;

        for j in i+1..points.len() {
            if /* i != j && */ points[i].manhattan_dist4(&points[j]) <= 3 {
                if points[j].set_number == usize::MAX {
                    points[j].set_number = parent;
                }
                else { // Merge two sets if they are disjunct
                    let jparent = flatten(&mut points, j);
                    if jparent != parent { // Two different sets to be merged, pick the lowest/oldest set #!
                        if jparent < parent {
                            parent = flatten_to(&mut points, i, jparent);
                        }
                        else { //if jparent > parent {
                            parent = flatten_to(&mut points, j, parent);
                        }
                        sets -= 1;
                    }
                }
            }
        }
    }
    sets
}

fn main() {
    assert!(discrete_sets(
"-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0") == 4);

    let fname = "input.txt"; // instead of args[1]
    let input = fs::read_to_string(fname).expect("Error readin input file");

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(25, |_| { discrete_sets(&input); }); bench_result.print_stats();

    devtime.start();
    let part1 = discrete_sets(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}