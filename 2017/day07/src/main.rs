use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

struct Tower {
    name:String,
    parent:usize,
    weight:i32,
    total_weight:i32,
    above:Vec<usize>,
    balanced:bool,
}

fn total_weight(tow:&mut Vec<Tower>, idx:usize)->i32
{
    if tow[idx].total_weight > 0 { return tow[idx].total_weight; }
    let mut w = tow[idx].weight;
    for a in tow[idx].above.clone() {
        w += total_weight(tow, a);
    }
    tow[idx].total_weight = w;
    return w;
}

fn is_balanced(tow:&mut Vec<Tower>, idx:usize)->bool
{
    // Is the tower balanced above?
    let weight_above = total_weight(tow, idx)-tow[idx].weight;
    let alen = tow[idx].above.len();
    for a in tow[idx].above.clone() {
        if weight_above != total_weight(tow, a)*alen as i32 { return false;}
    }
    return true;
}

fn find_imbalance(tow:&mut Vec<Tower>, idx:usize)->i32
{
    for t in 0..tow.len() {
        tow[t].balanced = is_balanced(tow, t);
    }
    // Start at root (which must be out of balance, find node where all children are OK
    if tow[idx].balanced { return 0; }

    for a in tow[idx].above.clone() {
        if !tow[a].balanced {
            let f = find_imbalance(tow, a);
            if f != 0 { return f; }
        }
    }
    // This is the point of imbalance, all above are OK
    let mut weights:Vec<i32> = vec![];
//    print!("{idx}:{}:{} +",tow[idx].total_weight, tow[idx].name);
    for a in tow[idx].above.clone() { 
        weights.push(tow[a].total_weight); 
//        print!(" {a}:{}:{}",tow[a].total_weight, tow[a].name);
    }
//    println!("\n");
    if weights.len() > 2 {
        weights.sort();
        let mut exc = weights[weights.len()-1];
        let mut ok = weights[0];
        if weights[0] != weights[1] {
            exc = weights[0];
            ok = weights[1];
        }
        for a in tow[idx].above.clone() {
            if tow[a].total_weight == exc {
                return tow[a].weight + (ok - exc);
            }
        }
    }
    return 0;
}

fn process(inp:String) -> (String,i32)
{
    let p2;
    let mut lines:Vec<&str> = inp.split("\n").collect();
    if lines.len() > 0 && lines[lines.len()-1].len() == 0 {
        lines.pop();
    }

    let mut tow:Vec<Tower> = vec![];
    let mut name2idx:HashMap<String,usize> = HashMap::<String,usize>::new();
    
    for idx in 0..lines.len() {
        let li:String = lines[idx].to_string();
        let parts:Vec<&str> = li.split(" -> ").collect();
        let base:Vec<&str> = parts[0].split(" (").collect();
        let nm = base[0].to_string();
        name2idx.insert(nm.clone(), idx);
        let mut w:String = base[1].to_string();
        w.pop(); // Get rid of ')'
        let t = Tower {
            name: nm,
            parent: usize::MAX,
            weight: w.parse::<i32>().unwrap(), 
            total_weight: -1,
            above:vec![],
            balanced: false,
        };
        tow.push(t);
    }

    // Process list above:
    for idx in 0..lines.len() {
        let li:String = lines[idx].to_string();
        let parts:Vec<&str> = li.split(" -> ").collect();
        if parts.len() > 1 {
            let alist:Vec<&str> = parts[1].split(", ").collect();
            for a in alist {
                let ai :usize = name2idx[a.clone()];
                tow[idx].above.push(ai);
                tow[ai].parent = idx;
            }
        }
    }
    let mut idx = 0;
    while tow[idx].parent != usize::MAX {
        idx = tow[idx].parent;
    }
    p2 = find_imbalance(&mut tow, idx);
    return (tow[idx].name.clone(), p2);
}

fn main() {

    assert!(process("pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)".to_owned()) == ("tknk".to_string(), 60));
    let fname = "input.txt"; // instead of args[1]
    let input = fs::read_to_string(fname).expect("Error readin input file");
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(25, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1,part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}