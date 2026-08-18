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

my @hlines = ();

foreach (@inp) {
	my @points = split(/\-\>/);
	my ($x0,$y0) = split(/,/,$points[0]);
	for (my $p = 1; $p < scalar(@points); $p++) {
		my ($x, $y) = split(/,/,$points[$p]);
		my $dx = signum($x-$x0);
		my $dy = signum($y-$y0);
		if ($dx == 0) { # vertical line
			push(@hlines,sprintf("%5d,%5d,%5d", $y0, $x, $x+1));
			while ($y0 != $y) {
				$y0 += $dy;
				push(@hlines,sprintf("%5d,%5d,%5d", $y0, $x, $x+1));
			}
		}
		else { # horizontal
			if ($x0 <= $x) {
				push(@hlines,sprintf("%5d,%5d,%5d", $y, $x0, $x+1));
			}
			else {
				push(@hlines,sprintf("%5d,%5d,%5d", $y, $x, $x0+1));
			}
		}
	}
}

@hlines = sort @hlines;
$VBOTTOM = (split(/,/,$hlines[scalar(@hlines)-1]))[0] + 2;

my @sand = (500,501);
my $sand = 1;
my $start = 500; my $slutt = 500;
my @holes = ();
my $h = 0;
my ($ly,$lx0,$lx1) = split(/,/,$holes[0]);
for (my $y = 1; $y < $VBOTTOM; $y++) {
	$start--; $slutt++;
	$sand += ($slutt-$start)+1;
	while ($ly < $y) {
		$h++;
		($ly,$lx0,$lx1) = split(/,/,$holes[$h]);
	}
	while (my $b,$e = splice(@holes,0,2)) {
		$b++; $e--;
		if ($e >= $b) {
			push(@newholes, $b, $e);
		}
	}
	while ($ly ==$y) {
		push(@newholes, $lx0, $lx1-1);
		$h++;
		($ly,$lx0,$lx1) = split(/,/,$holes[$h]);
	}
	
	

$part2 = $sand;

printf("Part1: %s\n", $part1);
printf(STDERR "Part1 time = %f\n", $p1time);
printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);
