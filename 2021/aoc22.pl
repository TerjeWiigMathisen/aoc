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

my $DEBUG = 0;
my $part1;
my $part2;

my %init = ();
my @fr = ();
my @rr = ();

foreach (@inp) {
	my @or = split(/ /);
	my $on = (substr($or[0],0,2) eq 'on') ? 1 : 0;
	my @ranges = split(/,/, $or[1]);
	my @r = ();
	my @f = ();
	my @p1 = ();
	foreach (@ranges) {
		my ($xyz, $r) = split(/=/);
		my ($b,$e) = split(/\.\./,$r); $e++;
		push(@r,join(",",$b,$e));
		
		if ($b < -50) { $b = -50; }
		if ($e > 50) {$e = 51; }
		push(@f, join(",",$b,$e)); # if ($e > $b);
	}
	push(@rr,join(",",$on,@f)) if (scalar(@f));
	push(@fr,join(",",$on,@r));

my $q = q(
	my ($x0,$x1) = split(/,/,$ranges[0]);
	my ($y0,$y1) = split(/,/,$ranges[1]);
	my ($z0,$z1) = split(/,/,$ranges[2]);
	
	for (my $z = $z0; $z <= $z1; $z++) {
		for (my $y = $y0; $y <= $y1; $y++) {
			for (my $x = $x0; $x <= $x1; $x++) {
				$init{"$x,$y,$z"} = $on;
			}
		}
	}
	);
}

my $partcnt = 0;
my $volcnt = 0;

sub partition
{
	my @inp = @_;
	$partcnt++;
	return 0 if (scalar(@inp) < 1);
	my %blocks = ();
	
	my (%x, %y, %z);
	my $idx = 0;
	my $skip = 0;
	foreach (@inp) {
		my ($on, $x0,$x1, $y0,$y1, $z0,$z1) = split(/,/);
		my $key = join(",",$x0,$x1, $y0,$y1, $z0,$z1);
		$blocks{$key} = $on ? $idx : -(1+$idx);
		$x{$x0}++;
		$y{$y0}++;
		$z{$z0}++;
		$x{$x1}++;
		$y{$y1}++;
		$z{$z1}++;
		$idx++;
	}
	if (scalar (keys %blocks) == 1) { # All overlaps
		my ($on, $x0,$x1, $y0,$y1, $z0,$z1) = split(/,/, $inp[-1]);
		$volcnt++;
		return $on*($x1-$x0)*($y1-$y0)*($z1-$z0);
	}
				
	if (scalar(@inp) == 2) {
		printf(STDERR "%s & %s\n", $inp[0], $inp[1]) if ($DEBUG);
	}
	
	my @x = sort {$a <=> $b} (keys %x);
	my @y = sort {$a <=> $b} (keys %y);
	my @z = sort {$a <=> $b} (keys %z);
	printf(STDERR "Partition %d blocks (%d,%d,%d) - (%d,%d,%d)", scalar(@inp), $x[0],$y[0],$z[0],$x[-1],$y[-1],$z[-1]) if ($DEBUG);

	# pick the axis with the most unique values:
	my $vol = 0;
	my ($dx, $dy, $dz) = ($x[-1]-$x[0], $y[-1]-$y[0], $z[-1],$z[0]);
	if (scalar(@x) >= scalar(@y) && scalar(@x) >= scalar(@z)) {
		my $spax = $x[scalar(@x)>>1];
		printf(STDERR " -> X = %d\n", $spax) if ($DEBUG);
		my (@left, @right);
		foreach (@inp) {
			my ($on, $x0,$x1, $y0,$y1, $z0,$z1) = split(/,/);
			next unless ($x1 > $x0 && $y1 > $y0 && $z1 > $z0);

			if ($x1 <= $spax) {
				push(@left,$_);
			}
			elsif ($x0 >= $spax) {
				push(@right,$_);
			}
			else { # split!
				push(@left,join(",",$on,$x0,$spax,$y0,$y1,$z0,$z1));
				push(@right,join(",",$on,$spax,$x1,$y0,$y1,$z0,$z1));
			}
		}
		$vol = partition(@left) + partition(@right);
	}
	elsif (scalar(@y) >= scalar(@x) && scalar(@y) >= scalar(@z)) {
		my $spay = $y[scalar(@y)>>1];
		printf(STDERR " -> Y = %d\n", $spay) if ($DEBUG);
		my (@up, @down);
		foreach (@inp) {
			my ($on, $x0,$x1, $y0,$y1, $z0,$z1) = split(/,/);
			next unless ($x1 > $x0 && $y1 > $y0 && $z1 > $z0);

			if ($y1 <= $spay) {
				push(@down,$_);
			}
			elsif ($y0 >= $spay) {
				push(@up,$_);
			}
			else { # split!
				push(@down,join(",",$on,$x0,$x1,$y0,$spay,$z0,$z1));
				push(@up,join(",",$on,$x0,$x1,$spay,$y1,$z0,$z1));
			}
		}
		$vol = partition(@down) + partition(@up);
	}
	else {
		my $spaz = $z[scalar(@z)>>1];
		printf(STDERR " -> Z = %d\n", $spaz) if ($DEBUG);
		my (@above, @below);
		foreach (@inp) {
			my ($on, $x0,$x1, $y0,$y1, $z0,$z1) = split(/,/);
			next unless ($x1 > $x0 && $y1 > $y0 && $z1 > $z0);

			if ($z1 <= $spaz) {
				push(@below,$_);
			}
			elsif ($z0 >= $spaz) {
				push(@above,$_);
			}
			else { # split!
				push(@below,join(",",$on,$x0,$x1,$y0,$y1,$z0,$spaz));
				push(@above,join(",",$on,$x0,$x1,$y0,$y1,$spaz,$z1));
			}
		}
		$vol = partition(@above) + partition(@below);
	}
	return $vol;
}

$part1 = partition(@rr);
$part2 = partition(@fr);

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %d\n", $part1);
printf("Part2: %1.0f\n", $part2);

printf(STDERR "Partition calls: %d, volumes measured: %d\n", $partcnt, $volcnt);