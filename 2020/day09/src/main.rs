// Fastest run (Surface): 14.7 us
//              Acer:      7.2 us
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;
//use ahash::AHashSet;

fn findsum(numbers:&[u64], sum:u64) -> bool
{
    for k in 0..24 {
        let first = numbers[k];
        for l in (k+1)..25 {
            if first + numbers[l] == sum /* && first != second */ {return true}
        }
    }
    false
}

// fn p1(numbers:&Vec<u64>) -> u64
// {
//     let mut set:AHashSet<u64> = AHashSet::new();
//     for i in 0..25 {
//         set.insert(numbers[i]);
//     }
//     for i in 25..numbers.len() {
//         let sum = numbers[i];
//         let start = numbers[i-25];
//         let mut found = false;
//         for j in i-25..i {
//             let first = numbers[j];
//             if first >= sum {continue}
//             let diff = sum-first;
//             if set.contains(&diff) {found = true; break}
//         }
//         if !found {return sum}
//         set.remove(&start);
//         set.insert(sum);
//     }
//     0
// }

fn parse(inp:&str) -> Vec<u64>
{
    let mut num:Vec<u64> = Vec::with_capacity(1000);
    let mut i = 0;
    let bytes = inp.as_bytes();
    while i < bytes.len() {
        let mut n = (bytes[i]-b'0') as u64; i += 1;
        while bytes[i] >= b'0' {
            n = n*10 + (bytes[i]-b'0') as u64;
            i += 1;
        }
        num.push(n);
        i += 1;
    }
    num
}

fn process(inp:&str) -> (u64, u64)
{
    let numbers:Vec<u64> = parse(&inp); //inp.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    //println!("numbers: {:?}", numbers);
    //let part1 = p1(&numbers);
    let mut part1 = 0;
    for i in 25..numbers.len() {
        let sum = numbers[i];
        if !findsum(&numbers[i-25..i], sum) { 
            part1 = sum; 
            //println!("First non-sum value = {sum} (at index {}", i+1);
            break;
        }
    }
    let mut left = 0;
    let mut right = 1;
    let mut sum = numbers[left];
    while sum != part1 {
        while sum < part1 {sum += numbers[right]; right += 1}
        while sum > part1 {sum -= numbers[left]; left += 1}
    }
    let mut smallest = u64::MAX;
    let mut largest = 0;
    for i in left..right {
        let n = numbers[i];
        if n < smallest {smallest = n}
        if n > largest {largest = n}
    }
    //println!("found range: [{smallest}-{largest}, numbers[{left}..{right}]");
    let part2 = smallest + largest;
    (part1,part2)
}

fn _process_1000(inp:&str) -> (u64,u64)
{
    for _ in 0..1000 {
        process(&inp);
    }
    (0,0)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.len()-1] != b'\n' {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { _process_1000(&input); }); bench_result.print_stats();

    //process(&input);
    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us for 1000 runs",devtime.time_in_micros().unwrap());
}