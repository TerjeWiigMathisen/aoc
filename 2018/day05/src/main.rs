// Fastest run 454 us
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

fn react2(dst:&mut Vec<u8>, inp:&Vec<u8>, skip:u8) -> i64
{
    let mut n = 0;
    for i in 0..inp.len() {
        if inp[i] | 32 != skip {
            if dst[n] ^ inp[i] == 32 {
                n -= 1;
            }
            else {
                n += 1;
                dst[n] = inp[i];
            }
        }
    }
    n as i64
}

fn process_both(inp:&str) -> (i64,i64)
{
    let poly:Vec<u8> = inp.as_bytes().to_vec();
    let mut dst:Vec<u8> = vec![0;poly.len()+1];
    let p1 = react2(&mut dst,&poly,0);
    let poly2:Vec<u8> = dst[1..=p1 as usize].to_vec();
    let mut p2 = i64::MAX;
    for letter in b'a'..=b'z' {
        let n = react2(&mut dst,&poly2,letter);
        if n < p2 {p2 = n}
    }
    (p1,p2)
}

fn main() {
    assert!(process_both(
"dabAcCaCBAcCcaDA").0 == 10);

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { 
        process_both(&input); 
    }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process_both(&input);
    // let part1 = process(input.clone());
    // let part2 = process2(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    
    // println!("Part1,2 = {part1},{part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}