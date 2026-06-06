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

my %outside = ();

sub scan_surface
{
	my ($vox, $x,$y,$z) = @_;
	$z--;
	die("rock!") if (cube($vox,$x,$y,$z));
	$z0--; $x0--; $y0--; $x1++; $y1++; $z1++; # Extend all boundaries so we completely surround the rocks

	my $surface = 0;
	my %seen;
	my @seed = ($x,$y,$z); # Seed position outside everything!
	while (scalar(@seed)) {
		($x,$y,$z) = splice(@seed,0,3);
		if (cube($vox,$x,$y,$z)) { # Hit a cube!
			$surface++;
			next;
		}
		next if ($seen{"$x,$y,$z"}++);

		if ($x < $x0 || $y < $y0 || $z < $z0-1 || $x > $x1 || $y > $y1 || $z > $z1) {
			# Hit boundary!
			next;
		}

		push(@seed,$x-1,$y,$z);
		push(@seed,$x+1,$y,$z);
		push(@seed,$x,$y-1,$z);
		push(@seed,$x,$y+1,$z);
		push(@seed,$x,$y,$z-1);
		push(@seed,$x,$y,$z+1);
	}
	return $surface;
}

$part2 = scan_surface(\%vox, $x0,$y0,$z0);
printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);

die;

sub find_holes
{
	my ($add, $vox, $out, @voxels) = @_;
	my %v;
	my ($surf,$x0,$y0,$z0,$x1,$y1,$z1) = surface_and_extent(\%v, @voxels);
# Scan over enclosing volume
	my %tried = ();
	for (my $x = $x0; $x <= $x1; $x++) {
		for (my $y= $y0; $y <= $y1; $y++) {
			my $pc = 0;
			for (my $z = $z0; $z <= $z1; $z++) {
				my $c = cube($vox,$x,$y,$z);
				if ($pc & !$c) { # We have passed a wall
					next if $tried{"$x,$y,$z"}++;
					#if ($x == 10 && $y == 10 && $z == 10) {
					#	printf("Starting hole at $x,$y,$z\n");
					#}
					my @void = internal_volume($vox, \%tried, $x,$y,$z);# Will flood fill to boundary!
					next unless (scalar(@void));
					my %vi;
					my ($surfi,$x0i,$y0i,$z0i,$x1i,$y1i,$z1i) = surface_and_extent(\%vi, @void);
					if ($surfi) {
						printf("Void(%d,%d,%d) volume %d, surface %d\n", $x,$y,$z,scalar(@void)/3, $surfi);
						$part2 += $add*$surfi;
						if (scalar(@void) >= 26) { # Can have internal holes!
							my @v = @void;
							while (scalar(@v)) {
								my ($x,$y,$z) = splice(@v,0,3);
								printf("%d,%d,%d\n",$x,$y,$z);
							}
							my %vi;
							find_holes(-$add, \%vi, @void);
							printf("Done looking for inner holes\n");
						}
					}
				}
				$pc = $c;
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
		next if ($tried->{"$x,$y,$z"});
		
		if ($x < $x0 || $y < $y0 || $z < $z0 || $x > $x1 || $y > $y1 || $z > $z1) {
			# Hit boundary!
			foreach (keys %seen) {
				$tried->{$_}++;
			}
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
	#printf("Found internal volume of size %d\n", scalar(keys %seen)); #, join(" ",keys %seen));
	my @v = ();
	foreach (keys %seen) { 
		push(@v,split(/,/)); 
		$tried->{$_}++;
	}
	return @v;
}

#my %tried;
#my @void = internal_volume(\%vox,\%tried,10,10,10);
find_holes(-1, \%vox, @voxels);

printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);
