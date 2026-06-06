#!perl -w

use strict;
use Time::HiRes qw (time);
use Math::BigInt;

my $start = time;

my @inp = (<>);
chomp(@inp);

my $a10 = 0;
my @b10 = ();
my %pa = (')'=>3, ']'=>57, '}'=>1197, '>'=>25137);

foreach (@inp) {
	while (s/\(\)|\[\]|\{\}|\<\>//g) {};
	
	my $p = 0;
	foreach (split(//, $_)) {
		if (defined($pa{$_})) { $p = $pa{$_}; last; }
	}
	$a10 += $p;
	if ($p == 0) { # Incomplete string, fix these for part b
		my $base5 = scalar reverse $_;
		$base5 =~ tr/\(\[\{\</1234/;
		my $sum = Math::BigInt->from_base($base5, 5);
		push(@b10, $sum);
	}
}
@b10 = sort {$a <=> $b} @b10;
my $b10 = $b10[scalar(@b10)>>1];

printf(STDERR "Total time = %f\n", time - $start);
printf("aoc10a: %s\n", $a10);
printf("aoc10b: %s\n", $b10);
