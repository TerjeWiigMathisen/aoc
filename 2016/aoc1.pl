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

my $start = time;

my $DEBUG = 0;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

my @i = ();
foreach (@inp) {
	push(@i, split(/,/));
}

my ($x,$y) = (0,0);
my @dir = (0,1);

my %visit = ("0,0" => 1);
my $part2;

foreach (@i) {
	if (/([RL])(\d+)$/) {
		my ($t,$l) = ($1,$2);
		if ($t eq 'R') {
			@dir = ($dir[1],-$dir[0]);
		}
		else { # L
			@dir = (-$dir[1],$dir[0]);
		}
		
		for (my $i = 0; $i < $l; $i++) {
			$x += $dir[0];
			$y += $dir[1];
			$visit{"$x,$y"}++;
			if (!defined($part2) && $visit{"$x,$y"} > 1) {
				$part2 = abs($x)+abs($y);
			}
		}
	}
}

my $part1 = abs($x)+abs($y);


printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
