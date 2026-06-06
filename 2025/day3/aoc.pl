#!perl -w
use strict;
use Time::HiRes qw(time);

# Used 0.007294s (linear)

my $best = 1e38;
my $part1 = 0;
my $part2 = 0;

my @lines = ();
while (<>) {
	chomp;
	push(@lines,$_);
}

sub recursive_jolt
{
	my ($first, $digits, $n) = @_;
	$digits--;
	my $max = 0;
	my $pos;
	for (my $p = $first; $p + $digits < scalar(@{$n}); $p++) {
		if ($n->[$p] gt $max) {
			$max = $n->[$p];
			$pos = $p;
		}
	}
	if ($digits) {
		$max .= recursive_jolt($pos+1,$digits, $n);
	}
	return $max;
}

sub linear_jolt
{
	my ($first, $digits, $n) = @_;
	my $res = '';
	my $pos;
	do {
		$digits--;
		my $max = 0;
		for (my $p = $first; $p + $digits < scalar(@{$n}); $p++) {
			if ($n->[$p] gt $max) {
				$max = $n->[$p];
				$pos = $p;
			}
		}
		$res .= $max;
		$first = $pos+1;
	} while ($digits);
	return $res;
}

for (my $iter = 0; $iter < 100; $iter++) {
	my $t0 = time;

	$part1 = 0;
	$part2 = 0;

	foreach (@lines) {
		my @n = split(//,$_);
		$part1 += linear_jolt(0,2,\@n);
		$part2 += linear_jolt(0,12,\@n);
	}

	my $used = time-$t0;
	if ($used < $best) {
		$best = $used;
	}
}
my $used = $best;

printf("Test: 357 3121910778619\n");
printf("Input: 17074 169512729575727\n");
printf("%s\n%s\n", $part1, $part2);
printf("Linear\n");
printf("Used %1.6fs\n", $used);

$best = 1e38;

for (my $iter = 0; $iter < 100; $iter++) {
	my $t0 = time;

	$part1 = 0;
	$part2 = 0;

	foreach (@lines) {
		my @n = split(//,$_);
		$part1 += recursive_jolt(0,2,\@n);
		$part2 += recursive_jolt(0,12,\@n);
	}

	my $used = time-$t0;
	if ($used < $best) {
		$best = $used;
	}
}
$used = $best;

printf("%s\n%s\n", $part1, $part2);
printf("Recursive\n");
printf("Used %1.6fs\n", $used);

