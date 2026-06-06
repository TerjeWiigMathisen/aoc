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

my @stacks = ();

my @lines = ();
my @c;
foreach (<F>) {
	chomp;
	@c = split(//);
	for (my $i = 4; $i <= scalar(@c); $i++) {
		my ($a,$b,$c,$d) = @c[$i-4..$i-1];
		next if ($a eq $b || $a eq $c || $a eq $d);
		next if ($b eq $c || $b eq $d);
		next if ($c eq $d);
		$part1 = $i;
		last;
	}
}
close(F);

for (my $i = 14; $i <= scalar(@c); $i++) {
	my %seen = ();
	my $ok = 0;
	for (my $j = $i-14; $j < $i; $j++) {
		$seen{$c[$j]}++;
		$ok = 1;
		if ($seen{$c[$j]}>1) {
			$ok = 0;
			last;
		}
	}
	if ($ok) {
		$part2 = $i;
		last;
	}
}

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
