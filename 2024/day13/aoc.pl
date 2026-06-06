#!perl -w
use strict;
use Time::HiRes qw (time);
use English;
use warnings;
no warnings 'recursion';
#use bigint;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

while (<>) {
	push(@lines, $_);
}

my $oneline = join('',@lines);

#part1
my @puzzles = split(/\n\n/,$oneline);

sub solve
{
	my ($ax,$ay,$bx,$by,$px,$py) = @_;

	# Floating point (double) calculations, so add a fudge to make sure
	# The int() truncation grabs the correct value:
	my $bp = int(($px*$ay-$py*$ax) / ($bx*$ay-$by*$ax)+0.0001);
	my $ap = int(($px-$bp*$bx)/$ax+0.0001);
	# Check the integer only answer, this fails for many
	if ($ap*$ax+$bp*$bx == $px && $ap*$ay+$bp*$by == $py) {
		printf("$ax,$ay,$bx,$by,$px,$py -> $ap,$bp\n");
		return $ap*3+$bp;
	}
	return 0;
}

sub solve2
{
	my ($puzzle, $offset) = @_;
	my ($ax,$ay,$bx,$by,$px,$py);
	my ($a,$b,$p) = split(/\n/,$puzzle);
	if ($a =~ /Button A:\s+X\+(\d+),\s+Y\+(\d+)\s*/) {
		($ax,$ay)=($1,$2);
		if ($b =~ /Button B:\s+X\+(\d+),\s+Y\+(\d+)\s*/) {
			($bx,$by)=($1,$2);
			if ($p =~ /Prize: X=(\d+), Y=(\d+)/) {
				($px,$py)=($1,$2);
			}
		}
	}
	if (!defined($py)) {
		printf("Input parsing error:\n%s\n", $puzzle);
		return (0,0);
	}
	my $p1 = solve($ax,$ay,$bx,$by,$px,$py,0);
	my $p2 = solve($ax,$ay,$bx,$by,$px+10000000000000,$py+10000000000000);
	return ($p1,$p2);
}

foreach (@puzzles) {
	my ($p1,$p2) = solve2($_);
	$part1 += $p1;
	$part2 += $p2;
}

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);

