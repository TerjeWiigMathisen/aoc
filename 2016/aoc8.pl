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
my $part2 = 0;

my @disp;
foreach (1..6) {
	push(@disp,"." x 50);
}

my $ROWS = 6;
my $COLS = 50;

foreach (@inp) {
	if (/rect (\d+)x(\d+)$/) {
		for (my $y = 0; $y < $2; $y++) {
			substr($disp[$y],0,$1) = '#' x $1;
		}
	}
	elsif (/rotate column x=(\d+) by (\d+)$/) {
		my ($c, $n) = ($1, $2);
		$n %= $ROWS;
		my $col = '';
		for (my $y = 0; $y < $ROWS; $y++) {
			$col .= substr($disp[$y],$c,1);
		}
		# Rotate it:
		$col = substr($col,-$n).substr($col,0,$ROWS-$n);
		for (my $y = 0; $y < $ROWS; $y++) {
			substr($disp[$y],$c,1) = substr($col,$y,1);
		}
	}
	elsif (/rotate row y=(\d+) by (\d+)$/) {
		my ($r, $n) = ($1, $2);
		$n %= $COLS;
		my $col = $disp[$r];
		# Rotate it:
		$col = substr($col,-$n).substr($col,0,$COLS-$n);
		$disp[$r] = $col;
	}
}

my $d = join("", @disp);
$d =~ s/[^#]//g;

$part1 = length($d);

printf(STDERR "Total time = %f\n", time - $start);

printf("%s\n\n", join("\n", @disp));

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
