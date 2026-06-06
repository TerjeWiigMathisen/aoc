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
$fname = 'input.txt' unless (defined($fname));
open(F ,'<',$fname);

my $start = time;

my $part1 = 0;
my $part2 = 0;

my ($t, $x, $look, $screen, $row) = (0,1,20,'',0);

sub inct
{
	$t++;
	if ($t == $look) {
		my $s = $t * $x;
		$part1 += $s;
		$look += 40;
	}
	if ($row >= $x-1 && $row <= $x+1) {
		$screen .= '#';
	}
	else {
		$screen .= '.';
	}
	$row++;
	if ($row >= 40) {
		$screen .= "\n";
		$row = 0;
	}
}

foreach (<F>) {
	chomp;
	my ($o, $d) = split;
	if ($o eq 'noop') {
		inct();
	}
	elsif ($o eq 'addx') {
		inct(); inct();
		$x += $d;
	}
}
close(F);
printf(STDERR "Total time = %f\n", time - $start);

printf("Part1: %s\n", $part1);
printf("Part2:\n%s\n", $screen);

printf(STDERR "Total time = %f\n", time - $start);
