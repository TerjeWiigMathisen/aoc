// Fastest run: 7.9 us
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

fn process(test:&str) -> (i32,i32)
{
    let chars = test.as_bytes();
    let charlen = chars.len();
    let mut p1 = 0;
    let mut p2 = 0;
    let mut nest = 0;
    let mut i = 0;
    while i < charlen {
        let mut c = chars[i];
        i += 1;
        if c == b'{' {
            nest += 1;
            continue;
        }
        if c == b'<' { // Start garbage
            while c != b'>' {
                c = chars[i];
                i += 1;
                p2 += 1;
                while c == b'!' { // Ignore next char 
                    i += 1;
                    c = chars[i];
                    i += 1;
                } 
            }
            p2 -= 1;
            continue;
        }
        if c == b'}' {
            p1 += nest;
            nest -= 1;
        }
    }
    assert!(nest == 0);
    return (p1,p2);
}

fn main() {
    let test0 = "{{{}}}";
    assert!(process(test0).0 == 6);
    let test1 = "{{},{}}";
    assert!(process(test1).0 == 5);
    let test = "{{<a!>},{<a!>},{<a!>},{<ab>}}";
    assert!(process(test).0 == 3);
    let bang = "<{o\"i!a,<{i<a>";
    assert!(process(bang).1 == 10);
    let fname = "input.txt";
    let input = fs::read_to_string(fname).expect("Error readin input file");

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1,part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}