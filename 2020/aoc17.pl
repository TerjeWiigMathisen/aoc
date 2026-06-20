#!perl -w 

use strict;
use Time::HiRes qw(time);
#use bigint;

my $inp = 
q();

$inp = q
(.#.
..#
###);
my ($x0,$y0,$z0,$w0,$x1,$y1,$z1,$w1) = (0,0,0,0,2,2,0,0);

if (defined(shift)) {
	$inp = q
(..##.#.#
.#####..
#.....##
##.##.#.
..#...#.
.#..##..
.#...#.#
#..##.##);
($x1,$y1) = (7,7);
}

my $t = time();

my %dim = ();

my @slices = split(/\n\n/,$inp);
my $z = 0;
my $w = 0;
foreach (@slices) {
	my $slice = $_;
	my @lines = split(/\n/);
	my $y = 0;
	foreach (@lines) {
		my @line = split(//);
		for (my $x = 0; $x < scalar(@line); $x++) {
			$dim{"$x,$y,$z,$w"} = $line[$x];
		}
		$y++;
	}
	$z++;
}

sub total_active
{
	my $active = 0;
	foreach (keys %dim) {
		$active += $dim{$_} eq '#';
	}
	return $active;
}

sub active
{
	my ($x, $y, $z, $w) = @_;
	my $key = "$x,$y,$z,$w";
	return defined($dim{$key}) && $dim{$key} eq '#';
}

sub volume_active
{
	my ($x0, $y0, $z0, $w0, $x1, $y1, $z1, $w1) = @_;
	my $total = 0;
	for (my $nw = $w0; $nw <= $w1; $nw++) {
		for (my $nz = $z0; $nz <= $z1; $nz++) {
			for (my $ny = $y0; $ny <= $y1; $ny++) {
				for (my $nx = $x0; $nx <= $x1; $nx++) {
					my $n = active($nx, $ny, $nz,$nw);
					$total += $n;
				}
			}
		}
	}
	return $total;
}
	

sub neighbors
{
	my ($x, $y, $z, $w) = @_;
	my ($total, $curr) = (0,0);
	$total = volume_active($x-1,$y-1,$z-1,$w-1, $x+1, $y+1, $z+1,$w+1);
	$curr = active($x, $y, $z,$w);
	$total -= $curr;
	return ($curr, $total);
}

sub generation
{
	my ($x0,$y0,$z0,$w0,$x1,$y1,$z1,$w1) = (1,1,1,1,-1,-1,-1,-1);
	foreach (keys %dim) {
		my ($x,$y,$z,$w) = split(/,/);
		$x0 = $x-1 if ($x <= $x0);
		$x1 = $x+1 if ($x >= $x1);
		$y0 = $y-1 if ($y <= $y0);
		$y1 = $y+1 if ($y >= $y1);
		$z0 = $z-1 if ($z <= $z0);
		$z1 = $z+1 if ($z >= $z1);
		$w0 = $w-1 if ($w <= $w0);
		$w1 = $w+1 if ($w >= $w1);
	}
	my %next = ();
	for (my $w = $w0; $w <= $w1; $w++) {
		for (my $z = $z0; $z <= $z1; $z++) {
			for (my $y = $y0; $y <= $y1; $y++) {
				for (my $x = $x0; $x <= $x1; $x++) {
					my ($curr, $total) = neighbors($x,$y,$z,$w);
					if ($total == 3 || ($curr+$total) == 3) {
						$next{"$x,$y,$z,$w"} = '#';
					}
				}
			}
		}
	}
	%dim = ();
	foreach (keys %next) {
		$dim{$_} = $next{$_};
	}
}

for (my $g = 1; $g <= 6; $g++) {
	generation();
	my $active = total_active();
	printf("%3d %5d\n",$g, $active);
}	

$t = time()-$t;
printf("Total time = %1.5fms\n", $t*1000);

exit();

