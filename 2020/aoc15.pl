#!perl -w 

use strict;
use Time::HiRes qw(time);
#use bigint;

my $inp = 
q(0,3,6);

if (defined(shift)) {
	$inp = 
q(8,0,17,4,1,12);

}

my $t = time();

my @inp = split(/,/,$inp);

my %last = ();

my $turn = 0;
my $l;
foreach (@inp) {
	$turn++;
	$l = $_;
	$last{$l} = $turn;
	#printf(STDERR "%5d, %6d\n", $turn, $l);
}

my %prev = ();

do {
	my $c = $turn; #$last{$l};
	my $p = $prev{$l};
	$l = 0;
	if (defined($p)) {
		$l = $c - $p;
	}
	$turn++;
	$prev{$l} = $last{$l};
	$last{$l} = $turn;
	printf(STDERR "%5d, %6d\n", $turn, $l) if ($turn == 2020);
} while ($turn < 30000000);

printf("%d\n", $l);

printf(STDERR "Max value seen = %d\n", (sort {$a <=> $b} keys %last)[-1]);


$t = time()-$t;
printf("Total time = %1.5fms\n", $t*1000);

exit();

