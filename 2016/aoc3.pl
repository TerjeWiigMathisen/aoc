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

my $part1 = 0;
my $part2 = '';

foreach (@inp) {
	my @m = sort {$a <=> $b} split;
	if ($m[0]+$m[1] > $m[2]) {
		$part1++;
	}
}

my @m = ();
foreach (@inp) {
	push(@m,split);
	if (scalar(@m) == 9) {
		for (my $i = 0; $i < 3; $i++) {
			my @t = sort {$a <=> $b} ($m[$i],$m[$i+3],$m[$i+6]);
			if ($t[0]+$t[1] > $t[2]) {
				$part2++;
			}
		}
		@m = ();
	}
}

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
