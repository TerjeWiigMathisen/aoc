#!perl -w

use strict;

my ($f, $r) = (0,0);
for (my $d = 1; $d < 31; $d++) {
	for (my $m = 1; $m < 12; $m++) {
		next if ($m == $d);
		my $y2 = $d*$d+$m*$m;
		my $y = int(sqrt($y2));
		if ($y*$y == $y2) {
			printf("F %2d-%02d-%02d\n", 2000+$y, $m, $d);
			$f++;
		}
		$y2 = abs($d*$d-$m*$m);
		$y = int(sqrt($y2));
		if ($y*$y == $y2) {
			printf("R %2d-%02d-%02d\n", 2000+$y, $m, $d);
			$r++;
		}
	}
}
printf("%d forward, %d backwards\n", $f, $r);
