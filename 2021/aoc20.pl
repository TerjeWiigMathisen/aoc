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

my $rule = shift (@inp);

#$rule =~ tr/ #/01/;
my @rule = split(//,$rule);
shift @inp;

my %board = ();
for (my $y = 0; $y < scalar(@inp); $y++) {
#	$inp[$y] =~tr/ #/01/;
	my $x = 0;
	foreach (split(//,$inp[$y])) {
		$board{"$x,$y"}++ if ($_ eq '#');
		$x++;
	}
}
my $border = ' ';

sub cell
{
	my ($x, $y) = @_;
	my $key = "$x,$y";
	my $r = '0';
	$r = '1' if (defined($board{$key}));
	return $r;
}

sub gen
{
	my %b = ();
	# Find active extent
	my ($x0,$x1,$y0,$y1) = (1e38,-1e38,1e38,-1e38);
	foreach (keys %board) {
		my $key = $_;
		my ($x,$y) = split(/,/, $key);
		$x0 = $x if ($x < $x0);
		$x1 = $x if ($x > $x1);
		$y0 = $y if ($y < $y0);
		$y1 = $y if ($y > $y1);
	}
	# Processing needs to handle the infinite surrounding blinkers
	if ($border eq ' ') { # All black around the image, extend it by a 3-pixel frame (which will be all white)
		for (my $y = $y0-3; $y <= $y1+3; $y++) {
			for (my $x = $x0-3; $x <= $x1+3; $x++) {
			
				my $sample = 0;
				if ($x == 2 && $y == 2) {
					$sample += 0;
				}
				for (my $cy = $y-1; $cy <= $y+1; $cy++) {
					for (my $cx = $x-1; $cx <= $x+1; $cx++) {
						$sample = $sample*2+cell($cx,$cy);
					}
				}
				my $out = $rule[$sample];
				$b{"$x,$y"}++ if ($out eq "#");
			}
		}
		$border = '#';
	}
	else { # We have an infinite white field, only do the internal part
		for (my $y = $y0+1; $y <= $y1-1; $y++) {
			for (my $x = $x0+1; $x <= $x1-1; $x++) {
				my $sample = 0;
				if ($x == 2 && $y == 2) {
					$sample += 0;
				}
				for (my $cy = $y-1; $cy <= $y+1; $cy++) {
					for (my $cx = $x-1; $cx <= $x+1; $cx++) {
						$sample = $sample*2+cell($cx,$cy);
					}
				}
				my $out = $rule[$sample];
				$b{"$x,$y"}++ if ($out eq "#");
			}
		}
		$border = ' ';
	}
	return %b;
}

sub dmp
{
	my ($x0,$x1,$y0,$y1) = (1e38,-1e38,1e38,-1e38);
	foreach (keys %board) {
		my $key = $_;
		my ($x,$y) = split(/,/, $key);
		$x0 = $x if ($x < $x0);
		$x1 = $x if ($x > $x1);
		$y0 = $y if ($y < $y0);
		$y1 = $y if ($y > $y1);
	}
	for (my $y = $y0; $y <= $y1; $y++) {
		my $line = "";
		for (my $x = $x0; $x <= $x1; $x++) {
			$line .= cell($x,$y) ? '#' : ' ';
		}
		printf(STDERR "%s\n",$line);
	}
	printf(STDERR "\n");
}

dmp();

for (my $iter = 0; $iter < 2; $iter++) {
	%board = gen();
}

dmp();

my $part1 = scalar(keys %board);

for (my $iter = 2; $iter < 50; $iter++) {
	%board = gen();
}
dmp();


my $part2 = scalar(keys %board);

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %d\n", $part1);
printf("Part2: %d\n", $part2);
