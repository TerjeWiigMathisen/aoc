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

my @h = (0,0);
my @t = @h;

my %v = ("0,0" => 1);
my %v2 = ("0,0" => 1);

sub follow
{
	my ($h0,$h1, $t0, $t1) = @_;
	my @d = ($h0-$t0,$h1-$t1);
	if (abs($d[0]) >= 2 && $d[1] == 0) {
		$t0 += $d[0] > 0 ? 1 : -1;
	}
	elsif (abs($d[1]) >= 2 && $d[0] == 0) {
		$t1 += $d[1] > 0 ? 1 : -1;
	}
	elsif ($d[0] && $d[1] && abs($d[0]*$d[1]) > 1) {
		$t0 += $d[0] > 0 ? 1 : -1;
		$t1 += $d[1] > 0 ? 1 : -1;
	}
	return ($t0,$t1);
}

my @tails = ();
foreach (0..9) {
	push(@{$tails[$_]},@h);
}

sub track
{
	my ($x, $y) = @_;
	return 'H' if ($x == $h[0] && $y == $h[1]);
	return '1' if ($x == $t[0] && $y == $t[1]);
	foreach (2..9) {
		my @tt = @{$tails[$_]};
		return $_ if ($x == $tt[0] && $y == $tt[1]);
	}
	return 'x' if (defined($v2{"$x,$y"}));
	return '#' if (defined($v{"$x,$y"}));
	return '.';
}

foreach (<F>) {
	chomp;
	my ($d, $l) = split;
	my @t9;
	for (my $m = 1; $m <= $l; $m++) {
		if ($d eq 'U') {
			$h[1]++;
		}
		elsif ($d eq 'D') {
			$h[1]--;
		}
		elsif ($d eq 'L') {
			$h[0]--;
		}
		elsif ($d eq 'R') {
			$h[0]++;
		}
		@t = follow(@h, @t);
		my $tail = $t[0].",".$t[1];
		$v{$tail}++;
		@{$tails[1]} = @t;
		foreach (2..9) {
			@{$tails[$_]} = follow(@{$tails[$_-1]}, @{$tails[$_]});
		}
		@t9 = @{$tails[9]};
		$tail = $t9[0].",".$t9[1];
		$v2{$tail}++;
	}
}
close(F);
printf(STDERR "Total time = %f\n", time - $start);

my ($x0,$y0,$x1,$y1) = (0,0,0,0);
foreach (keys %v) {
	my ($x,$y) = split(/,/, $_);
	$x0 = $x if ($x < $x0);
	$x1 = $x if ($x > $x1);
	$y0 = $y if ($y < $y0);
	$y1 = $y if ($y > $y1);
}

for (my $y = $y1; $y >= $y0; $y--) {
	for (my $x = $x0; $x < $x1; $x++) {
		printf("%s", track($x, $y));
	}
	printf("\n");
}
printf("\n");

printf(STDERR "Grid is %d x %d LL at %d,%d)\n", $x1-$x0+1, $y1-$y0+1, $x0,$y0);

$part1 = scalar(keys %v);
$part2 = scalar(keys %v2);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);
