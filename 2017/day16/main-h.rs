//use std::io;
//use std::env;
use std::fs;
use std::collections::HashMap;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
use substring::Substring;

fn process(len:usize, inp:String) -> String
{
    let mut d:Vec<u8> = vec![0;len];
    let mut t = d.clone();
    for i in 0..len {
        d[i] = (i + 'a' as usize) as u8;
    }
    let cmds = inp.split(",");

    for c in cmds {
        let com = c.substring(0,1);
        let tail = c.substring(1,c.len());
        if com == "s".to_string() { // Spin
            let l = tail.parse::<usize>().unwrap();
            let mut j = 0;
            for i in (len-l)..len {
                t[j] = d[i];
                j += 1;
            }
            for i in 0..(len-l) {
                t[j] = d[i];
                j += 1;
            }
            d = t.clone();
        }
        else if com == "x".to_string() { // eXchange
            let parts:Vec<&str> = tail.split("/").collect();
            let a = parts[0].parse::<usize>().unwrap();
            let b = parts[1].parse::<usize>().unwrap();
            let t = d[a]; d[a] = d[b]; d[b] = t;
        }
        else if com == "p".to_string() {
            let ca = tail.as_bytes()[0];
            let cb = tail.as_bytes()[2];
            let (mut a,mut b) = (0,0);
            for i in 0..len {
                if d[i] == ca { a = i;}
                if d[i] == cb { b = i;}
            }
            let t = d[a]; d[a] = d[b]; d[b] = t;
        }
        else { println!("Bad command:{}", c.to_string());}
    }
    return String::from_utf8_lossy(&d).into_owned();
}

fn process2(inp:String) -> String
{
    let mut d:Vec<u8> = vec![0;16];
    let mut t = d.clone();
    for i in 0..16 {
        d[i] = (i + 'a' as usize) as u8;
    }
    let mut iter = 0;
    let mut hm:HashMap<Vec<u8>,usize> = HashMap::new();
    let mut searchloop = true;
    loop {
        if searchloop {
    //        if (iter & 0xffff) == 0 { print!("{iter}\r");}
            if hm.contains_key(&d) {
                let prev = hm[&d];
                println!("Found repeat after {iter} iterations, originally found at {prev}");
                let llen = iter - prev;
                let skip = (1000000000-iter) / llen * llen;
                iter += skip;
                searchloop = false;
            }
            hm.insert(d.clone(),iter);
        }
        if iter >= 1000000000 {
            break;
        }

        let cmds = inp.split(",");

        for c in cmds {
            let com = c.substring(0,1);
            let tail = c.substring(1,c.len());
            if com == "s".to_string() { // Spin
                let l = tail.parse::<usize>().unwrap();
                let mut j = 0;
                for i in (16-l)..16 {
                    t[j] = d[i];
                    j += 1;
                }
                for i in 0..(16-l) {
                    t[j] = d[i];
                    j += 1;
                }
                d = t.clone();
            }
            else if com == "x".to_string() { // eXchange
                let parts:Vec<&str> = tail.split("/").collect();
                let a = parts[0].parse::<usize>().unwrap();
                let b = parts[1].parse::<usize>().unwrap();
                let t = d[a]; d[a] = d[b]; d[b] = t;
            }
            else if com == "p".to_string() {
                let ca = tail.as_bytes()[0];
                let cb = tail.as_bytes()[2];
                let (mut a,mut b) = (0,0);
                for i in 0..16 {
                    if d[i] == ca { a = i;}
                    if d[i] == cb { b = i;}
                }
                let t = d[a]; d[a] = d[b]; d[b] = t;
            }
            else { println!("Bad command:{}", c.to_string());}
        }
        iter += 1;
    }
    return String::from_utf8_lossy(&d).into_owned();
}

fn main() {

    assert!(process(5, "s1,x3/4,pe/b".to_owned()) == "baedc".to_string());
    let fname = "input.txt";
    let input = fs::read_to_string(fname).expect("Error readin input file");
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1, |_| {
        process(16, input.clone());
    });
    bench_result.print_stats();

    devtime.start();
    let part1 = process(16, input.clone());
    let part2 = process2(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}