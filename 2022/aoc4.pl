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

foreach (<F>) {
	chomp;
	#printf(STDERR "%s\n", $_);
	my ($f, $s) = split(/,/);
	my @f = split(/-/,$f);
	my @s = split(/-/,$s);
	if ($f[0] <= $s[0] && $f[1] >= $s[1]) { $part1++; }
	elsif ($f[0] >= $s[0] && $f[1] <= $s[1]) { $part1++; }
	
	elsif ($f[0] <= $s[0] && $f[1] >= $s[0]) { $part2++; }
	elsif ($f[0] <= $s[1] && $f[1] >= $s[1]) { $part2++; }
	
}
close(F);

$part2 += $part1;

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
