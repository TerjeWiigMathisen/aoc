use std::vec;
use devtimer::DevTime;
use devtimer::run_benchmark;

fn process(input: &str) -> (u32, u32)
{
    let nums = input.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let mut last = vec![u32::MAX; 30000000];
    
    let mut turn = 0;
    let mut l:usize = 0;
    for i in nums {
        turn += 1;
        l = i as usize;
        last[l] = turn;
    }
    
    let mut prev = vec![u32::MAX; 30000000];
    
    while turn < 2020 {
        let c = turn;
        let p = prev[l];
        l = 0;
        if p != u32::MAX {
            l = (c - p) as usize;
        }
        turn += 1;
        prev[l] = last[l];
        last[l] = turn;
    }
    let part1 = l as u32;

    while turn < 30000000 {
        let c = turn;
        let p = prev[l];
        l = 0;
        if p != u32::MAX {
            l = (c - p) as usize;
        }
        turn += 1;
        prev[l] = last[l];
        last[l] = turn;
    }
    let part2 = l as u32; 
       
    (part1, part2)
}    

fn main() {
    let input = "8,0,17,4,1,12";
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { process(input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());

    let _r = process("8,0,17,4,1,12");
}
