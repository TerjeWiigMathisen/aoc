// Fastest run (Surface): 50.8 us
//              Acer:     32.7 us
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;
use rustc_hash::FxHashMap;

//#[derive(Clone, Debug)]

// Trial and error found this value for the size of the overrun target window 
// as well as the step size to use when searching for the number of loops needed.
const TARGETSIZE:usize = 1700;

fn getcount(subject_number:usize, target_key:usize) -> usize
{
    let mut map: FxHashMap<usize, usize> = FxHashMap::default();
    let mut k = target_key;
    let mut sn = 1;
    for i in 0..TARGETSIZE {
        map.insert(k, i);
        //println!("{i}:{k}");
        k = (k * subject_number) % 20201227;
        sn = (sn * subject_number) % 20201227;
    }
    //let sn = k;
    //println!("sn = {sn}");
    k = 1;
    let mut iterations = 0;
    loop {
        iterations += TARGETSIZE;
        k = (k * sn) % 20201227;
        if map.contains_key(&k) { 
//            println!("Found {k} in index {} after {iterations} iterations", map[&k]);
            return iterations - map[&k];
        }
    }
}

fn process(inp:&str) -> usize
{
    let lines:Vec<&str> = inp.lines().collect();
    let cardkey = lines[0].parse::<usize>().unwrap();
    let doorkey = lines[1].parse::<usize>().unwrap();
    let cardloops = getcount(7,cardkey);
    //let doorloops = getcount8(7,doorkey);
    //let total = getcount(7,14897079);
    //println!("Card loops = {cardloops}, door loops = {doorloops}");
    // let mut k = 1;
    // for _ in 0..doorloops {
    //     k = (k * cardkey) % 20201227;
    // }
    // let mut d = 1;
    // for _ in 0..cardloops {
    //     d = (d * doorkey) % 20201227;
    // }
    let mut cl = cardloops;
    let mut powmod = 1;
    let mut key = doorkey;
    while cl != 0 {
        if cl & 1 != 0 { powmod = (powmod * key) % 20201227;}
        cl >>= 1;
        key = (key * key) % 20201227;
    }
    //println!("powmod = {powmod}");
    // assert!(d == k);
    powmod
}

fn _process_1000(inp:&str) -> (usize, usize)
{
    for _ in 0..1000 {
        process(&inp);
    }
    (0,0)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
//    let fname = "test.txt"; // instead of args[1]
    let input = fs::read_to_string(fname).expect("Error reading input file");
 //   if input.as_bytes()[input.len()-1] != b'\n' {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(&input); }); bench_result.print_stats();

    //process(&input);
    devtime.start();
    let part1 = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}