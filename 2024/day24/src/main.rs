//use std::collections::VecDeque;
use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
use substring::Substring;

pub fn evaluate(xydata:Vec<&str>, logic:Vec<&str>) -> u64 {
    let mut operand = HashMap::<&str,usize>::new();
    let mut regs:Vec<u8> = Vec::new();
    
    for l in xydata.iter() {
        let r = l.substring(0,3);
        let v = l.as_bytes()[5] - b'0';
        operand.insert(r, regs.len());
        regs.push(v);
    }
    for iregs in operand.into_keys() {
        println!("{} = {}", iregs, operand[iregs]);
    }
    for l in logic.iter() {
        let parts = split_string(l, " ");
        assert!(parts.len() == 4);
        let a = parts[0].as_str();
        let op = parts[1].as_str();
        let b = parts[2].as_str();
        let res = parts[3].as_str();
        println!("{} = {} {} {}", res, a, op, b);
        if !operand.contains_key(a) {
            operand.insert(a, regs.len());
            regs.push(0xff);
        }
        if !operand.contains_key(b) {
            operand.insert(b, regs.len());
            regs.push(0xff);
        }
        if !operand.contains_key(res) {
            operand.insert(res, regs.len());
            regs.push(0xff);
        }    
    }
    let mut it = 0;
    
    loop {
        it += 1;
        let mut found = 0;
        for l in logic.iter() {
            let myl = l.clone();
            let parts = split_string(myl, " "); // myl.split_ascii_whitespace().collect();
            let a = parts[4].as_str(); let ai = operand[a];
            let op = parts[1].as_str();
            let b = parts[2].as_str(); let bi = operand[b];
            let res = parts[3].as_str(); let ri = operand[res];

            if regs[ri] == 0xff && regs[ai] != 0xff && regs[bi] != 0xff {
                let mut r = 0;
                match op {
                    "AND" => {r = regs[ai] & regs[bi];}
                    "OR" => {r = regs[ai] | regs[bi];}
                    "XOR" => {r = regs[ai] ^ regs[bi];}
                    _ => { assert!("Bad operation!" == "")},
                }
                regs[ri] = r;
                println!("{} = {} {} {}", res, a, op, b);
                found += 1;
            }
        }
        if found == 0 {
            println!("No valid operation found!");
            break;
        }
    }
    let mut part1 = 0;
    for z in 0..=45 {
        let lo = z % 10;
        let hi = z / 10;
        let keys = format!("z{hi}{lo}"); 
        let key = keys.as_str();
        part1 |= (operand[key] as u64) << z;
    }
    part1
}

fn split_string(input: &str, delimiter: &str) -> Vec<String> {
    input.split(delimiter).map(String::from).collect()
}

fn process(inp:String) -> (u64, u64)
{
    let inputs = split_string(&inp, "\n\n"); //inp.as_str().split("\n\n").map(String::from).collect();
    let (xy, lo) = (inputs[0].clone(), inputs[1].clone());
    let xydata = split_string(&xy, "\n"); //xy.split("\n").collect();
    for _x in xydata.iter() {
        //println!("{}", x);
    }
    let logic = split_string(&lo, "\n"); //lo.split("\n").collect();
    //let logic ):Vec<&str> = lo.split("\n").collect();
    for _l in logic.iter() {
        //println!("{}", l);
    }
    let part1 = evaluate(xydata,logic);
    (part1, 0)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}