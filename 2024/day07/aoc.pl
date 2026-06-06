#!perl -w
# Best runtime 17.761ms

use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

while (<>) {
	chomp;
	my ($target, $tail) = split(/: /);
	my @nums = split(/\s+/,$tail);
	if (try_opsr($target, @nums)) {
		$part1 += $target;
		$part2 += $target;
	}
	elsif (try_opsr3($target, @nums)) {
		#printf("$_\n");
		$part2 += $target;
	}
}

sub try_opsr
{
	my ($target, @nums) = @_;
	if (scalar(@nums) == 0) { return 0; }
	my $n = pop(@nums);
	if (scalar(@nums) == 0) {
		return ($n == $target);
	}
	if ($target % $n == 0) {
		my $rem = $target / $n;
		if (try_opsr($rem,@nums)) { return 1; }
	}
	my $diff = $target - $n;
	return 0 if ($diff <= 0);
	return try_opsr($diff,@nums);
}

sub try_opsr3
{
	my ($target, @nums) = @_;
	if (scalar(@nums) == 0) { return 0; }
	my $n = pop(@nums);
	if (scalar(@nums) == 0) {
		return ($n == $target);
	}
	if ($target % $n == 0) {
		my $rem = $target / $n;
		if (try_opsr3($rem,@nums)) { return 1; }
	}
	if ($target > $n && substr($target,-length($n)) eq $n) {
		my $front = substr($target,0,length($target)-length($n));
		if (try_opsr3($front,@nums)) { return 1; }
	}
	my $diff = $target - $n;
	return 0 if ($diff <= 0);
	return try_opsr3($diff,@nums);
}

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
