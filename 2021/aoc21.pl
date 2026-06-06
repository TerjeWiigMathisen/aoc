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

my @pos = ();
my @sum = (0,0);
foreach (@inp) {
	if (/Player (\d+) starting position: (\d+)$/) {
		push(@pos,$2);
	}
}

my $part1;

my $die = 1;
my $rolls = 0;
my @op = @pos;

while (1) {
	for (my $p = 0; $p <= 1; $p++) {
		for (my $d = 0; $d < 3; $d++) {
			$pos[$p] = ($pos[$p] + $die - 1) % 10 + 1;
			$die = ($die % 100) + 1;
			$rolls++;
		}
		$sum[$p] += $pos[$p];
		if ($sum[$p] >= 1000) {
			$part1 = $sum[1-$p]*$rolls;
			goto done1;
		}
	}
}
done1:

@sum = (0,0);
my @steps = (0,0,0,1,3,6,7,6,3,1); # 27 permutations, sum in (3..9)

my @p1 = ((0) x 11);
my @p2 = ((0) x 11);
$p1[$op[0]] = 1;
$p2[$op[1]] = 1;

my @sum1 = ((0) x 11);
my @sum2 = ((0) x 11);
my $WIN = 21;

sub step
{
	my ($cnt, $sum) = @_;
	my @nc = @{$cnt};
	@{$cnt} = ((0) x 11);
	my @sum = @{$sum};
	@{$sum} = ((0) x 11);
	
	for (my $p = 1; $p <= 10; $p++) {
		my $prev = $nc[$p];
		next unless ($prev);
		for (my $s = 3; $s <= 9; $s++) {
			my $sc = $steps[$s];
			my $npos = ($p+$s-1) % 10 + 1;
			$cnt->[$npos] += $prev * $sc;
			$sum->[$npos] += $npos;
		}
	}
}

my %cache = ();
my $hits = 0;

sub step1
{
	my ($lvl, $pos1, $rest1, $pos2, $rest2) = @_;
	my $key = join(",",$pos1, $rest1, $pos2, $rest2);
	if (defined($cache{$key})) { $hits++; return split(/,/,$cache{$key}); }

	my ($wsum, $lsum) = (0,0);
	for (my $s = 3; $s <= 9; $s++) {
		my $sc = $steps[$s];
		my $npos = ($pos1+$s-1) % 10 + 1;
		my $r1 = $rest1-$npos;
		if ($r1 <= 0) {
			$wsum += $sc;
		}
		else {
			my ($l,$w) = step1($lvl+1,$pos2,$rest2,$npos,$r1);
			$wsum += $w*$sc;
			$lsum += $l*$sc;
		}
	}
	$cache{$key} = "$wsum,$lsum";
	return ($wsum,$lsum);
}

my ($w,$l) = step1(1, $op[0],21,$op[1],21);

sub dmp
{
	printf("Steps=%s\n", join("\t",@_));
	foreach (@p1) { printf(STDERR " %8.0f",$_); } printf(STDERR "\n");
	foreach (@sum1) { printf(STDERR " %8.0f",$_); } printf(STDERR "\n");
	foreach (@p2) { printf(STDERR " %8.0f",$_); } printf(STDERR "\n");
	foreach (@sum2) { printf(STDERR " %8.0f",$_); } printf(STDERR "\n");
	printf(STDERR "\n");
}

my $part2 = $w > $l ? $w : $l;

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %d\n", $part1);
printf("Part2: %1.0f\n", $part2);

printf(STDERR "Total cache keys: %d, hits: %d, wl: %s,%s\n",scalar(keys %cache), $hits, $w, $l);
