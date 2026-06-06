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
my @s = ();
my $TARGET = 2000000;
my $MAX = 4000000;
if ($fname eq 't.txt') {
	$TARGET = 10;
	$MAX = 20;
}

my %none = ();
my %beacons = ();
my %sensors = ();
foreach (<F>) {
	chomp;
	push(@inp,$_);
	die ("Bad input $_") unless 
		(/Sensor at x=(\-?\d+), y=(\-?\d+): closest beacon is at x=(\-?\d+), y=(\-?\d+)/);
	my ($x0,$y0,$x,$y) = ($1,$2,$3,$4);
	$beacons{"$x,$y"}++;
	my ($dx,$dy) = ($x-$x0, $y-$y0);
	my $manh = abs($dx)+abs($dy);
	$sensors{"$x0,$y0"} = $manh;
}

foreach (@inp) {
	die ("Bad input $_") unless 
		(/Sensor at x=(\-?\d+), y=(\-?\d+): closest beacon is at x=(\-?\d+), y=(\-?\d+)/);
	my ($x0,$y0,$x,$y) = ($1,$2,$3,$4);
	my ($dx,$dy) = ($x-$x0, $y-$y0);
#	printf(STDERR "%s\n", join(',',$x0,$y0,$dx,$dy));
	push(@s,\($x0,$y0,$dx,$dy));

	my ($sx, $sy) = signum($dx, $dy);
	my $manh = abs($dx)+abs($dy);
	if ($y0-$manh <= $TARGET && $y0+$manh >= $TARGET) {
		my $dty = abs($TARGET - $y0);
		my $manx = $manh - $dty;
		die("Negative manx!") if ($manx < 0);
		for (my $x = $x0-$manx; $x <= $x0+$manx; $x++) {
			$none{$x}++;
		}
#		printf(STDERR "Mark %d to %d (sum = %d)\n", $x0-$manx, $x0+$manx, scalar(keys %none));
	}
}
close(F);
$part1 = keys(%none);
foreach (keys %beacons) {
	my ($x,$y) = split(/,/);
	if ($y == $TARGET && defined($none{$x})) {
		$part1--;
	}
}

sub search
{
	my ($MAX, @blocks) = @_;
	while (1) {
		my $step = int($MAX / 10);
		$step = 1 if ($step <= 0);

		printf(STDERR "Searching %d blocks of size %d using step %d\n", scalar(@blocks), $MAX, $step);
		my @possibles = (); # Save candidates for next round
		foreach (@blocks) {
			my ($x0,$y0) = split(/,/);
			for (my $x = $x0; $x < $x0+$MAX; $x+= $step) {
				for (my $y = $y0; $y < $y0+$MAX; $y+= $step) {
					my $possible = 1;
					foreach (keys %sensors) {
						my ($sx,$sy) = split(/,/);
						my $manh = $sensors{$_};
						if (abs($sx-$x)+abs($sy-$y) <= $manh) {
							if (abs($sx-($x+$step-1))+abs($sy-$y) <= $manh) {
								if (abs($sx-$x)+abs($sy-($y+$step-1)) <= $manh) {
									if (abs($sx-($x+$step-1))+abs($sy-($y+$step-1)) <= $manh) {
										$possible = 0; # Totally covered by this sensor
										last;
									}
								}
							}
						}
					}
					if ($possible) {
						if ($step == 1) {
							$part2 = $x * 4000000 + $y;
							printf(STDERR "Found (%d,%d)\n", $x,$y);
							return;
						}
						else {
							push(@possibles,"$x,$y");
						}
					}
				}
			}
		}
		die("No solution!") unless (scalar(@possibles));
		$MAX = $step;
		@blocks = @possibles;
	}
}

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);

my $m4 = $MAX>>1;
my @pids;
for (my $x = 0; $x < $MAX; $x += $m4) {
	for (my $y = 0; $y < $MAX; $y += $m4) {
		if (my $pid = fork()) {
			printf(STDERR "Started thread!\n");
			push(@pids, $pid);
		}
		else {
			search($m4,"$x,$y");
			die;
		}
	}
}

foreach (@pids) {
	wait();
}
#search($MAX,"0,0");

sub signum
{
	my (@inp) = @_;
	foreach (@inp) {
		if ($_ < 0) { $_ = -1; }
		elsif ($_ > 0) { $_ = 1; }
	}
	return @inp;
}

printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);
