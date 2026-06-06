#!perl -w
use strict;
use Time::HiRes qw(time);

my $t0 = time;


my $part1 = 0;
my $part2 = 0;
my $pos = 50;

while (<>) {
	chomp;
	my $r;
	my $old = $pos;
	if (/R(\d+)$/i) {
		$r = $1;
		$pos += $r;
		while ($pos >= 100) {
			$pos -= 100;
			$part2 += 1;
		}
		if ($pos == 0) {
			$part1 += 1;
		}
	}
	elsif (/L(\d+)$/i) {
		$r = -$1;
		$pos += $r;
		while ($pos < 0) {
			$pos += 100;
			$part2 += 1;
		}
		if ($pos == 0) {
			$part1 += 1;
			$part2 += 1;
		}
		if ($old == 0) {
			$part2 -= 1;
		}
	}
	else { die("Bad input $_"); }
	
#	printf("part1: %d, part2: %d, pos: %d, r: %d\n", $part1, $part2, $pos, $r);
}

my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.3fs\n", $used);
