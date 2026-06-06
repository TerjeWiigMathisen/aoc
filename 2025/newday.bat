cargo new %1
md %1
cd %1
cargo add devtimer substring aoc-parse rustc_hash
copy /-y ..\day1\src\main.rs src\
copy /-y ..\aoc-template.pl aoc.pl