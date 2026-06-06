// Fastest run (Acer): 4.5 us with u8 vars
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

fn process(inp:&str) -> (usize, usize)
{
    let bytes = inp.as_bytes();
    let mut i = 0;
    let mut part1 = 0;
    let mut part2 = 0;
    while i < bytes.len() {
        let mut fra = (bytes[i]-b'0') as usize;
        i += 1;
        while bytes[i] != b'-' {
            fra = fra*10 + (bytes[i] - b'0') as usize;
            i += 1;
        }
        i += 1; // Skip '-'
        let mut til  = (bytes[i]-b'0') as usize;
        i += 1;
        while bytes[i] != b' ' {
            til = til*10 + (bytes[i] - b'0') as usize;
            i += 1;
        }
        // First space, after nn-mm
        let target = bytes[i+1];
        i += 4; // Skip ' A: ' 
        let mut lettercount = 0;
        let pstart = i-1; // Password string is 1-indexed
        //println!("{fra}-{til} {}: {pstart}", target);
        while bytes[i] >= b'a' {
            let b = bytes[i];
            i+=1;
            lettercount += (b == target) as usize;
        }
        // Found end of current line, check results:
        if lettercount >= fra && lettercount <= til {part1 += 1;}
        if (bytes[pstart+fra] == target) != (bytes[pstart+til] == target) {part2 += 1;}
        i += 1;
    }
    (part1, part2)
}

fn process8(inp:&str) -> (usize, usize)
{
    let bytes = inp.as_bytes();
    let mut i = 0;
    let mut part1 = 0;
    let mut part2 = 0;
    while i < bytes.len() {
        let mut fra = bytes[i]-b'0';
        i += 1;
        while bytes[i] != b'-' {
            fra = fra*10 + bytes[i] - b'0';
            i += 1;
        }
        i += 1; // Skip '-'
        let mut til  = bytes[i]-b'0';
        i += 1;
        while bytes[i] != b' ' {
            til = til*10 + bytes[i] - b'0';
            i += 1;
        }
        // First space, after nn-mm
        let target = bytes[i+1];
        i += 4; // Skip ' A: ' 
        let mut lettercount:u8 = 0;
        let pstart = i-1; // Password string is 1-indexed
        //println!("{fra}-{til} {}: {pstart}", target);
        while bytes[i] >= b'a' {
            let b = bytes[i];
            i+=1;
            lettercount += (b == target) as u8;
        }
        // Found end of current line, check results:
        part1 += ((lettercount >= fra) & (lettercount <= til)) as usize;
        part2 += ((bytes[pstart+fra as usize] == target) ^ (bytes[pstart+til as usize] == target)) as usize;
        i += 1;
    }
    (part1, part2)
}
fn main() {
//    let bench_result = run_benchmark(10, |_| {bench_permute();}); bench_result.print_stats();

    //panic!("Stop now");

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.len()-1] != b'\n' {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process8(&input); }); bench_result.print_stats();

    //process(&input);
    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}