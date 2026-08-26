// day2
// Surface:  3.8 us
// Acer:     1.7 us

use std::fs;
use devtimer::run_benchmark;

pub fn process(inp:&String)->(u32, u32)
{
    let input = inp.as_bytes();
    let mut part1 = 0;
    let mut part2 = 0;
    let mut i = 6; // skip "Game 1"
    let mut game = 0;
    let mut rm = 0;
    let mut gm = 0;
    let mut bm = 0;
    while i < input.len() {
        game += 1;
        while input[i] != b':' { i += 1;}
        i += 2; //skip :space
        loop  { // games ending on newline
            let mut n = (input[i]-b'0') as u32; i += 1;
            while input[i] >= b'0' {
                n = n*10 + (input[i]-b'0') as u32; i += 1;
            }
//            println!("n={n} {}", input[i+1] as char);
            i += 1; //skip space
            if input[i] == b'r' {
                if n > rm {rm = n;}
                i += 3;  // "red"
            }
            else if input[i] == b'g' {
                if n > gm {gm = n;}
                i += 5; // "green"
            }
            else if input[i] == b'b' {
                if n > bm {bm = n;}
                i += 4; // "blue"
            }
            if input[i] == b'\n' { //end of line, aggregate results
                break;
            }
            i += 2; // Skip ,; and space
        }
//        println!("{game}: RGB=({rm},{gm},{bm})");
        if rm <= 12 && gm <= 13 && bm <= 14 {
            part1 += game;
        }
        part2 += rm*gm*bm;
        i += 7;  // skip "\nGame n"
        (rm,gm,bm) = (0,0,0);
    }

    return (part1, part2);
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] != b'\n' as u8 {input.push('\n');}

    let bench_result = run_benchmark(1000, |_| {
        process(&input);
    });
    bench_result.print_stats();

    let part1 = process(&input);
    println!("part1={}\npart2={}", part1.0, part1.1);
}
