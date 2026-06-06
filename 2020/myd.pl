#!perl -w

use strict;
use bigint;

my $y = 10;
my $power = 1024;
my $diff;

while (1) {
	$diff = $power - 615;
	my $x = int(sqrt($diff));
	if ($x*$x + 615 == $power) {
		printf("x = %s, y = %s\n", $x, $y);
	}
	$y++;
	$power += $power;
}