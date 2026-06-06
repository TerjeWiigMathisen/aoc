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
	$x1 = $x if ($x > $x0);
	$y1 = $y if ($y > $y0);
	$z1 = $z if ($z > $z0);
}

printf("Extent (%d,%d,%d) - (%d,%d,%d)\n", $x0,$y0,$z1,$x1,$y1,$z1);

# Color external surface, start with first droplet found along a diagonal:

my ($x,$y,$z) = (0,0,0);
my @dir;
while (1) {
	$x++;
	if ($vox{"$x,$y,$z"}) {
		printf("Found cube at %d,%d,%d\n", $x,$y,$z);
		printf("Open surface towards (%d,%d,%d)\n",$x-1,$y,$z);
		@dir = (-1,0,0);
		last;
	}
	$y++;
	if ($vox{"$x,$y,$z"}) {
		printf("Found cube at %d,%d,%d\n", $x,$y,$z);
		printf("Open surface towards (%d,%d,%d)\n",$x,$y-1,$z);
		@dir = (0,-1,0);
		last;
	}
	$z++;
	if ($vox{"$x,$y,$z"}) {
		printf("Found cube at %d,%d,%d\n", $x,$y,$z);
		printf("Open surface towards (%d,%d,%d)\n",$x,$y,$z-1);
		@dir = (0,0,-1);
		last;
	}
}

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);

# This is the seed, look for neighbors on the four surrounding sides of the cube:
my %outer;
sub color
{
	my ($x,$y,$z,$dx,$dy,$dz) = @_;
	$outer{sprintf("%d,%d,%d,%d,%d,%d",$x,$y,$z,$dx,$dy,$dz)}++;
	printf("Colored at %d,%d,%d,%d,%d,%d\n",$x,$y,$z,$dx,$dy,$dz)
}

sub isblank
{
	my ($x,$y,$z,$dx,$dy,$dz) = @_;
	my $k = sprintf("%d,%d,%d,%d,%d,%d",$x,$y,$z,$dx,$dy,$dz);
	return (!cube($x+$dx,$y+$dy,$z+$dz) && !defined($outer{$k}));
}

my @seed = (); #($x,$y,$z,@dir);

my %tried;
sub try
{
	my ($x,$y,$z,$dx,$dy,$dz) = @_;
	return if (cube($x+$dx,$y+$dy,$z+$dz));
	return if ($tried{"$x,$y,$z,$dx,$dy,$dz"}++);
	return unless (isblank($x,$y,$z,$dx,$dy,$dz));
	push(@seed,$x,$y,$z,$dx,$dy,$dz);
}
	
while (scalar(@seed)) {
	my ($x,$y,$z,$dx,$dy,$dz) = splice(@seed,0,6);
	color($x,$y,$z,$dx,$dy,$dz);
	try($x,$y,$z,-1,0,0) unless ($dx);
	try($x,$y,$z,1,0,0) unless ($dx);
	try($x,$y,$z,0,-1,0) unless ($dy);
	try($x,$y,$z,0,+1,0) unless ($dy);
	try($x,$y,$z,0,0,-1) unless ($dz);
	try($x,$y,$z,0,0,+1) unless ($dz);
}

$part2 = scalar(keys %outer);
printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);
