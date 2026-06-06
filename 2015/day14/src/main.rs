//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

struct RainDeer {
    name: String,
    speed: i32,
    fly_time: i32,
    rest_time: i32,
    distance: i32,
    points: i32,
    flying: bool,
    time: i32,
}
impl RainDeer {
    fn new(name: &str, speed: i32, fly_time: i32, rest_time: i32) -> RainDeer {
        RainDeer {
            name: name.to_string(),
            speed: speed,
            fly_time: fly_time,
            rest_time: rest_time,
            distance: 0,
            points: 0,
            flying: true,
            time: 0,
        }
    }
    fn update(&mut self) {
        if self.flying {
            self.distance += self.speed;
            self.time += 1;
            if self.time == self.fly_time {
                self.flying = false;
                self.time = 0;
            }
        } else {
            self.time += 1;
            if self.time == self.rest_time {
                self.flying = true;
                self.time = 0;
            }
        }
    }
    
}

fn process(inp:String) -> (i32, i32, String, String)
{
    let lines = inp.split("\n").collect::<Vec<&str>>();
    let mut part1 = 0;
    let mut part1_deer = 0;
    let mut part2 = 0;
    let mut part2_deer = 0;
    let mut deers:Vec<RainDeer> = Vec::new();
    for line in lines {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let name = parts[0];
        let speed = parts[3].parse::<i32>().unwrap();
        let fly_time = parts[6].parse::<i32>().unwrap();
        let rest_time = parts[13].parse::<i32>().unwrap();
        deers.push(RainDeer::new(name, speed, fly_time, rest_time));
    }
    for _ in 0..2503 {
        let mut max_deer = 0;
        let mut max_dist = 0;
        let mut deer = 0;
        for raindeer in deers.iter_mut() {
            raindeer.update();
            if raindeer.distance > max_dist {
                max_dist = raindeer.distance;
                max_deer = deer;
            }
            deer += 1;
        }
        part1 = max_dist;
        part1_deer = max_deer;
        deers[max_deer].points += 1;
    }
    for deer in 0..deers.len() {
        if deers[deer].points > part2 {
            part2 = deers[deer].points;
            part2_deer = deer;
        }
    }
    (part1, part2, deers[part1_deer].name.clone(), deers[part2_deer].name.clone())
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2, name1, name2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1} ({name1})");
    println!("Part2 = {part2} ({name2})");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}