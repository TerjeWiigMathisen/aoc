#!perl -w

use strict;

my $pos = 0;
my $depth = 0;
my $aim = 0;

while (<>) {
	chomp;
	if (/^forward\s+(\d+)$/) {
		$pos += $1;
		$depth += $1 * $aim;
	}
	elsif (/^down\s+(\d+)$/) {
		$aim += $1;
	} elsif (/^up\s+(\d+)$/) {
		$aim -= $1;
	}
}

printf("%d, %d, %d, %d\n",$pos, $depth, $aim, $pos*$depth);
