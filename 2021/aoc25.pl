#!perl -w

use strict;
use Time::HiRes qw (time);
use List::PriorityQueue;
use warnings;
#no warnings 'recursion';

my $start = time;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

my $DEBUG = 0;
my $part1;
my $part2;

my $ROWS = scalar(@inp);
my $COLS = length($inp[0]);

sub move
{
	my $cnt = 0;
	# Move east:
	for (my $y = 0; $y < $ROWS; $y++) {
		my $left = substr($inp[$y],-1,1).substr($inp[$y],0,1);
		my $l = substr($inp[$y],0,1);
		for (my $x = 1; $x < $COLS; $x++) {
			my $r = substr($inp[$y],$x,1);
			if ($l.$r eq '>.') {
				substr($inp[$y],$x-1,2) = '.>';
				$cnt++;
			}
			$l = $r;
		}
		if ($left eq '>.') {
			substr($inp[$y],0,1) = '>';
			substr($inp[$y],-1,1) = '.';
			$cnt++;
		}
	}
	# Move down
	for (my $x = 0; $x < $COLS; $x++) {
		my $top = substr($inp[-1],$x,1).substr($inp[0],$x,1);
		my $t = substr($inp[0],$x,1);
		for (my $y = 1; $y < $ROWS; $y++) {
			my $b = substr($inp[$y],$x,1);
			if ($t.$b eq 'v.') {
				substr($inp[$y-1],$x,1) = '.';
				substr($inp[$y],$x,1) = 'v';
				$cnt++;
			}
			$t = $b;
		}
		if ($top eq 'v.') {
			substr($inp[-1],$x,1) = '.';
			substr($inp[0],$x,1) = 'v';
			$cnt++;
		}
	}
	return $cnt;
}

sub dmp
{
	printf(STDERR "%s\n\n",join("\n",@inp));
}

for ($part1 = 1; $part1; $part1++) {
	my $cnt = move();
	if ($part1 % 10 == 0) {
		dmp();
	}
	last if ($cnt == 0);
}
dmp();

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %d\n", $part1);
printf("Part2: %1.0f\n", $part2);
