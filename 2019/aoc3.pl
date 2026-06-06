#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);

#use bigint;

#use JSON::Parse;
#no warnings 'recursion';

my $DEBUG = 0;

my $fname = shift;
$fname = 'input.txt' unless (defined($fname));
open(F ,'<',$fname);

my $BUFFER = shift;
$BUFFER = 100 unless (defined($BUFFER));

my $start = time;

my $part1 = 0;
my $part2 = 0;

my @inp;
foreach (<F>) {
	chomp;
	push(@inp,$_);
}
close(F);

my @moves = split(/,/,$inp[0]);
my %points;
my %delta = ('R' => '1,0', 'D' => '0,1', 'L' => '-1,0', 'U' => '0,-1');
#my %delta = ('R' => [1,0], 'D' => [0,1], 'L' => [-1,0], 'U' => [0,-1]);

my @mm = (1e12,1e12,-1e12,-1e12);
my ($x, $y) = (0,0);
my $steps = 0;
foreach (@moves) {
	die("parse") unless (/^([RUDL])(\d+)$/);
	my ($d,$len) = ($1,$2);
	my ($dx,$dy) = split(/,/,$delta{$d});
#	my @d = $delta{$d};
#	my ($dx,$dy) = ($d[0][0],$d[0][1]);
	for (my $i = 0; $i < $len; $i++) {
		$steps++;
		$x += $dx; $y += $dy;
		$points{"$x,$y"} = $steps unless defined($points{"$x,$y"});
	}
	$mm[0] = $x if ($x < $mm[0]);
	$mm[1] = $y if ($y < $mm[1]);
	$mm[2] = $x if ($x > $mm[2]);
	$mm[3] = $y if ($y > $mm[3]);
}

printf("Range = (%d,%d) to (%d,%d)\n", $mm[0],$mm[1],$mm[2],$mm[3]);

#printf("Found %d points\n", scalar(keys %points));

$part1 = 1e12;
$part2 = 1e12;

@moves = split(/,/,$inp[1]);
($x, $y) = (0,0);
$steps = 0;
foreach (@moves) {
	die("parse $_") unless (/^([RUDL])(\d+)$/);
	my ($d,$len) = ($1,$2);
	my ($dx,$dy) = split(/,/,$delta{$d});
	for (my $i = 0; $i < $len; $i++) {
		$steps++;
		$x += $dx; $y += $dy;
		if (defined($points{"$x,$y"})) {
			my $manh = abs($x)+abs($y);
			if ($manh < $part1) {
#				printf("found shorter manhattan at ($x,$y)->$manh\n");
				$part1 = $manh;
			}
			my $signal = $steps + $points{"$x,$y"};
			if ($signal < $part2) {
#				printf("found closer combined signal at ($x,$y)->$signal\n");
				$part2 = $signal;
			}
		}
	}
}

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
