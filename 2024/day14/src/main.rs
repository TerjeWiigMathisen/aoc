// Acer 2540 us
// Using minimum variance to x and y iteration independently, then finding the first common 
// iteration that is a multiple of both periods. This is much faster than simulating all iterations 
// and checking for collisions.
// Acer 51.0 us

use std::env::args;
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

struct Point {
    x:u32,
    y:u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point {x:x, y:y}
    }
}

const WIDTH:u32 = 101;
const HEIGHT:u32 = 103;

struct Robot {
    x:u32,
    y:u32,
    vx:u32,
    vy:u32,
}
impl Robot {

    fn newpos(&mut self, iter:u32) -> (u32, u32)
    {
        let x = (self.x + self.vx * iter) % WIDTH;
        let y = (self.y + self.vy * iter) % HEIGHT;
        (x,y)
    }
    fn nextpos(&mut self) -> (u32, u32)
    {
        self.x += self.vx;
        if self.x >= WIDTH {self.x -= WIDTH;}
        self.y += self.vy;
        if self.y >= HEIGHT {self.y -= HEIGHT;}
        (self.x,self.y)
    }
}

fn process2(inp:&str) -> (i32, i32)
{
    let mut robots:Vec<Robot> = Vec::new();

    let input = inp.as_bytes();
    let mut i = 0;
    while i+10 < input.len() {
        let mut x:u32 = 0;
        let mut y:u32 = 0;
        let mut vx:i32 = 0;
        let mut vy:i32 = 0;
        while input[i] != b'=' {i += 1;} i += 1; // p=
        while input[i] != b',' {x = x*10 + (input[i]-b'0') as u32; i += 1;} i += 1; // ,
        while input[i] != b' ' {y = y*10 + (input[i]-b'0') as u32; i += 1;} i += 3; // " v="
        if input[i] == b'-' {i += 1; while input[i] != b',' {vx = vx*10 - (input[i]-b'0') as i32; i += 1;}} 
        else {while input[i] != b',' {vx = vx*10 + (input[i]-b'0') as i32; i += 1;}} i += 1; // ,
        if input[i] == b'-' {i += 1; while input[i] != b'\n' && i < input.len() {vy = vy*10 - (input[i]-b'0') as i32; i += 1;}} 
        else {while input[i] != b'\n' && i < input.len() {vy = vy*10 + (input[i]-b'0') as i32; i += 1;}}
        if vx < 0 {vx += WIDTH as i32;}
        if vy < 0 {vy += HEIGHT as i32;}
        robots.push(Robot{x, y, vx:vx as u32, vy:vy as u32});
    }

    let mut quad:Vec<i32> = vec![0, 0, 0, 0];
    let x2 = WIDTH/2;
    let y2 = HEIGHT/2;

    // part2
    let mut xvar_min_index = 0;
    let mut xvarmax = i64::MAX;
    let mut yvar_min_index = 0;
    let mut yvarmax = i64::MAX;
    for i in 1..=103 {
        let mut cnt = 0;
        let mut xdelta:i64 = 0;
        let mut xmean:i64 = 0;
        let mut x_sum_squared:i64 = 0;
        let mut ydelta:i64 = 0;
        let mut ymean:i64 = 0;
        let mut y_sum_squared:i64 = 0;

        for r in &mut robots {
            let (x,y) = r.nextpos();
            xmean += x as i64;
            ymean += y as i64;
            if i == 100 {
                let xq = if x < x2 {0} else if x > x2 {1} else {continue};
                let yq = if y < y2 {0} else if y > y2 {2} else {continue};
                let idx = (xq + yq) as usize;
                quad[idx] += 1;
            }
        }
        xmean /= robots.len() as i64;
        ymean /= robots.len() as i64;
        for r in &robots {
            let (x,y) = (r.x, r.y);
            xdelta = x as i64 - xmean;
            ydelta = y as i64 - ymean;
            x_sum_squared += xdelta * xdelta;
            y_sum_squared += ydelta * ydelta;
        }
        if x_sum_squared < xvarmax {
            xvarmax = x_sum_squared; xvar_min_index = i;
//            println!("i={} xvarmax={}", i, xvarmax);
        }
        if y_sum_squared < yvarmax {
            yvarmax = y_sum_squared; yvar_min_index = i;
//            println!("i={} yvarmax={}", i, yvarmax);
        }
        // if (i == 43 || i == 68) {
        //     println!("i={} xvar_min_index={} yvar_min_index={}", i, xvar_min_index, yvar_min_index);
        // }
    }
    loop {
        if xvar_min_index == yvar_min_index {break;} 
        if xvar_min_index < yvar_min_index {xvar_min_index += 101;} else {yvar_min_index += 103;}
    }
    let part1 = quad.iter().product();
    let part2 = xvar_min_index;
    (part1, part2)
}

/*
fn process_collide(inp:&str) -> (i32, i32)
{
    let mut robots:Vec<Robot> = Vec::new();

//    let lp = parser!("p=" x:i32 "," y:i32 " v=" vx:i32 "," vy:i32 => Robot {x, y, vx, vy });
    for line in inp.lines() {
//        let mut r = lp.parse(line).unwrap();
        let r = Robot::newpos(0); // placeholder for actual parsing logic
        /* Make all delta values positive, this simplifies and speeds up the % operations */
        if r.vx < 0 {r.vx += LR.x;}
        if r.vy < 0 {r.vy += LR.y;}
        robots.push(r);
    }
    // part1
    let mut quad:Vec<i32> = vec![0, 0, 0, 0];
    let x2 = LR.x/2;
    let y2 = LR.y/2;
    for r in &mut robots {
        let (x, y) = r.newpos(100);
        let xq = if x < x2 {0} else if x > x2 {1} else {continue};
        let yq = if y < y2 {0} else if y > y2 {2} else {continue};
        let idx = (xq + yq) as usize;
        quad[idx] += 1;
    }
    let part1 = quad.iter().product();

    // part2
    let mut part2 = 0;
    let size = (LR.x+2 + ((LR.y+2)*111)) as usize;
    for i in 1..=10403 { // 101*103, the pattern repeats after this point
        let mut rb:Vec<u8> = vec![0; size]; // With guard cells
        let mut collide = false;
        for r in &mut robots {
            let (x,y) = r.newpos(i);
            let o = ((x+1) + (y+1)*111) as usize;
            if rb[o] > 0 {collide = true; break;}
            rb[o] += 1;
        }
        if !collide {
            part2 = i;
            break;
        }
    }

    (part1, part2)
}
*/

fn main() {
    let args = args().collect::<Vec<String>>();
    let fname = if args.len() > 1 {&args[1]} else {"input.txt"}; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] != '\n' as u8 {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { process2(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process2(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}