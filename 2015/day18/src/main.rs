// Fastest run 5.5 (or 2.4?) ms.
// SIMD /AVX 1404.1 us
// BitGrid 93.1 us

use std::u128;

//#[allow(dead_code, unused, unused_variables, unused_imports)]

use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

fn half_adder(a:u128, b:u128) -> (u128,u128) {
    let sum = a ^ b;
    let carry = a & b;
    (sum,carry)
}

fn full_adder(a:u128, b:u128, c:u128) -> (u128,u128) {
    let xor = a ^ b;
    let and = a & b;
    let sum = xor ^ c;
    let carry = and | (xor & c);
    (sum,carry)
}

fn add_carry(a0:u128, a1:u128, carry:u128) -> (u128,u128) {
    let sum = a0 ^ carry;
    let carry = (a0 & carry) | a1;
    (sum,carry)
}


#[derive(Clone)]
struct BitGrid {
    width: usize,
    height: usize,
    mask:u128,
    page:usize,
    grid: Vec<u128>,
}

impl BitGrid {
    fn from_text(inp:&str) -> BitGrid {
        let lines:Vec<&str> = inp.lines().collect();
        let height = lines.len();
        let width = lines[0].len();
        let mask:u128 = ((2 as u128) << width)-2;
        let mut grid:BitGrid = BitGrid{width:width,height:height,mask:mask,page:0,grid:vec![0 as u128; height*2+3]};
        for (y,line) in lines.iter().enumerate() {
            let mut u:u128 = 0;
            let mut bit:u128 = 2;
            let bytes = line.as_bytes();
            for c in bytes {
                if *c == b'#' {
                    u |= bit;
                }
                bit <<= 1;
            }
            grid.grid[y+1] = u;
        }
        grid
    }
    fn _get(&self, x: usize, y: usize) -> u8 {
        (self.grid[self.page+y+1] >> (x+1)) as u8 & 1
    }
    pub fn set(&mut self, x: usize, y: usize, val: u8) {
        self.grid[self.page+y+1] |= (val as u128) << (x+1);
    }
    pub fn _print(&self) {
        let ptab:[u8;2] = [b'.',b'#'];
        for y in 1..=self.height {
            let mut bits = self.grid[self.page+y];
            for _x in 0..self.width {
                bits >>= 1;
                print!("{}",ptab[(bits & 1) as usize] as char);
            }
            println!();
        }
        println!();
    } 

    fn fillcorners(&mut self) {
        self.set(0,0,1);
        self.set(self.width-1,0,1);
        self.set(0,self.height-1,1);
        self.set(self.width-1,self.height-1,1);
    }

    fn generation(&mut self) {
        let new_page:usize = self.page ^ (self.height+1);
        let (mut asum,mut acarry) = (0 as u128, 0 as u128); 

        let b0 = self.grid[self.page+1];
        let mut b1 = b0 >> 1;
        let b2 = b1 >> 1;
        let (mut bssum, mut bscarry) = half_adder(b0,b2);
        let (mut bsum,mut bcarry) = add_carry(bssum, bscarry, b1);

        for y in 0..self.height {
            let c0 = self.grid[self.page+y+2];
            let c1 = c0 >> 1;
            let c2 = c1 >> 1;
            let (cssum, cscarry) = half_adder(c0,c2);
            let (csum, ccarry) = add_carry(cssum, cscarry, c1);

            let (sum0,t1) = full_adder(asum, bssum, csum);
            let (t2,t3) = full_adder(acarry, bscarry, ccarry);
            let (sum1,t5) = half_adder(t1,t2);
            let sum2 = t5 | t3;

            let new_state = (sum0 | b1) & sum1 & !sum2;
            self.grid[new_page+y+1] = (new_state+new_state) & self.mask;
            asum = bsum;
            acarry = bcarry;
            bsum = csum;
            bssum = cssum;
            bcarry = ccarry;
            bscarry = cscarry;
            b1 = c1;
        }
        self.page = new_page;
    }
}

fn bitcount128(n:u128) -> i32 {
    let mut count = 0;
    let mut n = n;
    while n != 0 {
        count += 1;
        n &= n-1;
    }
    count
}   

fn bit_process(inp:&str) -> (i32, i32)
{
    let mut grid:BitGrid = BitGrid::from_text(inp);
    // grid._print();
    let grid2 = grid.clone();
    for _iter in 0..100 {
        grid.generation();
        // grid._print();
    }
    let part1 = grid.grid[grid.page+1..=grid.page+grid.height].iter().map(|x| bitcount128(*x)).sum();

    grid = grid2;
    grid.fillcorners();
    for _iter in 0..100 {
        grid.generation();
        grid.fillcorners();
    }
    let part2 = grid.grid[grid.page+1..=grid.page+grid.height].iter().map(|x| bitcount128(*x)).sum();

    (part1,part2)
}

fn main() {
    let args:Vec<String> = std::env::args().collect();
    let fname = if args.len() > 1 { args[1].clone() } else { "input.txt".to_string() };
    let mut input = fs::read_to_string(fname).expect("Error reading input file");
    if input.as_bytes()[input.as_bytes().len()-1] == b'\n' {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { bit_process(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = bit_process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}