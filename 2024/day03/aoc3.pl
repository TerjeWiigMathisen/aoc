#!perl -w
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;


my $start = time;

my $enable = 1;
while (<>) {
	chomp;
	while (/mul\((\d+),(\d+)\)/) {
		my($a,$b) = ($1,$2);
		my $pre = $`;
		my $post = $';
		my $do = rindex($pre,"do()"); # -1 if not found!
		my $dont = rindex($pre,"don't()");
		if ($dont > $do) {
			$enable = 0;
		}
		elsif ($do > $dont) {
			$enable = 1;
		}
		if ($a < 1000 && $b < 1000) {
			$part1 += $a*$b;
			$part2 += $a*$b*$enable;
		}
		else {
			printf("a = $a, b = $b\n");
		}
		$_ = $post;
	}
}

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
