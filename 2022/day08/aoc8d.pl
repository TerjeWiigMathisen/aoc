#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);
#use List::Util qw (reduce);

#use bigint;

#use JSON::Parse;
#no warnings 'recursion';

my $DEBUG = 0;

my $fname = shift;
open(F ,'<',$fname);

my $start = time;

my $part1 = 0;
my $part2 = 0;

my $l = 0;
my %g = ();
my $LINES;
my $ROWS;

foreach (<F>) {
	chomp;
	my @l = split(//);
	$ROWS = scalar(@l);
	my $r = 0;
	foreach (@l) {
		$g{"$l,$r"} = $_;
		$r++;
	}
	$l++;
}
$LINES = $l;
close(F);
printf(STDERR "Grid is %d x %d\n", $ROWS, $LINES);

my %v = ();
for (my $l = 0; $l < $LINES; $l++) {
	my $h = -1;
	for (my $r = 0; $r < $ROWS; $r++) {
		if ($g{"$l,$r"} > $h) {
			$v{"$l,$r"}++;
			$h = $g{"$l,$r"};
		}
	}
	$h = -1;
	for (my $r = $ROWS-1; $r; $r--) {
		if ($g{"$l,$r"} > $h) {
			$v{"$l,$r"}++;
			$h = $g{"$l,$r"};
		}
	}
}

for (my $r = 0; $r < $ROWS; $r++) {
	my $h = -1;
	for (my $l = 0; $l < $LINES; $l++) {
		if ($g{"$l,$r"} > $h) {
			$v{"$l,$r"}++;
			$h = $g{"$l,$r"};
		}
	}
	$h = -1;
	for (my $l = $LINES-1; $l; $l--) {
		if ($g{"$l,$r"} > $h) {
			$v{"$l,$r"}++;
			$h = $g{"$l,$r"};
		}
	}
}
printf("Visible = %d\n", scalar keys %v);
$part1 = scalar keys %v;

for (my $l = 1; $l < $LINES-1; $l++) {
	for (my $r = 1; $r < $ROWS-1; $r++) {
		#next if ($v{"$l,$r"}); # Only invisible locations
		#printf(STDERR "Trying %d, %d\n", $l, $r);
		
		my ($up, $dn, $lt, $rt);
		my ($x, $y);
		my $curr = $g{"$l,$r"};
		$y = $l;

		my $max = -1;
		my $cnt = 0;
		for ($x = $r-1; $x >= 0; $x--) {
#			if ($g{"$y,$x"} >= $max ) {
				$cnt++;
				$max = $g{"$y,$x"};
				last if ($max >= $curr);
#			}
		}
		$lt = $cnt;

		$max = -1;
		$cnt = 0;
		for ($x = $r+1; $x < $ROWS; $x++) {
#			if ($g{"$y,$x"} >= $max ) {
				$cnt++;
				$max = $g{"$y,$x"};
				last if ($max >= $curr);
#			}
		}
		$rt = $cnt;
		
		$x = $r;

		$max = -1;
		$cnt = 0;
		for ($y = $l-1; $y >= 0; $y--) {
#			if ($g{"$y,$x"} >= $max ) {
				$cnt++;
				$max = $g{"$y,$x"};
				last if ($max >= $curr);
#			}
		}
		$up = $cnt;

		$max = -1;
		$cnt = 0;
		for ($y = $l+1; $y < $LINES; $y++) {
#			if ($g{"$y,$x"} >= $max ) {
				$cnt++;
				$max = $g{"$y,$x"};
				last if ($max >= $curr);
#			}
		}
		$dn = $cnt;
		
		my $p = $lt * $rt * $up * $dn;
		if ($p > $part2) {
			$part2 = $p;
#			printf(STDERR "New max (%d = %d*%d*%d*%d) found at (%d,%d)\n", $p, $lt, $rt, $up, $dn, $l+1, $r+1);
		}
#		elsif ($p > $part2 / 2) {
#			printf(STDERR "New --- (%d = %d*%d*%d*%d) found at (%d,%d)\n", $p, $lt, $rt, $up, $dn, $l+1, $r+1);
#		}
	}
}

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);