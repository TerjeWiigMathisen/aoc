// Fastest run Surface 527 us
// Dirty list 990 us

use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

fn removed3(stride:usize, start:usize, end:usize, buffer:&mut Vec<u8>)-> (usize, usize, usize)
{
    let mut remove:Vec<u16> = Vec::with_capacity(1500);
    for i in start..end {
        let b = buffer[i];
        if (b & 1) != 0 && b < 8 { 
            remove.push(i as u16);
        }
    }
    let remcnt = remove.len();
    if remcnt == 0 { return (0,0,0); }

    for j in 0..remcnt {
        let i = remove[j] as usize;
        buffer[i-stride-1] -= 2;
        buffer[i-stride] -= 2;
        buffer[i-stride+1] -= 2;
        buffer[i-1] -= 2;
        buffer[i] -= 1;
        buffer[i+1] -= 2;
        buffer[i+stride-1] -= 2;
        buffer[i+stride] -= 2;
        buffer[i+stride+1] -= 2;
    }
    (remcnt, remove[0] as usize - stride - 1, remove[remcnt-1] as usize + stride + 1)
}

fn remove_dirty(stride:usize, buffer:&mut Vec<u8>, dirty:&mut Vec<u16>)-> usize
{
    let mut next:Vec<u16> = Vec::with_capacity(1500);
    for d in 0..dirty.len() {
        let i = dirty[d] as usize; 
        let b = buffer[i];
        if (b & 1) != 0 && b < 8 { 
            next.push(i as u16);
        }
    }
    let remcnt = next.len();
    dirty.clear();
    if remcnt == 0 { return 0; }
    for j in 0..remcnt {
        let i = next[j] as usize;
        for k in [i-stride-1, i-stride, i-stride+1, i-1, i+1, i-stride-1, i-stride, i-stride+1] {
            if buffer[k] & 1 != 0 {
                buffer[k] -= 2;
                dirty.push(k as u16);
            }
        }
        buffer[i] -= 1;
    }
    dirty.sort_unstable();
    dirty.dedup();
    remcnt
}

fn removed(stride:usize, start:usize, end:usize, buffer:&mut Vec<u8>)-> (usize, usize, usize)
{
    let mut remove:Vec<u16> = Vec::with_capacity(1500);
    for i in start..end {
        let b = buffer[i];
        if (b & 1) != 0 && b < 8 { 
            remove.push(i as u16);
        }
    }
    let remcnt = remove.len();
    if remcnt == 0 { return (0,0,0); }

    for j in 0..remcnt {
        let i = remove[j] as usize;

        let addr = i-stride-1;
        let b4 = buffer[addr..addr+4].try_into().unwrap();
        let u = u32::from_le_bytes(b4) - 0x20202;
        buffer[addr..addr+4].copy_from_slice(&u.to_le_bytes());

        let addr = i-1;
        let b4 = buffer[addr..addr+4].try_into().unwrap();
        let u = u32::from_le_bytes(b4) - 0x20102;
        buffer[addr..addr+4].copy_from_slice(&u.to_le_bytes());

        let addr = i+stride-1;
        let b4 = buffer[addr..addr+4].try_into().unwrap();
        let u = u32::from_le_bytes(b4) - 0x20202;
        buffer[addr..addr+4].copy_from_slice(&u.to_le_bytes());
    }
    (remcnt, remove[0] as usize - stride - 1, remove[remcnt-1] as usize + stride + 1)
}

fn process(inp:&str) -> (usize, usize)
{
    let linelen = inp.find('\n').unwrap();
    let stride = linelen+1;
    let mut start = stride+1;
    let bytes = inp.as_bytes();
    let blen = bytes.len();
    let mut buffer:Vec<u8> = vec![0;stride*2+blen+2];
    let mut i = start;
    let mut dirty:Vec<u16> = Vec::with_capacity(4000);
    for j in 0..blen {
        let b = bytes[j];
//        if b == b'\n' { x = 0; y += 1;}
        if b == b'@' {
            buffer[i-stride-1] += 2;
            buffer[i-stride] += 2;
            buffer[i-stride+1] += 2;
            buffer[i-1] += 2;
            buffer[i] += 1;
            buffer[i+1] += 2;
            buffer[i+stride-1] += 2;
            buffer[i+stride] += 2;
            buffer[i+stride+1] += 2;
            dirty.push(i as u16);
        }
        i += 1;
    }
    let part1 = remove_dirty(stride, &mut buffer, &mut dirty);
    let mut part2 = part1;
    loop {
        let rem = remove_dirty(stride, &mut buffer, &mut dirty);
        part2 += rem;
        if rem == 0 { break; }
    }
    (part1, part2)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut fname = "input.txt".to_string();
    if args.len() > 1 { fname = args[1].clone(); }

    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}