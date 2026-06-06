#!perl -w
use strict;
use Time::HiRes qw (time);

my $part1 = 0;
my $part2 = 0;

sub safe
{
	my (@l) = @_;
	my $p = $l[0];
	my $c = $l[1];
	if ($c > $p) {
		for (my $i = 1; $i < scalar(@l); $i++) {
			$c = $l[$i];
			my $d = $c-$p;
			if ($d < 1 or $d > 3) {
				return 0;
			}
			$p = $c;
		}
		return 1;
	}
	if ($c < $p) {
		for (my $i = 1; $i < scalar(@l); $i++) {
			$c = $l[$i];
			my $d = $p-$c; # Reverse difference!
			if ($d < 1 or $d > 3) {
				return 0;
			}
			$p = $c;
		}
		return 1;
	}
	return 0;
}

sub safe2
{
	my @l = @_;
	for (my $i = 0; $i < scalar(@l); $i++) {
		my @s = @l;
		splice(@s,$i,1); # Try to remove one entry
		if (safe(@s) > 0) {
			return 1;
		}
	}
	return 0;
}

my $start = time;

while (<>) {
	chomp;
	my @l = split;
	if (safe(@l)) {
		$part1++;
		$part2++;
	}
	else {
		$part2 += safe2(@l);
	}
}

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
