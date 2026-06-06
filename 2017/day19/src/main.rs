//use std::io;
//use std::env;
use std::fs;
// use std::collections::HashMap;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

fn get(lines:&Vec<&str>, x:i32, y:i32) -> u8
{
    if y < 0 || y >= lines.len() as i32 || x < 0 || x >= lines[y as usize].len() as i32 { return ' ' as u8;}
    return lines[y as usize].as_bytes()[x as usize];
}

fn process(inp:String) -> (String,i32)
{
    let mut p1:String = "".to_owned();
    let mut p2 = 0;
    let input = inp.clone();
    let lines:Vec<&str> = input.split("\n").collect();

    // Find the starting point:
    let mut x = lines[0].find('|').unwrap() as i32;
    let mut y = 0;
    let mut dir = 3; // Starting down!
    let dx:Vec<i32> = vec![1,0,-1,0];
    let dy:Vec<i32> = vec![0,-1,0,1];
    let cont = vec!['-','|','-','|'];
'outer: loop {
'inner:  loop {
            x += dx[dir];
            y += dy[dir];
//            let c = lines[y as usize].as_bytes()[x as usize];
            let c = get(&lines, x, y);
            p2 += 1;
            if c == '+' as u8 { // Change direction
                for d in [1,3] {
                    let dd = (dir+d) & 3;
                    let cn = get(&lines, x + dx[dd], y + dy[dd]);
                    if cn == cont[dd] as u8 || cn == '+' as u8 {
                        dir = dd;
                        continue 'inner;
                    }
                }
                for d in [1,3] {
                    let dd = (dir+d) & 3;
                    let cn = lines[(y + dy[dd]) as usize].as_bytes()[(x + dx[dd]) as usize];
                    if (cn >= 'A' as u8 ) && (cn <= 'Z' as u8) {
                        dir = dd;
                        continue 'inner;
                    }
                }
            }
            else if (c >= 'A' as u8 ) && (c <= 'Z' as u8) {
                p1.push(c as char);
//                println!("Found char: {p1}");
            }
            else if c == ' ' as u8 {
                break 'outer;
            }
            else if c != '-' as u8 && c != '|' as u8 {
                println!("Bad char {} at line {y} pos {x}", c);
                break 'outer;
            }
        }
    }
    return (p1,p2);
}

fn main() {

    assert!(process(
"     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
".to_owned()) == ("ABCDEF".to_string(),38));
    let fname = "input.txt"; // instead of args[1]
    let input = fs::read_to_string(fname).expect("Error readin input file");
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(25, |_| {
        process(input.clone());
    });
    bench_result.print_stats();

    devtime.start();
    let (part1,part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}