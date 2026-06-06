#!perl -w 

use strict;

my %seen = ();
for (my $i = 1; $i < 9; $i++) {
	for (my $j = $i+1; $j <= 9; $j++) {
		my $p = ($i*$j) % 10;
		$seen{$p}++;
	}
}

my @prod = sort {$seen{$a} <=> $seen{$b}} keys %seen;
foreach (@prod) {
	printf("prod: %d fantes %d ganger\n", $_, $seen{$_});
}
