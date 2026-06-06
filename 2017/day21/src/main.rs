//use std::io;
//use std::env;
use std::fs;
//use substring::Substring;
use std::collections::HashMap;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

fn rotate(rot:i32, pat:&Vec<u8>)->Vec<u8>
{
    let mut src = pat.clone();
    let mut tgt = pat.clone();
    let rot2:Vec<u8> = vec![1,4,2,0,3];
    let rot3:Vec<u8> = vec![2,6,10,3,1,5,9,7,0,4,8];
    for _ in 0..(rot/90) {
        if src.len() == 5 {
            for i in 0..5 {
                tgt[i] = src[rot2[i] as usize];
            }
        }
        else if src.len() == 11  {
            for i in 0..11 {
                tgt[i] = src[rot3[i] as usize];
            }
        }
        else { 
            println!("Bad matrix length: {}", src.len()); 
        }
        src = tgt.clone();
    }
    return tgt;
}

fn flip(src:&Vec<u8>)->Vec<u8>
{
    let mut tgt = src.clone();
    let flp2:Vec<u8> = vec![1,0,2,4,3];
    let flp3:Vec<u8> = vec![2,1,0,3,6,5,4,7,10,9,8];
    if src.len() == 5 {
        for i in 0..5 {
            tgt[i] = src[flp2[i] as usize];
        }
    }
    else if src.len() == 11  {
        for i in 0..11 {
            tgt[i] = src[flp3[i] as usize];
        }
    }
    else { 
        println!("Bad matrix length: {}", src.len());
    }
    return tgt;
}

fn process(inp:String, iterations:usize) -> i32
{
    let mut count:i32 = 0;
    let input = inp.clone();
    let lines:Vec<&str> = input.split("\n").collect();
    let mut rules:HashMap<Vec<u8>,Vec<u8>> = HashMap::new();

    for li in lines {
//        println!("li={li}");
        let parts:Vec<&str> = li.split(" => ").collect();
        let pattern:Vec<u8> = parts[0].to_string().as_bytes().to_vec().to_owned();
        let target:Vec<u8> = parts[1].to_string().as_bytes().to_vec().to_owned();
        rules.insert(pattern.clone(),target.clone());
        rules.insert(rotate(90,&pattern).clone(),target.clone());
        rules.insert(rotate(180,&pattern).clone(),target.clone());
        rules.insert(rotate(270,&pattern).clone(),target.clone());
        let flipped = flip(&pattern);
        rules.insert(flipped.clone(),target.clone());
        rules.insert(rotate(90,&flipped).clone(),target.clone());
        rules.insert(rotate(180,&flipped).clone(),target.clone());
        rules.insert(rotate(270,&flipped).clone(),target.clone());
    }
    let mut board:Vec<Vec<u8>> = 
".#.
..#
###".to_string().split("\n").map(|x| x.as_bytes().to_owned()).collect();
    for _ in 0..iterations {
        let ssize:usize = board.len();
        let sstep:usize;
        let tsize:usize; 
        let tstep:usize;
        if ssize & 1 == 0 {
            sstep = 2;
            tsize = ssize / 2 * 3;
            tstep = 3;
        }
        else {
            sstep = 3;
            tsize = ssize / 3 * 4;
            tstep = 4;
        }
        let mut tgt:Vec<Vec<u8>> = vec![vec![' ' as u8;tsize];tsize];
        let mut tx = 0;
        let mut ty = 0;
        count  = 0;
        for y in (0..ssize).step_by(sstep) {
            for x in (0..ssize).step_by(sstep) {
                let mut src:Vec<u8> = vec![];
                for i in 0..sstep {
                    for j in 0..sstep {
                        src.push(board[y+i][x+j]);
                    }
                    src.push('/' as u8);
                }
                src.pop();
                if !rules.contains_key(&src) {
                    println!("Src pattern not found: {}",src[0]);
                    return 0;
                }
                let pat = rules.get(&src).unwrap().clone();
                let mut xt = tx; let mut yt = ty;
                for i in 0..pat.len() {
                    let c = pat[i];
                    if c == '/' as u8 {
                        yt += 1;
                        xt = tx;
                    }
                    else {
                        tgt[yt][xt] = c;
                        xt += 1;
                        if c == '#' as u8 {count += 1;}
                    }
                }
                tx += tstep;
            }
            tx = 0;
            ty += tstep;
        }
        board = tgt.clone();
    }
    return count;
}

fn main() {

    assert!(process(
"../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#".to_owned(),2) == 12);
    let fname = "input.txt"; // instead of args[1]
    let input = fs::read_to_string(fname).expect("Error readin input file");
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| {
        process(input.clone(),18);
    });
    bench_result.print_stats();

    devtime.start();
    let part1 = process(input.clone(),5);
    let part2 = process(input.clone(),18);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}