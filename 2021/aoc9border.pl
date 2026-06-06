#!perl -w

use strict;
use Time::HiRes qw (time);

my $start = time;

my @inp = (<>);
chomp(@inp);

my $rows = scalar(@inp);
my $cols = length($inp[0]);

my @i = ();
my $nines = join("", ('9') x (length($inp[0])+2));
push(@i, $nines);
foreach (@inp) {
	push(@i, '9'.$_.'9');
}
push(@i, $nines);

@inp = @i;

my $DEBUG = 0;

my $risk = 0;

my %basins = ();

for (my $y = 1; $y <= $rows; $y++) {
	for (my $x = 1; $x <= $cols; $x++) {
		my $h = substr($inp[$y], $x, 1);
		next if (substr($inp[$y], $x-1, 1) <= $h);
		next if (substr($inp[$y], $x+1, 1) <= $h);
		next if (substr($inp[$y-1], $x, 1) <= $h);
		next if (substr($inp[$y+1], $x, 1) <= $h);
		printf(STDERR "low at $x,$y = $h\n") if ($DEBUG);
		$risk += $h+1;
		$basins{"$x;$y"} = $h;
	}
}

my %found;
my @members = ();

foreach (keys %basins) {
	my ($x, $y) = split(/;/);
	my $h = $basins{$_};

	%found = ();
	
	fill4(-1,$x,$y);
	push(@members, scalar(keys %found));
	printf(STDERR "basin %s size = %d\n", $_, scalar(keys %found)) if ($DEBUG);
}

@members = sort {$b <=> $a} @members; # Reverse numeric sort

my $prod = $members[0]*$members[1]* $members[2];

printf(STDERR "Total time = %f\n", time - $start);

printf("aoc9a: %s\n", $risk);

printf("aoc9b: %s\n", $prod);

sub fill4
{
	my ($h, $x, $y) = @_;
	return if (defined($found{"$x,$y"}));

	my $curr = substr($inp[$y], $x, 1);

	return if ($curr <= $h);
	return if ($curr >= 9);

	$found{"$x,$y"}++;
	$h = $curr;
	fill4($h, $x-1,$y);
	fill4($h, $x+1,$y);
	fill4($h, $x,$y-1);
	fill4($h, $x, $y+1);
}