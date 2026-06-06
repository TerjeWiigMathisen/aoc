#!perl -w
# Used 1.824ms

use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();
my @ing = (); 

while (<>) {
	chomp;
	last unless ($_); # Found the blank separator line!
	push(@lines, sprintf("%15d-%15d", split(/-/)));
}
@lines = sort @lines; # Sorted by range start, range end

my @fresh = (); # Pairs of [first..last> ranges of fresh ingredients
my ($first, $last); # Current range
foreach (@lines) {
	my ($f,$l) = split(/-/); # Inclusive range: [f..l]
	# Make the range endpoint exclusive: [f..l>
	$l++; 
	unless (defined($last)) {
		($first, $last) = ($f, $l);
		next;
	}
	if ($f > $last) {
		push(@fresh, $first, $last);
		$part2 += $last-$first;
		($first, $last) = ($f, $l);
	}
	elsif ($l > $last) {
		$last = $l;
	}
#	printf("first-last: $first $last, curr: $f, $l\n"); 
}
push(@fresh, $first, $last);
$part2 += $last-$first;

while (<>) {
	chomp;
	push(@ing, $_);
}

@ing = sort {$a <=> $b} @ing;
my $guard = $ing[-1]+1; # Higher than all ing 

push(@fresh, $guard,$guard); #  Zero-length guard range simplifies the following code!

my $f = 1;
foreach (@ing) {
	my $ing = $_;
	while ($fresh[$f] < $ing) {
		$f += 2;
	}
	if ($fresh[$f-1] <= $ing) {
		$part1++;
	}
}

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
