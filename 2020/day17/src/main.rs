// Fastest run Surface:  782 us
//                Acer:  792 us
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;
//use rustc_hash::FxHashMap;

#[derive(Debug,Clone)]
struct Grid3 {
    xlen:usize,
    ylen:usize,
    zlen:usize,
    ystride:i64,
    zstride:i64,
    topleft:i64,
    grid:Vec<u8>,
}

const GENERATIONS:usize = 6;
const STEP:usize = GENERATIONS+1;
const ISTEP:i64 = STEP as i64;

fn parse3(inp:&str) -> Grid3
{
    let lines = inp.lines().collect::<Vec<&str>>();
    let xs = lines[0].len();
    let ys = lines.len();
    let xlen = xs+STEP*2;
    let ylen = ys+STEP*2;
    let zlen = STEP*2+1;
    let ystride = xlen as i64;
    let zstride = ystride * ylen as i64;
    let mut g3 = Grid3{xlen:xlen,ylen:ylen,zlen:zlen, ystride:ystride, zstride:zstride, 
        topleft:zstride*ISTEP + ystride*ISTEP + ISTEP, grid:vec![0;zstride as usize * zlen]};
    let mut lstart = g3.topleft as usize;
    for l in lines {
        let bytes = l.as_bytes();
        for i in 0..bytes.len() {
            if bytes[i] == b'#' { 
                for dz in -1..=1 {
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let adr = ((i+lstart) as i64 + dz * g3.zstride + dy * g3.ystride + dx) as usize;
                            g3.grid[adr] += 1;
                            //assert!(new3.grid[adr] > g3.grid[adr]);
                        }
                    }
                }
                g3.grid[i+lstart] += 128-1; // Reverse the update of the cell itself
            }
        }
        lstart += ystride as usize;
    }
    g3
}

#[derive(Debug,Clone)]
struct Grid4 {
    xlen:usize,
    ylen:usize,
    zlen:usize,
    wlen:usize,
    ystride:i64,
    zstride:i64,
    wstride:i64,
    topleft:i64,
    grid:Vec<u8>,
}

fn parse4(inp:&str) -> Grid4
{
    let lines = inp.lines().collect::<Vec<&str>>();
    let xs = lines[0].len();
    let ys = lines.len();
    let xlen = xs+STEP*2;
    let ylen = ys+STEP*2;
    let zlen = STEP*2+1;
    let wlen = STEP*2+1;
    let ystride = xlen as i64;
    let zstride = ystride * ylen as i64;
    let wstride = zstride * zlen as i64;
    let mut g4 = Grid4{xlen:xlen,ylen:ylen,zlen:zlen,wlen:wlen, ystride:ystride, zstride:zstride, wstride:wstride,
        topleft:wstride*ISTEP + zstride*ISTEP + ystride*ISTEP + ISTEP, grid:vec![0;wstride as usize * wlen]};
    let mut lstart = g4.topleft as usize;
    for l in lines {
        let bytes = l.as_bytes();
        for i in 0..bytes.len() {
            if bytes[i] == b'#' { 
//                g4.grid[lstart+i] = 128;}
                for dw in -1..=1 {
                    for dz in -1..=1 {
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                let adr = ((i+lstart) as i64 + dw * g4.wstride + dz * g4.zstride + dy * g4.ystride + dx) as usize;
                                g4.grid[adr] += 1;
                                //assert!(new3.grid[adr] > g3.grid[adr]);
                            }
                        }
                    }
                }
                g4.grid[i+lstart] += 128-1; // Reverse the update of the cell itself
            }
        }
        lstart += ystride as usize;
    }
    g4
}

fn alive(grid:&[u8]) -> usize
{
    let mut cnt = 0;
    for i in 0..grid.len() {
        cnt += (grid[i] & 128) as usize;
    }
    cnt >> 7
}

fn gen1(g3:&Grid3) -> Grid3
{
    let mut new3 = g3.clone();
    for i in 0..g3.grid.len() {
        let cell = g3.grid[i];
        if cell >= 128 {
            if (cell | 1) != 131 {
                // This cell dies, decrement neighbor counts
                for dz in -1..=1 {
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let adr = (i as i64 + dz * g3.zstride + dy * g3.ystride + dx) as usize;
                            new3.grid[adr] -= 1;
                            //assert!(new3.grid[adr] < g3.grid[adr]);
                        }
                    }
                }
                new3.grid[i] -= 128-1;
                //assert!(new3.grid[i] == cell);
            }
        }
        else if cell == 3 { // Currently dead, 3 neighbors turns it active
            for dz in -1..=1 {
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let adr = (i as i64 + dz * g3.zstride + dy * g3.ystride + dx) as usize;
                        new3.grid[adr] += 1;
                        //assert!(new3.grid[adr] > g3.grid[adr]);
                    }
                }
            }
            new3.grid[i] += 128-1; // Reverse the update of the cell itself
            //assert!(new3.grid[i] == cell);
        }
    }
    new3
}

fn gen2(g4:&Grid4) -> Grid4
{
    let mut new4 = g4.clone();
    for i in 0..g4.grid.len() {
        let cell = g4.grid[i];
        if cell >= 128 {
            if (cell | 1) != 131 {
                // This cell dies, decrement neighbor counts
                for dw in -1..=1 {
                    for dz in -1..=1 {
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                let adr = (i as i64 + dw * g4.wstride + dz * g4.zstride + dy * g4.ystride + dx) as usize;
                                new4.grid[adr] -= 1;
                            }
                        }
                    }
                }
                new4.grid[i] -= 128-1;
            }
        }
        else if cell == 3 { // Currently dead, 3 neighbors turns it active
            for dw in -1..=1 {
                for dz in -1..=1 {
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let adr = (i as i64 + dw * g4.wstride + dz * g4.zstride + dy * g4.ystride + dx) as usize;
                            new4.grid[adr] += 1;
                        }
                    }
                }
            }
            new4.grid[i] += 128-1;
        }
    }
    new4
}

fn _checknb(g3:&Grid3)
{
    for i in 0..g3.grid.len() {
        let curr = g3.grid[i];
        if curr != 0 {
            let mut nbor = 0;
            for dz in -1..=1 {
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let adr = (i as i64 + dz * g3.zstride + dy * g3.ystride + dx) as usize;
                        nbor += (g3.grid[adr] & 128) as usize;
                    }
                }
            }
            nbor -= (curr & 128) as usize;
            let nb = (nbor >> 7) as u8;
            if (curr & 128) + nb != curr {
                println!("Found bad neighbor count for i = {i}: {} should have been {}", curr, nb);
                panic!("");
            }
            else {print!("{}", if curr >= 128 {"#"} else {"."});}
        }
    }
    println!();
}

fn _dumpgrid(g3:&Grid3)
{
    let mut nl = g3.ystride - 1;
    let mut np = g3.zstride - 1;
    for i in 0..g3.grid.len() {
        if g3.grid[i] >= 128 {
            print!("#");
        }
        else {print!["."];}
        if i >= nl as usize { println!(); nl += g3.ystride;}
        if i >= np as usize { println!(); np += g3.zstride;}
    }
}

fn process(inp:&str) -> (usize, usize)
{
    let mut g3 = parse3(inp);
    for _i in 1..=6 {
        g3 = gen1(&g3);
    }
    let part1 = alive(&g3.grid);

    let mut g4 = parse4(inp);
    for _i in 1..=6 {
        g4 = gen2(&g4);
    }
    let part2 = alive(&g4.grid);

    (part1,part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
//    let fname = "test.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error reading input file");
    if input.as_bytes()[input.len()-1] != b'\n' {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(&input); }); bench_result.print_stats();

    //process(&input);
    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}