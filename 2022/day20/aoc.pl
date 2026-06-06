#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);
#use List::Util qw (reduce);
use List::PriorityQueue;
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

foreach (<F>) {
	chomp;
	push(@inp,$_);
}
close(F);

my $p = 0;
my @num;
foreach (@inp) {
	$p++;
	push(@num,"$_,$p");
}
#printf("%s\n", join(" ", @num));

my $P1 = $p-1;

for (my $i = 1; $i <= $p; $i++) {
	# Find node:
	my ($n,$orgp, $j);
	for ($j = 0; $j < $p; $j++) {
		($n,$orgp) = split(/,/, $num[$j]);
		if (defined($orgp) && ($orgp == $i)) {
			last;
		}
	}
	die ("Node $i not found!") if ($j >= $p);
	
	# Extract it:
	splice(@num,$j,1);
	$j = $j % $P1;
	# Move this node $n positions +/-
	my $an = ($j+$n+$P1*abs($n)) % $P1;
#	printf("Moving %s from $j to $an\n", "$n,$orgp");
	splice(@num,$an,0,sprintf("%d",$n));
#	printf("%s\n", join(" ", @num));
}
# Find zero
for (my $j = 0; $j < $p; $j++) {
	if ($num[$j] == 0) {
		$part1 = $num[($j+1000) % $p] + $num[($j+2000) % $p] + $num[($j+3000) % $p];
		last;
	}
}

printf("Part1: %s\n", $part1);
printf(STDERR "Total time = %f\n", time - $start);

@num =();
$p = 0;
foreach (@inp) {
	$p++;
	push(@num,sprintf("%d,%d",$_ * 811589153,$p));
}

for (my $round = 1; $round <= 10; $round++) {
	for (my $i = 1; $i <= $p; $i++) {
		# Find node:
		my ($n,$orgp, $j);
		for ($j = 0; $j < $p; $j++) {
			($n,$orgp) = split(/,/, $num[$j]);
			if (defined($orgp) && ($orgp == $i)) {
				last;
			}
		}
		die ("Node $i not found!") if ($j >= $p);
		
		# Extract it:
		splice(@num,$j,1);
		$j = $j % $P1;
		# Move this node $n positions +/-
		my $an = ($j+$n+$P1*abs($n)) % $P1;
	#	printf("Moving %s from $j to $an\n", "$n,$orgp");
		splice(@num,$an,0,sprintf("%d,%d",$n,$orgp));
	#	printf("%s\n", join(" ", @num));
	}
	#printf("$round: %s\n", join(" ", @num));
}
# Find zero
for (my $j = 0; $j < $p; $j++) {
	my ($v, $pos) = split(/,/,$num[$j]);
	if ($v == 0) {
		$part2 = (split(/,/,$num[($j+1000) % $p]))[0] + (split(/,/,$num[($j+2000) % $p]))[0] + (split(/,/,$num[($j+3000) % $p]))[0];
		last;
	}
}


printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);
