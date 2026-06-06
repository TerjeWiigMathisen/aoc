//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

struct Particle {
    x:i32, y:i32,
    vx:i32, vy:i32,
}

fn process(inp:&str) -> (String,i64)
{
//    let mut lines:Vec<String> = inp.split("\n").map(|s| s.to_string()).collect();
    let p = parser!(lines(
        {"position=< ","position=<"} x0:i32 {",  ",", "} y0:i32 
        {"> velocity=< ","> velocity=<"} vx0:i32 {",  ",", "} vy0:i32 ">" =>
        Particle {x:x0, y:y0, vx:vx0, vy:vy0}));
    let pt:Vec<Particle> = p.parse(&inp).unwrap().into_iter().collect();
    let mut p1 = String::from("");
    let mut p2 = 0;
//    let mut fsum = 0.0;
    let mut max:f64 = -1e9;
    let mut min:f64 = 1e9;
    for i in 0..pt.len() {
        let p = &pt[i];
        if p.vy.abs() == 1 {
            let scaled = p.y as f64;
//        fsum += scaled;
            if scaled > max {max = scaled;}
            if scaled < min {min = scaled;}
        }
    }
//    fsum = fsum / pt.len() as f64; // Average center
    let center = (max+min)*0.5;
    let seconds = (max - center + 0.5) as i64;
    let mut min_y_spread = i32::MAX;

    for s in seconds-6..seconds+6 {
        let mut miny = i32::MAX;
        let mut maxy = i32::MIN;
        let mut minx = i32::MAX;
        let mut maxx = i32::MIN;
        for i in 0..pt.len() {
            let p = &pt[i];
            let y = p.y+p.vy*s as i32;
            if y < miny {miny = y;}
            if y > maxy {maxy = y;}
            let x = p.x+p.vx*s as i32;
            if x < minx {minx = x;}
            if x > maxx {maxx = x;}
        }
        if maxy - miny < 15 {
            if maxy-miny+1 < min_y_spread {
                min_y_spread = maxy-miny+1;
                p2 = s;
                let mut display:Vec<Vec<u8>> = vec![vec![32;(maxx-minx+1) as usize];(maxy-miny+1) as usize];
                for i in 0..pt.len() {
                    let p = &pt[i];
                    let y = p.y+p.vy*s as i32;
                    let x = p.x+p.vx*s  as i32;
                    display[(y-miny) as usize][(x-minx) as usize] = '#' as u8;
                }
                println!("After {s} seconds");
                p1 = String::from("\n");
                for y in miny..=maxy {
                    let line:String = display[(y-miny) as usize].iter().
                        map(|x| char::from(*x)).collect();
//                    println!("{line}");
                    p1 = p1 + &line + "\n";
                }
            }
            else { return (p1,p2); }
        }
    }
    return (p1, p2);
}


fn main() {
    assert!(process(
"position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>").1 == 3);

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { process(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}