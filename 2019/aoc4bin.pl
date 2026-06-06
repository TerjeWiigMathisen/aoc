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

sub ascii2bin5
{
	my ($a) = @_;
	my $n = 0;
	foreach(split(//,$a)) {
		$n = ($n << 5) + $_;
	}
	return $n;
}

sub bin5
{
	my ($n) = @_;
	my $s = '';
	do {
		$s = ($n & 15).$s;
		$n >>= 5;
	} while ($n);
	return $s;
}

sub digtest
{
	my ($digits) = @_;
	my (@digits) = split(//,$digits);
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
	return ($double, $notriple);
}

my $digits = ascii2bin5(join('',@digits));
$end = ascii2bin5($end);

my $iterations = 0;
my $mask1 = 1+(1<<5)+(1<<10)+(1<<15)+(1<<20)+(1<<25)+(1<<30);
my $mask16 = $mask1 << 4;
my $mask15 = $mask16-$mask1;
my $mask111 = 1+(1<<5)+(1<<10);
my $mask101 = 1 + (1 << 10);
while ($digits <= $end) {
#	printf(STDERR "Testing %x = %d\n", $digits,bin5($digits));
	$iterations++;
	my $n1 = $digits << 5;
	my $eq = ($digits ^ $n1) | (1 + (1 << 30)); # The bottom or top slots cannot be equal!
	$eq += $mask15;
	$eq &= $mask16; # Top bit set for all non-equal
	$eq >>= 4; # Bottom bit => not part of a pair
	my $double = ($eq != $mask1); # ? 1 : 0;
	my $e = $eq & (~($eq << 5)) & ($eq << 10);
	my $not_triple = $e != 0;
#	printf("N = %s, EQ = %03D, pair = %d, not_triple = %d\n",bin5($digits),bin5($eq),$double,$not_triple);
	$part1 += $double;
	$part2 += $not_triple;
#	my ($d,$n) = digtest(bin5($digits));
#	if ($d != $double || $n != $not_triple) {
#		printf("Error at %s: Facit = %d, %d, got %d,%d\n", bin5($digits),$d,$n,$double,$not_triple);
#		($d,$n) = digtest(bin5($digits));
#		die;
#	}
	# Increment the number as ascii digits
	my $mask = 15;
	my $nine = 9;
	my $inc = 1;
#	printf("Before increment: %s",bin5($digits));
	while ((($digits+=$inc) & $mask) > $nine) {
		$inc <<= 5;
		$mask <<= 5;
		$nine <<= 5;
	}
	my $dig = $digits & $mask;
	while ($mask > 15) {
		$mask >>= 5;
		$dig >>= 5;
		$digits = ($digits & ~$mask) | $dig;
	}
#	printf("   after: %s\n",bin5($digits));
}

printf(STDERR "%d iterations\n",$iterations);

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
