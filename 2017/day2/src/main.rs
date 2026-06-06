//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

fn process(inp:String) -> (u32,u32)
{
    let mut p1:u32 = 0;
    let mut p2:u32 = 0;

    let lines = inp.split("\n");
    for li in lines {
        if li.len() == 0 {break; }
//        println!("li = {li}");
//        let sn:Vec<&str> = li.split("\t").collect();
        let mut nums:Vec<u32> = li.split("\t").map(|x| u32::from_str_radix(x,10).unwrap()).collect();
        nums.sort();
        p1 += nums[nums.len()-1]-nums[0];

        for a in 1..nums.len() {
            for b in 0..a {
                if nums[a] % nums[b] == 0 {
                    p2 += nums[a] / nums[b];
                    break;
                }
            }
        } 
    }
    return (p1,p2);
}

fn main() {
    //let args: Vec<String> = env::args().collect();

    let test = "5\t1\t9\t5
7\t5\t3
2\t4\t6\t8".to_string();

    assert!(process(test).0 == 18);
//    assert!(process("1122".to_string()) == (3,0));
//    assert!(process("1111".to_string()) == (4,0));
//    assert!(process("1234".to_string()) == (0,0));

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
    println!("Total time {} ns",devtime.time_in_nanos().unwrap());
}