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

my ($x0,$x1, $y0,$y1);
if ($inp[0] =~ /target area: x=([\-\d]+)\.\.([\-\d]+), y=([\-\d]+)\.\.([\-\d]+)$/) {
	($x0,$x1, $y0,$y1) = ($1,$2, $3, $4);
}

# target area: x=20..30, y=-10..-5

sub hit
{
	my ($vx,$vy) = @_;
	my ($x, $y) = 0;
	my $top = 0;
	
	while ($x < $x0) {
		$x += $vx; $vx-- if ($vx);
		$y += $vy; $vy--;
		$top = $y if ($y > $top);
		return (-1,undef) if ($y < $y0);
	}
	return (1,undef) if ($x > $x1);
	return (-1,undef) if ($y < $y0);
	while ($x <= $x1) {
		return (-2,undef) if ($y < $y0); # Jumped past!
		return (0, $top) if ($y <= $y1);
		$x += $vx; $vx-- if ($vx);
		$y += $vy; $vy--;
		$top = $y if ($y > $top);
	}
	return (2,undef);
}

my @hits = ();

sub try
{
	my ($bvx, $bvy, $btop) = (0,0,0);
	
	for (my $vx = 1; $vx < $x0; $vx++) {
		for (my $vy = $y0; $vy <= 500; $vy++) {
			my ($r, $t) = hit($vx, $vy);
#			printf(STDERR "%d,%d -> %d", $vx, $vy, $r);
			if (defined($t)) {
				push(@hits,"$vx,$vy");
				printf("Hit: %d, %d\n", $vx, $vy);

				if ($t > $btop) {
					$btop = $t;
					$bvx = $vx;
					$bvy = $vy;
					printf(STDERR "Higher = %d,%d -> %d\n", $bvx, $bvy, $btop);
				}
			}
		}
	}
	printf(STDERR "Best = %d,%d -> %d\n", $bvx, $bvy, $btop);
	return $btop;
}

my $part1 = try ();

my $part2 = scalar(@hits) + ($x1-$x0+1)*($y1-$y0+1);

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %d\n", $part1);
printf("Part2: %d\n", $part2);
