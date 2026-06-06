// Fastest run (Acer): 86.1 us vs 33
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

fn process(inp:&str) -> (usize, usize)
{
    let bytes = inp.as_bytes();
    let mut i = 0;
    let mut part1 = 0;
    let mut part2 = 0;

    let mut  p1  = 0;
    let mut p2 = 0;
    //let prev = b'\n';
    let eyecolors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    while i < bytes.len() {
        let b = bytes[i];
        if b >= b'a' {
            let key:[u8;3] = [bytes[i],bytes[i+1],bytes[i+2]];
            i += 4;
            let vstart = i;
            while bytes[i] > b' ' { i += 1;}
            let val = String::from_utf8(bytes[vstart..i].to_vec()).unwrap();
            //println!("key = {}, val = {val}", String::from_utf8(key.to_vec()).unwrap());
            match &key {
                b"byr" => {
                    p1 |= 1;
                    let byr:usize = val.parse().expect("Expected birth year");
                    if byr >= 1920 && byr <= 2002 { p2 |= 1;}
                },
                b"iyr" => {
                    p1 |= 2;
                    let iyr:usize = val.parse().expect("Expected issue year");
                    if iyr >= 2010 && iyr <= 2020 { p2 |= 2;}
                },
                b"eyr" => {
                    p1 |= 4;
                    let eyr:usize = val.parse().expect("Expected expiry year");
                    if eyr >= 2020 && eyr <= 2030 { p2 |= 4;}
                },
                b"hgt" => {
                    p1 |= 8;
                    if val.len() >= 4 {
                        let valh = String::from_utf8(bytes[vstart..i-2].to_vec()).unwrap();
                        let hgt:usize = valh.parse().expect("Expected height");
                        match &bytes[i-2..i] {
                            b"cm" => {if hgt >= 150 && hgt <= 193 {p2 |= 8;}},
                            b"in" => {if hgt >= 59 && hgt <= 76 {p2 |= 8;}},
                            _ => {}
                        }
                    }
                },
                b"hcl" => {
                    p1 |= 16;
                    if val.len() == 7 && bytes[vstart] == b'#' {
                        let val = String::from_utf8(bytes[vstart+1..i].to_vec()).unwrap();
                        let _ = u32::from_str_radix(&val, 16).unwrap();
                        p2 |= 16;
                    }
                },
                b"ecl" => {
                    p1 |= 32;
                    if eyecolors.contains(&val.as_str())  { p2 |= 32;}
                },
                b"pid" => {
                    p1 |= 64;
                    if val.len() == 9 {
                        let _ = u32::from_str_radix(&val, 10).unwrap();
                        p2 |= 64;
                    }
                },
                _ => {}
            }
        }
        i += 1;
        if i < bytes.len() && bytes[i] == b'\n' {
            //println!("p1 = {p1}, p2 = {p2}");
            part1 += (p1 == 127) as usize; p1 = 0;
            part2 += (p2 == 127) as usize; p2 = 0;
            i += 1;
        }
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

    let bench_result = run_benchmark(100, |_| { process(&input); }); bench_result.print_stats();

    //process(&input);
    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}