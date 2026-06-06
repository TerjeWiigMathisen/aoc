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


my @inp = ();
if ($fname eq 't.txt') {
}

foreach (<F>) {
	chomp;
	push(@inp,$_);
}
close(F);

my %vox = ();

my ($x0,$y0,$z0,$x1,$y1,$z1) = (1e8,1e8,1e8,-1e8,-1e8,-1e8);

sub cube
{
	my ($x,$y,$z) = @_;
	return (defined($vox{"$x,$y,$z"}) && $vox{"$x,$y,$z"});
}

foreach (@inp) {
	chomp;
	my ($x,$y,$z) = split(/,/);
	$vox{$_}++;
	$part1 += 6;
	if ($vox{sprintf("%d,%d,%d",$x,$y,$z+1)}) { $part1-=2; }
	if ($vox{sprintf("%d,%d,%d",$x,$y,$z-1)}) { $part1-=2; }
	if ($vox{sprintf("%d,%d,%d",$x,$y+1,$z)}) { $part1-=2; }
	if ($vox{sprintf("%d,%d,%d",$x,$y-1,$z)}) { $part1-=2; }
	if ($vox{sprintf("%d,%d,%d",$x+1,$y,$z)}) { $part1-=2; }
	if ($vox{sprintf("%d,%d,%d",$x-1,$y,$z)}) { $part1-=2; }
	
	$x0 = $x if ($x < $x0);
	$y0 = $y if ($y < $y0);
	$z0 = $z if ($z < $z0);
	$x1 = $x if ($x > $x1);
	$y1 = $y if ($y > $y1);
	$z1 = $z if ($z > $z1);
}

printf("Extent (%d,%d,%d) - (%d,%d,%d)\n", $x0,$y0,$z0,$x1,$y1,$z1);

printf("Part1: %s\n", $part1);

printf(STDERR "Total time = %f\n", time - $start);

# Find a non-cube, check if it connects internally only

my %tried;

sub internal_surface
{
	my ($x,$y,$z) = @_;

	my %seen = ();
	my @seed = @_;
	while (scalar(@seed)) {
		($x,$y,$z) = splice(@seed,0,3);
		return 0 if ($tried{"$x,$y,$z"}++);
		if ($x < $x || $y < $y0 || $z < $z0 || $x > $x1 || $y > $y1 || $z > $z1) {
			# Hit boundary!
			return 0;
		}
		$seen{"$x,$y,$z"}++;
		push(@seed,$x-1,$y,$z) unless (cube($x-1,$y,$z) || $seen{sprintf("%d,%d,%d",$x-1,$y,$z)});
		push(@seed,$x+1,$y,$z) unless (cube($x+1,$y,$z) || $seen{sprintf("%d,%d,%d",$x+1,$y,$z)});
		push(@seed,$x,$y-1,$z) unless (cube($x,$y-1,$z) || $seen{sprintf("%d,%d,%d",$x,$y-1,$z)});
		push(@seed,$x,$y+1,$z) unless (cube($x,$y+1,$z) || $seen{sprintf("%d,%d,%d",$x,$y+1,$z)});
		push(@seed,$x,$y,$z-1) unless (cube($x,$y,$z-1) || $seen{sprintf("%d,%d,%d",$x,$y,$z-1)});
		push(@seed,$x,$y,$z+1) unless (cube($x,$y,$z+1) || $seen{sprintf("%d,%d,%d",$x,$y,$z+1)});
	}
	printf("Found internal volume of size %d: %s\n", scalar(keys %seen),
		join(" ",keys %seen));
	my $s = scalar(keys %seen);
	return 6 if ($s == 1);
	return 10 if ($s == 2);
	die;
}

$part2 = $part1;

# Scan over enclosing volume
for (my $x = $x0; $x <= $x1; $x++) {
	for (my $y= $y0; $y <= $y1; $y++) {
		for (my $z = $z0; $z <= $z1; $z++) {
			if (!cube($x,$y,$z)) { 
				$part2 -= internal_surface($x,$y,$z);# Will flood fill to boundary!
			}
		}
	}
}

printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);
