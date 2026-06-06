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

my $part1;
my $part2 = 0;

sub reduce
{
	my ($count) = @_;
	my $pos = 0;
	my $step = 1;
	for (my $c = $count; ($c & 1) == 0; $c >>= 1) {
		$step *= 2;
	}
	my @inp = ();
	for (my $i = 1; $i <= $count; $i+=$step) {
		push(@inp,$i);
	}
	
	while (scalar(@inp) > 1) {
		my @o = ();
		my $i;
		for ($i = $pos; $i < scalar(@inp); $i += 2) {
			push(@o, $inp[$i]);
		}
		# Odd number in the current round?
		if (scalar(@inp) & 1) {
			# If we hit the last position, then we start at the second in the next round:
			$pos ^= 1;
		}
		@inp = @o;
	}
	return $inp[0];
}

sub reduce2 # Hardcore approach, doing one elf/iteration, copying around
{
	my ($count) = @_;
	my $pos = 0;
	my $step = 1;
	for (my $c = $count; ($c & 1) == 0; $c >>= 1) {
#		$step *= 2;
	}
	my @inp = ();
	for (my $i = 1; $i <= $count; $i+=$step) {
		push(@inp,$i);
	}
	
	while (scalar(@inp) > 1) {
		printf("%s\n", join(",",@inp)) if ($DEBUG);
		my $cut = splice(@inp,scalar(@inp)>>1,1);
		my $first = shift @inp;
		push(@inp,$first);
	}
	printf("%d\n",$inp[0]) if ($DEBUG);
	return $inp[0];
}

sub reduce_loop
{
	my ($count) = @_;
	my $p = 2;
	for (my $c = 6; $c <= $count; $c++) { # Remove slot ($c>>1)+1, move 1 to the end
		my $skip = ($c>>1)+1;
		$p++;
		if ($p >= $skip) { $p++; }
		if ($p > $c) { # Past the end?
			$p = 1;
		}
			
	}
	return $p;
}
			
$DEBUG = 0;
#$part1 = reduce(5);
$part1 = reduce(3012210);

for (my $r = 5; $r < 5; $r++) {
	printf("%2d %d %d\n", $r, reduce2($r), reduce_loop($r));
}
#$part2 = reduce2(5);
$part2 = reduce_loop(3012210);

printf(STDERR "Total time = %f\n", time - $start);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);

