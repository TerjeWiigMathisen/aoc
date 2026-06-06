//use std::collections::HashMap;
//use std::io;
//use std::env;
//use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn process(e1:u8, e2:u8, rounds:usize) -> (i64,i64)
{
    let mut recipes:Vec<u8> = Vec::with_capacity(10000000);
    recipes.push(e1); recipes.push(e2);
    let mut elf1 = 0;
    let mut elf2 = 1;
    let mut p1 = -1;
    let mut p2:i64 = -1;
    let mut p2_target:Vec<u8> = format!("{rounds}").as_bytes().to_vec();
    for i in 0..p2_target.len() {
        p2_target[i] &= 15;
    }
    let p2_end = p2_target[p2_target.len()-1];
    let p2_end_in_1: bool = p2_end == 1;
    loop {
        let mut sum = recipes[elf1] + recipes[elf2];
        let mut possible1 = false;
        if sum >= 10 {
            recipes.push(1);
            sum -= 10;
            possible1 = p2_end_in_1 && recipes.len() > p2_target.len();
        }
        recipes.push(sum as u8);
        let possible0 = sum == p2_end;

        let step = |e| if e + 10 >= recipes.len() 
            {(e + recipes[e] as usize + 1) % recipes.len()} else {e + recipes[e] as usize + 1};
        elf1 = step(elf1);
        elf2 = step(elf2);
        if recipes.len() == rounds+10 {
            p1 = 0;
            for p in rounds..rounds+10 {
                p1 = p1*10 + recipes[p] as i64;
            }
            if p2 >= 0 {return (p1,p2);}
        }
        if p2 < 0 && recipes.len() >= p2_target.len() {
            let pos = recipes.len()-p2_target.len();
            let mut found;
            if possible1 {
                found = true;
                for i in 0..p2_target.len() {
                    if recipes[pos+i-1] != p2_target[i] {found = false; break;}
                }
                if found { 
                    p2 = pos as i64 - 1;
                    if p1 >= 0 {return (p1,p2);}
                }
            }
            if possible0 && p2 < 0 {
                found = true;
                for i in 0..p2_target.len() {
                    if recipes[pos+i] != p2_target[i] {found = false; break;}
                }
                if found { 
                    p2 = pos as i64;
                    if p1 >= 0 {return (p1,p2);}
                }
            }
        }
    }
}

fn process_str(e1:u8, e2:u8, rounds:&str) -> (i64,i64)
{
    let mut recipes:Vec<u8> = vec![1;21000000];
    let mut rlen = 2;
    recipes[0] = e1; 
    recipes[1] = e2;
    let mut elf1 = 0;
    let mut elf2 = 1;
    let mut p1 = -1;
    let mut p2:i64 = -1;
    let mut p2_target:Vec<u8> = rounds.as_bytes().to_vec();
    let p2len = p2_target.len();
    let mut skip[u8;10] = [p2len;10];
    for i in 0..p2len {
        let d = p2_target[i] & 15;
        p2_target[i] = d;
        skip[d as usize] = p2len-i-1;
    }

    let rounds10 = rounds.parse::<usize>().unwrap()+10;
    const BLOCKSIZE:usize = 1024*32;

    let step = |e| if e + 10 >= recipes.len() 
            {(e + recipes[e] as usize + 1) % recipes.len()} else {e + recipes[e] as usize + 1};

    for i in 2..40 {
        elf1 = step(elf1);
        elf2 = step(elf2);
        let mut sum = recipes[elf1] + recipes[elf2];
        if sum >= 10 {
            rlen += 1;
            sum -= 10;
        }
        recipes[rlen] = sum;
        rlen += 1;
    }
    let mut blockend = BLOCKSIZE;
    loop {
        let step1 = |e| if e + 10 >= recipes.len() 
            {(e + recipes[e] as usize + 1) - recipes.len()} else {e + recipes[e] as usize + 1};

        elf1 = step1(elf1);
        elf2 = step1(elf2);
        let mut sum = recipes[elf1] + recipes[elf2];
        if sum >= 10 {
            rlen += 1;
            sum -= 10;
        }
        recipes[rlen] = sum;
        rlen += 1;
        if rlen >= rounds10 {
            p1 = 0;
            for p in rounds10-10..rounds10 {
                p1 = p1*10 + recipes[p] as i64;
            }
            break;
        }
    }
    let mut blockstart = 0;
    let p2_end = p2_target[p2_target.len()-1];
    loop {
        let step1 = |e| if e + 10 >= recipes.len() 
            {(e + recipes[e] as usize + 1) - recipes.len()} else {e + recipes[e] as usize + 1};

        elf1 = step1(elf1);
        elf2 = step1(elf2);
        let mut sum = recipes[elf1] + recipes[elf2];
        if sum >= 10 {
            rlen += 1;
            sum -= 10;
        }
        recipes[rlen] = sum;
        rlen += 1;
        if rlen >= blockend {
            let mut i = blockstart + p2_len-1;
            loop {
                let last = recipes[i];
                if last == p2_end {
                    let mut found = true;
                    for j in 1..p2_len {
                        if recipes[i-p2_len+j] != p2_target[j] { found = false; break}
                    }
                    if found {
                        p2 = i-p2len;
                        return (p1,p2);
                    }
                }
                i += skip[last as usize];
                if i >= blockend {break;}
            }
            blockstart = blockend-10;
            blockend += BLOCKSIZE;
        }
    }
}

fn main() {
    assert!(process(3,7,9).0 == 5158916779);
    assert!(process(3,7,2018).0 == 5941429882);
    assert!(process(3,7,59414).1 == 2018);

    let fname = "input.txt"; // instead of args[1]
    let mut input = std::fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}
    println!("{input}");
    let myinp = input.parse::<usize>().unwrap();
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(25, |_| {process(3,7,myinp); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(3,7,myinp);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}