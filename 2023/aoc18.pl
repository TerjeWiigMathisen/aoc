#!perl -w
use strict;
use Time::HiRes qw(time);
#use List::PriorityQueue;
use Math::Polygon::Calc;

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @inp = (<>); chomp(@inp);
#printf("Input:\n%s\n\n", join("\n",@inp));

sub dig1
{
	my (@inp) = @_;
	my ($x,$y) = (0,0);
	my @poly = ([$x,$y]);
	my @dx = (1,0,-1,0);
	my @dy = (0,1,0,-1);
	my %d2dir = ('R' => 0,'D' => 1, 'L' => 2, 'U' => '3');
	foreach (@inp) {
		my ($d,$l,$c) = split;
		$d = $d2dir{$d};
		$x += $dx[$d]*$l; $y += $dy[$d]*$l;
		push(@poly,[$x,$y]);
	}
	my $area = polygon_area(@poly)+polygon_perimeter(@poly)*0.5+1;
	return $area;
}

sub dig2
{
	my (@inp) = @_;
	my ($x,$y) = (0,0);
	my @poly = ([$x,$y]);
	my @dx = (1,0,-1,0);
	my @dy = (0,1,0,-1);
	foreach (@inp) {
		my ($d,$l,$c) = split;
		$d = substr($c,-2,1);
		$l = hex(substr($c,2,5));
		$x += $dx[$d]*$l; $y += $dy[$d]*$l;
		push(@poly,[$x,$y]);
	}
	my $area = polygon_area(@poly)+polygon_perimeter(@poly)*0.5+1;
	return $area;
}

$part1 = dig1(@inp);
$part2 = dig2(@inp);
my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.6fs\n", $used);
