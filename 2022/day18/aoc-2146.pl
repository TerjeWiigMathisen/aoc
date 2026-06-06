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

my %vox = (); # Global voxel store

my ($x0,$y0,$z0,$x1,$y1,$z1) = (1e8,1e8,1e8,-1e8,-1e8,-1e8);

sub cube
{
	my ($vox,$x,$y,$z) = @_;
	return (defined($vox->{"$x,$y,$z"})); # && $vox{"$x,$y,$z"});
}

sub surface_and_extent
{
	my ($vox, @voxels) = @_;
	my ($x,$y,$z) = splice(@voxels,0,3);
	my ($x0,$y0,$z0,$x1,$y1,$z1) = ($x,$y,$z,$x,$y,$z);
	$vox->{"$x,$y,$z"}++;
	my $surf = 6;
	while (scalar(@voxels)) {
		($x,$y,$z) = splice(@voxels,0,3);
		$vox->{"$x,$y,$z"}++;
		$surf += 6;
		if (cube($vox,$x,$y,$z+1)) { $surf-=2; }
		if (cube($vox,$x,$y,$z-1)) { $surf-=2; }
		if (cube($vox,$x,$y+1,$z)) { $surf-=2; }
		if (cube($vox,$x,$y-1,$z)) { $surf-=2; }
		if (cube($vox,$x+1,$y,$z)) { $surf-=2; }
		if (cube($vox,$x-1,$y,$z)) { $surf-=2; }
		
		$x0 = $x if ($x < $x0);
		$y0 = $y if ($y < $y0);
		$z0 = $z if ($z < $z0);

		$x1 = $x if ($x > $x1);
		$y1 = $y if ($y > $y1);
		$z1 = $z if ($z > $z1);
	}
	return ($surf, $x0,$y0,$z0,$x1,$y1,$z1);
}

my @voxels = ();
foreach (@inp) {
	chomp;
	my ($x,$y,$z) = split(/,/);
	push(@voxels,$x,$y,$z);
}

#my ($x0,$y0,$z0,$x1,$y1,$z1);
#my %vox;
($part1, $x0,$y0,$z0,$x1,$y1,$z1) = surface_and_extent(\%vox,@voxels);

printf("Extent (%d,%d,%d) - (%d,%d,%d)\n", $x0,$y0,$z0,$x1,$y1,$z1);

printf("Part1: %s\n", $part1);
printf(STDERR "Total time = %f\n", time - $start);

sub find_holes
{
	my ($vox, @voxels) = @_;
	my %v;
	my ($surf,$x0,$y0,$z0,$x1,$y1,$z1) = surface_and_extent(\%v, @voxels);
# Scan over enclosing volume
	my %tried = ();
	for (my $x = $x0; $x <= $x1; $x++) {
		for (my $y= $y0; $y <= $y1; $y++) {
			for (my $z = $z0; $z <= $z1; $z++) {
				if (!cube($vox,$x,$y,$z) && !$tried{"$x,$y,$z"}++) { 
					if ($x == 10 && $y == 10 && $z == 10) {
						printf("Starting hole at $x,$y,$z\n");
					}
					my @void = internal_volume($vox, \%tried, $x,$y,$z);# Will flood fill to boundary!
					next unless (scalar(@void));
					my %vi;
					my ($surfi,$x0i,$y0i,$z0i,$x1i,$y1i,$z1i) = surface_and_extent(\%vi, @void);
					if ($surfi) {
						printf("Void volume %d, surface %d\n", scalar(@void)/3, $surfi);
						$part2 -= $surfi;
					}
				}
			}
		}
	}
}
	
# Find a non-cube, check if it connects internally only

my %tried;

sub internal_volume
{
	my ($vox, $tried, $x,$y,$z) = @_;

	my %seen = ();
	my @seed = ($x,$y,$z);
	$tried->{"$x,$y,$z"} = 0;
	while (scalar(@seed)) {
		($x,$y,$z) = splice(@seed,0,3);
		next if (cube($vox,$x,$y,$z));
		next if ($tried->{"$x,$y,$z"}++);
		
		if ($x < $x0 || $y < $y0 || $z < $z0 || $x > $x1 || $y > $y1 || $z > $z1) {
			# Hit boundary!
			return ();
		}
		next if ($seen{"$x,$y,$z"}++);
		
		push(@seed,$x-1,$y,$z);
		push(@seed,$x+1,$y,$z);
		push(@seed,$x,$y-1,$z);
		push(@seed,$x,$y+1,$z);
		push(@seed,$x,$y,$z-1);
		push(@seed,$x,$y,$z+1);
	}
	return () unless (scalar(keys %seen));
	printf("Found internal volume of size %d\n", scalar(keys %seen)); #, join(" ",keys %seen));
	my @v = ();
	foreach (keys %seen) { push(@v,split(/,/)); }
	return @v;
}

$part2 = $part1;

#my %tried;
#my @void = internal_volume(\%vox,\%tried,10,10,10);
find_holes(\%vox, @voxels);

printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);
