//use std::io;
//use std::env;
use std::fs;
use std::collections::VecDeque;
//use std::collections::HashMap;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

fn run2(reg:&mut Vec<i64>, recvq:&mut VecDeque<i64>, ipp:i64, lines:Vec<&str>)->(i64, VecDeque<i64>)
{
    let mut sendq:VecDeque<i64> = vec![].into();
    let mut ip:i64 = ipp;
    loop {
        let li = lines[ip as usize];
        let words:Vec<&str> = li.split(" ").collect();
        let w1 = words[1].as_bytes()[0] as i64 - 'a' as i64;
        let mut r = 0;
        let mut jgz = false;
        if w1 >= 0 {
            r = w1 as usize;
            jgz = reg[r] > 0;
        }
        else {
            let r1i = words[1].parse::<i64>().unwrap();
            jgz = r1i > 0;
        }
        let mut val = 0;
        if words.len() > 2 {
            let w2 = words[2].as_bytes()[0] as i64 - 'a' as i64;
            val = if w2 >= 0 { reg[w2 as usize] } else { words[2].parse::<i64>().unwrap() }
        }
        // match words[0] {
        //     "set" => reg[r] = val,
        //     "add" => reg[r] += val,
        //     "mul" => reg[r] *= val,
        //     "mod" => reg[r] %= val,
        //     "jgz" => if jgz {ip += val - 1;},
        //     "snd" => sendq.push_back(reg[r]),
        //     "rcv" => if recvq.len() == 0 {return (ip,sendq); } else {
        //         reg[r] = recvq.pop_front().unwrap();
        //     },
        //     _ => println!("Bad instruction {li}"),
        // }
        match words[0].as_bytes()[1] {
            b'e' => reg[r] = val,
            b'd' => reg[r] += val,
            b'u' => reg[r] *= val,
            b'o' => reg[r] %= val,
            b'g' => if jgz {ip += val - 1;},
            b'n' => sendq.push_back(reg[r]),
            b'c' => if recvq.len() == 0 {return (ip,sendq); } else 
                { reg[r] = recvq.pop_front().unwrap();},
            _ => println!("Bad instruction {li}"),
        }
        ip += 1;
        if ip as usize >= lines.len() {
            println!("Bad IP: {ip}");
            return (ip,sendq);
        }
    }
}

fn process2(inp:String) -> i64
{
    let lines:Vec<&str> = inp.split("\n").collect();

    let mut reg0:Vec<i64> = vec![0;26]; //reg1[(b'p' - b'a') as usize] = 0;
    let mut reg1:Vec<i64> = vec![0;26]; reg1[(b'p' - b'a') as usize] = 1;

    let mut rcv0:VecDeque<i64> = vec![].into();
    let mut rcv1:VecDeque<i64> = vec![].into();

    let mut ip0:i64 = 0;
    let mut ip1:i64 = 0;

    let mut p2 = 0;
    loop {
        let r0:VecDeque<i64>;
        let r1:VecDeque<i64>;
        (ip0,r1) = run2(&mut reg0, &mut rcv0, ip0, lines.clone());
        
        rcv1.extend(r1);
        (ip1,r0) = run2(&mut reg1, &mut rcv1, ip1, lines.clone());
        p2 += r0.len() as i64;
        rcv0.extend(r0);

        if rcv0.len() == 0 && rcv1.len() == 0 { return p2; }
    }
}

fn process(inp:String) -> i64
{
    let mut reg:Vec<i64> = vec![0;26];
    let mut frequency = 0;

    let lines:Vec<&str> = inp.split("\n").collect();
    let mut ip:i64 = 0;
    loop {
        let li = lines[ip as usize];
        if li.len() == 0 {break; }
        let words:Vec<&str> = li.split(" ").collect();
        let r:usize = words[1].as_bytes()[0] as usize - 'a' as usize;
        let mut val = 0;
        if words.len() > 2 {
            let w2 = words[2].as_bytes()[0] as i64 - 'a' as i64;
            if w2 >= 0 {
                val = reg[w2 as usize];
            }
            else {
                val = words[2].parse::<i64>().unwrap();
            }
        }
        match words[0] {
            "set" => reg[r] = val,
            "snd" => frequency = reg[r],
            "add" => reg[r] += val,
            "mul" => reg[r] *= val,
            "mod" => reg[r] %= val,
            "rcv" => if reg[r] != 0 {return frequency;},
            "jgz" => if reg[r] > 0 {ip += val - 1;},
            _ => println!("Bad instruction {li}"),
        }
        ip += 1;
        if ip < 0 || ip as usize >= lines.len() {
            println!("Bad IP: {ip}");
            break;
        }
    }
    return frequency;
}

fn main() {

let test =
"set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2".to_owned().to_string();
    assert!(process(test) == 4);
    let fname = "input.txt"; // instead of args[1]
    let input = fs::read_to_string(fname).expect("Error readin input file");
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(25, |_| {
        process(input.clone());
        process2(input.clone());
    });
    bench_result.print_stats();

    devtime.start();
    let part1 = process(input.clone());
    let part2 = process2(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}