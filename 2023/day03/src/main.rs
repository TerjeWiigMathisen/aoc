// day2
// Surface:     79.7 us
// Acer:   

use std::fs;
use devtimer::run_benchmark;

#[derive(Debug)]
struct Number {
    val:u32,
    pos:u16,
    len:u16,
}

const
    STAR:u16 = 0x8000;
const
    SYMBOL:u16 = 0x4000;

fn check_grid_pos(cell:&mut u16, val:u32) -> (u64,u64)
{
    let s = *cell;
    if s >= SYMBOL {
        let p1 = val as u64;
        if s == STAR { // Star, first time seen
            *cell = s + val as u16; // Remember the first number
            return (p1,0); 
        }
        if s > STAR { // Second time star
            if s > STAR+SYMBOL {
                println!("star has more than 2 numbers!")
            }
            *cell |= SYMBOL; 
            return (p1, (s & 0x3fff) as u64 * p1 );
        }
        // Plain symbol
        return (p1,0);
    }
    (0,0)
}

fn _dumpgrid(grid:&Vec<u16>, stride:usize)
{
    let mut x = 0;
    for i in 0..grid.len() {
        let s = grid[i];
        if s >= STAR { print!("*");}
        else if s >= SYMBOL { print!("#");}
        else if s > 0 {print!("N");}
        else {print!(".");}
        x += 1;
        if x >= stride {
            println!();
            x = 0;
        }
    }
    println!();
}

fn symbols_around_number(grid:&mut Vec<u16>, stride:usize, num:&Number) -> (u64,u64) 
{
    let mut pos = num.pos as usize-1;
    let val = num.val;
    let numlen = num.len;
    let mut part1 = 0;
    let mut part2 = 0;
//    print!("left: ");
    for offs in [pos-stride, pos, pos+stride] {
        let (p1,p2) = check_grid_pos(&mut grid[offs], val);
        if p1 > part1 {part1 = p1;}
        if p2 > part2 {part2 = p2;}
//        print!("({offs},{p1},{p2}) ");
    }
//    println!();
    pos += 1;
//    print!("middle: ");
    for _ in 0..numlen {
        for offs in [pos-stride, pos+stride] {
            let (p1,p2) = check_grid_pos(&mut grid[offs], val);
            if p1 > part1 {part1 = p1;}
            if p2 > part2 {part2 = p2;}
//            print!("({offs},{p1},{p2}) ");
        }
        pos += 1;
    }
//    println!();
//    print!("right: ");
    for offs in [pos-stride, pos, pos+stride] {
        let (p1,p2) = check_grid_pos(&mut grid[offs], val);
        if p1 > part1 {part1 = p1;}
        if p2 > part2 {part2 = p2;}
//        print!("({offs},{p1},{p2}) ");
    }
//    println!();
    (part1, part2)
}


pub fn process(inp:&String)->(u64, u64)
{
    let input = inp.as_bytes();
    let mut part1 = 0;
    let mut part2 = 0;
    let mut i = 0;
    while input[i] != b'\n' { i +=1;}
    let stride = i+1;

    let mut grid:Vec<u16> = vec![0;stride*2+input.len()+2];
    let mut nums:Vec<Number> = vec![];
    i = 0;
    while i < input.len() {
        let b = input[i]; i += 1;
        if b == b'.' || b <= b' ' {continue;}

        if b >= b'0' && b <= b'9' {
            let mut n = (b-b'0') as u32;
            let start = (i-1) as u16;
            let mut len = 1;
            while input[i] >= b'0' && input[i] <= b'9' {
                n = n*10 + (input[i] - b'0') as u32;
                i += 1;
                len += 1;
            }
            nums.push(Number{val:n,pos:start+1+stride as u16,len:len});
//            println!("{:?}", nums[nums.len()-1]);
            continue;
        }
        grid[i+stride] = if b == b'*' {STAR} else {SYMBOL};
    }
//    dumpgrid(&grid, stride);
    for n in 0..nums.len() {
        let (p1,p2) = symbols_around_number(&mut grid, stride, &nums[n]);
//        println!("Testing {:?} -> ({p1},{p2})", nums[n]);
        part1 += p1;
        part2 += p2;
    }
    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
//    let fname = "test.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] != b'\n' as u8 {input.push('\n');}

    let bench_result = run_benchmark(1000, |_| {
        process(&input);
    });
    bench_result.print_stats();

    let part1 = process(&input);
    println!("part1={}\npart2={}", part1.0, part1.1);
}
