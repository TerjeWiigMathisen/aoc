#!perl -w

use strict;
use Time::HiRes qw (time);
use Math::BigInt;

my $start = time;

my @inp = (<>);
chomp(@inp);

my @fold = ();
my @fx = ();
my @fy = ();
my $hash;
my %fxy = ();

my @sheet = ();

my ($maxx, $maxy) = (0,0);

foreach (@inp) {
	if (/^fold along ([xy])=(\d+)$/) {
		push(@fold,"$1,$2");
	}
	elsif (/^(\d+),(\d+)$/) {
		$hash->{$_}++;
		my ($x, $y) = split(/,/);
		$maxx = $x if ($x > $maxx);
		$maxy = $y if ($y > $maxy);
	}
}

sub fold
{
	my ($hash, @fold) = @_;
	my @xf = ();
	my @yf = ();
	my @fx = ();
	my @fy = ();

	foreach (@fold) {
		my ($dir, $pos) = split(/,/);
		if ($dir eq 'x') {
			push(@fx, $pos);
		}
		else {
			push(@fy, $pos);
		}
	}
	#@fx = reverse @fx; @fy = reverse @fy;
	for (my $i = 0; $i < $maxy; $i++) {
		my $j = $i;
		foreach (@fy) { # In descending order!
			if ($j > $_) {
				$j = $_+$_-$j;
				if ($j < 0) {
					$j = undef;
					last;
				}
			}
		}
		$yf[$i] = $j;
	}
	for (my $i = 0; $i < $maxx; $i++) {
		my $j = $i;
		foreach (@fx) {
			if ($j > $_) {
				$j = $_+$_-$j;
				if ($j < 0) {
					$j = undef;
					last;
				}
			}
		}
		$xf[$i] = $j;
	}
	$maxy = $fy[-1]; $maxx = $fx[-1];

	my $h;
	foreach (keys %{$hash}) {
		my ($x,$y) = split(/,/);
		$x = $xf[$x]; $y = $yf[$y];
		$h->{"$x,$y"}++ if (defined($x) && defined($y));
	}
	return $h;
}

dmp($hash);

$hash = fold($hash, $fold[0]);

my $a13 = scalar(keys %{$hash});

for (my $f = 1; $f < scalar(@fold); $f++) {
	$hash = fold($hash, $fold[$f]);
}

#$hash = fold($hash, (@fold)[1..(scalar(@fold)-1)]);

printf(STDERR "Total time = %f\n", time - $start);
printf("aoc13a: %d\n", $a13);
dmp($hash);

sub dmp
{
	my ($hash) = @_;
	for (my $y = 0; $y <= $maxy; $y++) {
		for (my $x = 0; $x <= $maxx; $x++) {
			printf(STDERR "%s", defined($hash->{"$x,$y"}) ? '#' : '.');
		}
		printf(STDERR "\n");
	}
	printf(STDERR "\n");
}

