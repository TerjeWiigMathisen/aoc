#!perl -w

use strict;
use Time::HiRes qw (time);
use Math::BigInt;

my $start = time;

my @inp = (<>);
chomp(@inp);

my $R = scalar(@inp);
my $C = length($inp[0]);

my @nine = ();

sub inc
{
	my ($x, $y) = @_;
	if ($x >= 0 && $x < $C && $y >= 0 && $y < $R) {
		my $c = substr($inp[$y],$x,1);
		if ($c eq '9') {
			push(@nine, "$x;$y");
			$c = 'A';
		}
		else {
			$c++;
		}
		substr($inp[$y],$x,1) = $c;
	}
}

sub step
{
	@nine = ();
	for (my $y = 0; $y < $R; $y++) {
		for (my $x = 0; $x < $C; $x++) {
			inc($x, $y);
		}
	}
	for (my $f = 0; $f < scalar(@nine); $f++) {
		my $top = $nine[$f];
		my ($sx, $sy) = split(/;/, $top);
		foreach ("-1;-1","0;-1","1;-1","-1;0","1;0","-1;1","0;1","1;1") {
			my ($x,$y) = split(/;/);
			inc($x + $sx, $y + $sy);
		}
	}
	foreach (@nine) {
		my ($x, $y) = split(/;/);
		substr($inp[$y],$x,1) = '0';
	}

	return scalar(@nine);
}

sub dmp
{
	my ($gen) = @_;
	printf(STDERR "%d\n", $gen);
	foreach (@inp) {
		printf(STDERR "%s\n", $_);
	}
	printf(STDERR "\n");
}

my $a11 = 0;
my $b11;


dmp(0);
for (my $s = 1; 1; $s++) {
	my $flashes = step();
	$a11 += $flashes if ($s <= 100);
	if ($flashes == 100) {
		$b11 = $s;
		last;
	}
}
dmp($b11);

printf(STDERR "Total time = %f\n", time - $start);
printf("aoc11a: %s\n", $a11);
printf("aoc11b: %s\n", $b11);
