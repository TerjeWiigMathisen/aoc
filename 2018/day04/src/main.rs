// Fastest run 965.7 us
use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

struct Event {
    _year:u16,
    _mon:u8,
    _day:u8,
    _hh:u8,
    mm:u8,
    msg:String,
}

fn process(inp:&str) -> (i64,i64)
{
//    let mut lines:Vec<String> = inp.split("\n").map(|s| s.to_string()).collect();
    let mut lines:Vec<&str> = inp.lines().collect();
    lines.sort();
    let p = parser!("[" y:u16 "-" mo:u8 "-" da:u8 " " h:u8 ":" m:u8 "] " ms:string(any_char+) =>
        Event {_year:y, _mon:mo, _day:da, _hh:h, mm:m, msg:ms});
    let mut guard_to_sleep_index:HashMap<u16, u8> = HashMap::new();
    let mut sleep:Vec<Vec<u16>> = vec![];
    let mut idx2guard:Vec<u16> = vec![];
    let mut sbegin:usize = 60;
    let mut idx:usize = 0;
    let _loglines = lines.len();
    for li in lines {
        let ev = p.parse(&li).unwrap();
        //let mut key:u16 = 0;
        if ev.msg[0..5] == "Guard".to_string() {
//            println!("{}",ev.msg.substring(7,ev.msg.len()-13));
//            let key = ev.msg.substring(7,ev.msg.len()-13).parse::<u16>().unwrap();
            let key = ev.msg[7..ev.msg.len()-13].parse::<u16>().unwrap();
            if !guard_to_sleep_index.contains_key(&key) {
                guard_to_sleep_index.insert(key,sleep.len() as u8);
                sleep.push(vec![0;60]);
                idx2guard.push(key);
            }
            idx = guard_to_sleep_index[&key] as usize;
            sbegin = 60;
        }
        else if ev.msg == String::from("falls asleep") {
            sbegin = ev.mm as usize;
        } 
        else if ev.msg == String::from("wakes up") {
            let send = ev.mm as usize;
            for m in sbegin..send {
                sleep[idx][m] += 1;
            }
        } 
    }
    // Minutes asleep per guard:
    let mut maxsum = 0;
    let mut max_minute_asleep = 0;
    let mut maxguard = 0;
    let mut maxmin = 0;
    let mut maxming = 0;
    //println!("{} total guards, {} log length",idx2guard.len(), loglines);
    for g in 0..sleep.len() {
        let mut sum:u32 = 0;
        for m in 0..60 {
            let s = sleep[g][m];
            sum += s as u32;
            if s > maxmin {
                maxmin = s;
                maxming = g;
                max_minute_asleep = m;
            }
        }
        if sum > maxsum {
            maxsum = sum;
            maxguard = g;
        }
    }
    let p2:i64 = max_minute_asleep as i64 * idx2guard[maxming] as i64;
//    println!("Guard with index {maxguard} slept {maxssum} minutes in total");
    let mut maxsleep = 0;
    maxmin = 0;
    for m in 0..60 {
        let s = sleep[maxguard][m];
        if s > maxsleep {
            maxsleep = s;
            maxmin = m as u16;
        }
    }
    let p1 = (idx2guard[maxguard] as i64) * (maxmin as i64);
//    println!("Guard {p1} slept the most ({maxsleep}) during minute {maxmin}");
    (p1, p2)
}


fn main() {
    assert!(process(
"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up") == (240,4455));

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == b'\n' {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}