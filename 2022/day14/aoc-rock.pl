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
no warnings 'recursion';

my $DEBUG = 0;

my $fname = shift;
$fname = 'input.txt' unless (defined($fname));
open(F ,'<',$fname);

my $start = time;

my $part1 = 0;
my $part2 = 0;


my @inp = ();
foreach (<F>) {
	chomp;
	push(@inp,$_);
}
close(F);
#printf(STDERR "%s\n\n", join("\n", @inp));

sub signum
{
	my ($n) = @_;
	return 1 if ($n > 0);
	return -1 if ($n < 0);
	return 0;
}

my %rock = ();
my $bottom = 0;
my $LEFT = 500;
my $RIGHT = 500;

my $VBOTTOM;

sub generate_map
{
	%rock = ();
	foreach (@inp) {
		my @points = split(/\-\>/);
		my ($x0,$y0) = split(/,/,$points[0]);
		$rock{sprintf("%d,%d",$x0,$y0)} = "#";
		for (my $p = 1; $p < scalar(@points); $p++) {
			my ($x, $y) = split(/,/,$points[$p]);
			my $dx = signum($x-$x0);
			my $dy = signum($y-$y0);
			while ($x0 != $x || $y0 != $y) {
				$x0+= $dx; $y0 += $dy;
				$rock{"$x0,$y0"} = "#";
			} 
		}
	}
	find_boundaries();
}

sub room
{
	my ($x,$y) = @_;
	my $key = sprintf("%d,%d", $x,$y);
	return !defined($rock{$key}) || ($rock{$key} le ' ');
}

sub room2
{
	my ($x,$y) = @_;
	return 0 if ($y >= $VBOTTOM);
	#return room($x,$y);
	my $key = sprintf("%d,%d", $x,$y);
	return !defined($rock{$key}) || ($rock{$key} le ' ');
}

sub find_boundaries
{
	foreach (keys %rock) {
		my ($x0,$y0) = split(/,/);
		$bottom = $y0 if ($y0 > $bottom);
		$LEFT = $x0 if ($x0 < $LEFT);
		$RIGHT = $x0 if ($x0 > $RIGHT);
	}
	#printf("Found %d filled spaces\n", scalar(keys %rock));
}

sub show
{
	find_boundaries();
	for (my $y = 0; $y <= $bottom; $y++) {
		printf("%3d ",$y);
		for (my $x = $LEFT; $x <= $RIGHT; $x++) {
			my $c = $rock{"$x,$y"};
			printf("%s", defined($c) ? $c : '.');
		}
		printf("\n");
	}
	printf("\n");
}

# Part1
generate_map();

# Fill with sand
my $sand = 0; # Number of sand-filled cells
my $land = 1; # Turns to zero when sand starts to drop off
while ($land) {
	my ($x,$y) = (500,0);   # Starting position
	$land = 0;				# Assume it goes bad
	while ($y <= $bottom) {
		if (room($x,$y+1)) {
			#$y++;
		}
		elsif (room($x-1,$y+1)) {
			$x--; #$y++;
		}
		elsif (room($x+1,$y+1)) {
			$x++; #$y++;
		}
		else {
			$rock{"$x,$y"} = 'o';
			$sand++;
			$land = 1;
			$y = $bottom+1;
			#show();
		}
		$y++;
	}
}
#show();

$part1 = $sand;

my $p1time = time - $start;

printf("Part1: %s\n", $part1);
printf(STDERR "Part1 time = %f\n", $p1time);


# Part2
generate_map();

$VBOTTOM = $bottom + 2; 
$LEFT = 500-$bottom-1 if (500-$bottom-1 < $LEFT);
$RIGHT = 500+$bottom+1 if (500+$bottom+1 > $RIGHT);
printf("Part2 range = (%d - %d) x %d\n", $LEFT, $RIGHT, $VBOTTOM-1);

# Create text array
my @g = ();
my $l = '.' x ($RIGHT-$LEFT+1);
for (my $y = 0; $y < $VBOTTOM; $y++) {
	push(@g, $l);
}

# Initialize rock map
foreach (keys %rock) {
	my ($x,$y) = split(/,/);
	substr($g[$y],$x-$LEFT,1) = '#';
}
#printf("%s\n\n", join("\n", @g));

for (my $y = 0; $y <= $bottom; $y++) {
	my $spos = 0;
	while (($spos = index($g[$y],'###',$spos)) >= 0) {
		$spos++;
		substr($g[$y+1],$spos,1) = '#';
		$rock{sprintf("%d,%d", $spos+$LEFT, $y+1)} = '#';
	}
}
#printf("%s\n\n", join("\n", @g));
printf("Total rock = %d\n", scalar(keys %rock));
$part2 = $VBOTTOM*$VBOTTOM - scalar (keys %rock);
printf("Triangle area: %d - rock: %d = sand: %d\n", $VBOTTOM*$VBOTTOM, scalar (keys %rock), $part2);

printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);

if (1) {
# Fill with sand
	$sand = 0;
	while (room2(500,0)) {
		my ($x,$y) = (500,0);
		while ($y < $VBOTTOM) {
			if (room2($x,$y+1)) {
				#$y++;
			}
			elsif (room2($x-1,$y+1)) {
				$x--;
			}
			elsif (room2($x+1,$y+1)) {
				$x++;
			}
			else {
				$rock{"$x,$y"} = 'o';
				$sand++;
				$y = $VBOTTOM;
				#show();
			}
			$y++;
		}
	}
	#show();
	$part2 = $sand;
}

printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);
