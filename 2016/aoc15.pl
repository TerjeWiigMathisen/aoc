#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);
use List::PriorityQueue;
use Digest::MD5;

#use bigint;

#use JSON::Parse;
no warnings 'recursion';

my $start = time;

my $DEBUG = 0;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

my @disk = ();
my @posn = ();
my $rev = 1;
foreach (@inp) {
	if (/Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+)\./) {
		my ($disk, $size, $pos) = ($1,$2,$3);
		push(@disk, $size);
		push(@posn, ($pos + $rev) % $size);
		printf("Disk %d size=%d, pos=%d\n", $disk, $size, $pos);
		$rev++;
	}
}

push(@disk,11);
push(@posn,7);

my $part1;
my $part2;

my $step = 1;
my $t = 0;
for (my $d = 0; $d < scalar(@disk); $d++) {
	my $s = $disk[$d];
	my $p = ($posn[$d] + $t) % $s;
	while ($p) {
		$t += $step;
		$p = ($p + $step) % $s;
	}
	$part1 = $t if ($d == 5);
	printf("Disk %d (size %2d, start %2d) in sync t=%d\n", $d, $s, $posn[$d], $t);
	$step *= $s;
}
$part2 = $t;

#push(@disk,11);
#push(@posn,7);

printf(STDERR "Total time = %f\n", time - $start);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
