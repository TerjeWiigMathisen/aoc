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


my ($x,$y) = (0,0);

my $part1 = 0;
my $part2 = '';

foreach (@inp) {
	my @m = split(//);
	foreach (@m) {
		if (/U/) {
			$y++ unless ($y >= 1);
		}
		elsif (/D/) {
			$y-- unless ($y <= -1);
		}
		elsif (/R/) {
			$x++ unless ($x >= 1);
		}
		elsif (/L/) {
			$x-- unless ($x <= -1);
		}
	}
	$part1 = $part1*10 + 5+$x - ($y*3);
}

my @pad = (".......","...1...","..234..",".56789.","..ABC..","...D...",".......");

($x,$y) = (1,3);
foreach (@inp) {
	my @m = split(//);
	foreach (@m) {
		if (/D/) {
			$y++ unless (substr($pad[$y+1],$x,1) eq '.');
		}
		elsif (/U/) {
			$y-- unless (substr($pad[$y-1],$x,1) eq '.');
		}
		elsif (/R/) {
			$x++ unless (substr($pad[$y],$x+1,1) eq '.');
		}
		elsif (/L/) {
			$x-- unless (substr($pad[$y],$x-1,1) eq '.');
		}
	}
	$part2 .= substr($pad[$y],$x,1);
}


printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
