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
my %hash = ();
my %fxy = ();

my @sheet = ();

my ($maxx, $maxy) = (0,0);

foreach (@inp) {
	if (/^fold along ([xy])=(\d+)$/) {
		push(@fold,$_);
		$fxy{$1}++;
		eval("push(\@f$1, $2);");
	}
	elsif (/^(\d+),(\d+)$/) {
		$hash{$_}++;
		my ($x, $y) = split(/,/);
		$maxx = $x if ($x > $maxx);
		$maxy = $y if ($y > $maxy);
	}
}

push(@sheet, [(0) x ($maxx+1)]) for (0..$maxy);

foreach (keys %hash) {
	my ($x, $y) = split(/,/);
	my $mx = $maxx;
	for (my $i = 0; $i < $fxy{'x'}; $i++) {
		$mx >>= 1;
		if ($x > $mx) { $x = $mx - $x; }
	}
	my $my = $maxy;
	for (my $i = 0; $i < $fxy{'y'}; $i++) {
		$my >>= 1;
		if ($y > $my) { $y = $my - $y; }
	}
	
	$sheet[$y][$x] = 1;
}
for (my $i = 0; $i < $fxy{'x'}; $i++) {
	$maxx >>= 1;
}
for (my $i = 0; $i < $fxy{'y'}; $i++) {
	$maxy >>= 1;
}


#dmp(@sheet);
foreach (keys %hash) {
	my ($x, $y) = split(/,/);
	$sheet[$y][$x] = 1;
}
#dmp(@sheet);

my ($a13, $b13);

foreach (@fold) {
	if (/^fold along x=(\d+)$/) {
		my $vis = 0;
		for (my ($xf, $xt) = ($1+1, $1-1); $xf <= $maxx && $xt >= 0; $xf++, $xt--) {
			for (my $i = 0; $i <= $maxy; $i++) {
				$sheet[$i][$xt] |= $sheet[$i][$xf];
				$sheet[$i][$xf] = 0;
				$vis += $sheet[$i][$xt];
			}
		}
		$a13 = $vis unless(defined($a13));
		$maxx = $1;
	}
	elsif (/^fold along y=(\d+)$/) {
		my $vis = 0;
		for (my ($yf, $yt) = ($1+1, $1-1); $yf <= $maxy && $yt >= 0; $yf++, $yt--) {
			for (my $i = 0; $i <= $maxx; $i++) {
				$sheet[$yt][$i] |= $sheet[$yf][$i];
				$sheet[$yf][$i] = 0;
				$vis += $sheet[$yt][$i];
			}
		}
		$a13 = $vis unless(defined($a13));
		$maxy = $1;
	}
}

printf(STDERR "Total time = %f\n", time - $start);
printf("aoc13a: %d\n", $a13);
dmp(@sheet);

#printf("aoc13b: %d\n", $b13 );

sub dmp
{
	for (my $y = 0; $y <= $maxy; $y++) {
		for (my $x = 0; $x <= $maxx; $x++) {
			printf(STDERR "%s", $sheet[$y][$x] ? "#" : ".");
		}
		printf(STDERR "\n");
	}
	printf(STDERR "\n");
}

my @sh = ();
push(@sh, [(0) x ($maxx+1)]) for (0..$maxy);

foreach (keys %hash) {
	my ($x, $y) = split(/,/);
	foreach (@fx) { $x = ($x > $_) ? $_+$_-$x : $x; }
	foreach (@fy) { $y = ($y > $_) ? $_+$_-$y : $y; }

	$sh[$y][$x] = 1;
}

dmp(@sh);
