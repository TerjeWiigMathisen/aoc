// Fastest: Acer 202 ms
//       Surface 311 ms
use std::vec;
use devtimer::DevTime;
use devtimer::run_benchmark;

fn process(input: &str) -> (u32, u32)
{
    let nums = input.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let mut spoken = vec![0 as u32; 30000000];
    let mut seen:Vec<u64> = vec![0;(30000000+63)/64];
    
    let mut turn = 0;
    let mut last = nums[nums.len()-1];
    for i in nums {
        turn += 1;
        spoken[i as usize] = turn;
    }
    
    while turn < 2020 {
        let previous = spoken[last as usize];
        spoken[last as usize] = turn as u32;
        last = if previous == 0 {0} else {turn-previous};
        turn += 1;
    }
    let part1 = last as u32;

    // let mut scnt = 0;
    // let mut hits = 0;

    while turn < 30000000 {
        if last < 125000 { // 120K 193  100K 201 140K 206
            let previous = spoken[last as usize];
            spoken[last as usize] = turn as u32;
            last = if previous == 0 {0} else {turn-previous};
        }
        else {
            let bit = 1 << (last & 63);
            let word = (last >> 6) as usize;
            if seen[word] & bit == 0 {
                seen[word] |= bit;
                spoken[last as usize] = turn as u32;
                last = 0;
                // scnt += 1;
            }
            else {
                let previous = spoken[last as usize];
                spoken[last as usize] = turn as u32;
                last = turn-previous;
                //last = if previous == 0 {0} else {turn-previous};
                // hits += 1;
            }
        }
        turn += 1;
    }
    let part2 = last as u32; 
    // println!("Seen bitmap count: {scnt}, total hits = {hits}");
       
    (part1, part2)
}    

fn main() {
    let input = "8,0,17,4,1,12";
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(25, |_| { process(input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());

    let _r = process("8,0,17,4,1,12");
}
