#!perl -w

use strict;

my $LIMIT = 100000;
my $SQRTLIM = int(sqrt($LIMIT)+1);

my %primes = ();
my @numbers = ();
foreach (0..$LIMIT) { $numbers[$_] = $_; }
$numbers[1] = 0;

for (my $r = 2; $r < $SQRTLIM; $r++) {
	if ($numbers[$r]) {
		$primes{$r} = 1;
		for (my $i = $r+$r; $i <= $LIMIT; $i+=$r) {
			$numbers[$i] = 0;
		}
	}
}

my @prime_to_here = ();
my $primecount = 0;
for (my $r = 0; $r <= $LIMIT; $r++) {
	if ($numbers[$r]) {
		$primes{$r} = ++$primecount;
	}
	$prime_to_here[$r] = $primecount;
}

printf(STDERR "Found %d primes below $LIMIT\n", $primecount);

my $G = <>+0;
while (<>) {
	chomp;
	printf("%s\n", $prime_to_here[$_] & 1 ? "Alice" : "Bob");
}
