#!perl -w

use strict;
use Time::HiRes qw (time);

my $start = time;

my @inp = (<>);
chomp(@inp);

my $DEBUG = 0;

my $rows = scalar(@inp);
my $cols = length($inp[0]);

my $risk = 0;

my %basins = ();

for (my $y = 0; $y < $rows; $y++) {
	for (my $x = 0; $x < $cols; $x++) {
		my $h = substr($inp[$y], $x, 1);
		next if ($x > 0 && substr($inp[$y], $x-1, 1) <= $h);
		next if ($x < $cols-1 && substr($inp[$y], $x+1, 1) <= $h);
		next if ($y > 0 && substr($inp[$y-1], $x, 1) <= $h);
		next if ($y < $rows-1 && substr($inp[$y+1], $x, 1) <= $h);
		printf(STDERR "low at $x,$y = $h\n") if ($DEBUG);
		$risk += $h+1;
		$basins{"$x;$y"} = $h;
	}
}

my %members;
my @members = ();

foreach (keys %basins) {
	my ($x, $y) = split(/;/);
	my $h = $basins{$_};

	%members = ();
	
	fill4(-1,$x,$y);
	push(@members, scalar(keys %members));
	printf(STDERR "basin %s size = %d\n", $_, scalar(keys %members)) if ($DEBUG);
}

@members = sort {$b <=> $a} @members; # Reverse numeric sort

my $prod = $members[0]*$members[1]* $members[2];

printf(STDERR "Total time = %f\n", time - $start);

printf("aoc9a: %s\n", $risk);

printf("aoc9b: %s\n", $prod);

sub fill4
{
	my ($h, $x, $y) = @_;
	return if (defined($members{"$x,$y"}));

	return if (($x < 0) || ($x >= $cols));
	return if (($y < 0) || ($y >= $rows));
	
	my $curr = substr($inp[$y], $x, 1);

	return if ($curr <= $h);
	return if ($curr >= 9);

	$members{"$x,$y"}++;
	$h = $curr;
	fill4($h, $x-1,$y);
	fill4($h, $x+1,$y);
	fill4($h, $x,$y-1);
	fill4($h, $x, $y+1);
}