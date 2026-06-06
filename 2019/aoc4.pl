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

my ($begin, $end) = split(/\-/,$inp[0]);
my $p = $begin;
my @digits = split(//,$p);
# Start by advancing to first non-decreasing digits value:
# This is a single iteration over the 6 digits, so O(1);
for (my $i=1; $i<6;$i++) {
	if ($digits[$i-1] > $digits[$i]) {
		my $c = $digits[$i-1];
		for (my $j = $i; $j < 6; $j++) {
			$digits[$j] = $c;
		}
		last;
	}
}
my $iterations = 0;
while (join('',@digits) le $end) {
	$iterations++;
	my $cdouble = 0; # Current pair a double?
	my $pdouble = 0; # previous pair ditto?
	my $ppdouble = 0;# prev-prev pair ditto?

	my $double = 0; # Have we found a pair of repeating digits?
	my $notriple = 0; # which was not part of a triple?
	for (my $d = 0; $d < 5; $d++) {
		$ppdouble = $pdouble; # Copy from previous iteration
		$pdouble = $cdouble;  # This avoids the need for multiple pair comparisons
		$cdouble = $digits[$d+1] == $digits[$d]; # Is this a pair?
		$double |= $cdouble;
		$notriple |= !$ppdouble && $pdouble && !$cdouble; # nopair, pair, nopair
	}
	$part1 += $double;
	$part2 += $notriple;
	# Increment the number as ascii digits
	my $d = 5;
#	printf("Before increment: %s",join('',@digits));
	while ($digits[$d]++ == 9) {
		$d--;
	}
	for (my $i = $d+1; $i < 6; $i++) {
		$digits[$i] = $digits[$d];
	}
#	printf("   after: %s\n",join('',@digits));
}

printf(STDERR "%d iterations\n",$iterations);

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
