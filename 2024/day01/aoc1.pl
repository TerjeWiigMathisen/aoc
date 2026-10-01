#!perl -w
use strict;

my $part1 = 0;
my $part2 = 0;

my @l = ();
my @r = ();
my %r = ();
while (<>) {
	chomp;
	my ($l, $r) = split(/\s+/); # Split the input into two arrays
	push(@l, $l); push(@r, $r);
	$r{$r}++;	# Count instances of the second in a dict
}
printf("There are %d unique numbers in r\n",scalar(keys %r));

@l = sort {$a <=> $b} @l; # Sort both
@r = sort {$a <=> $b} @r;

my $match = 0;
for (my $i = 0; $i < scalar(@l); $i++) {
	my ($l, $r) = ($l[$i], $r[$i]);
	my $d = $l - $r;
	$part1 += abs($d);
	if (defined($r{$l})) {
		$part2 += $l * $r{$l};
		$match++;
	}
}
printf("%d matches between l and r\n", $match);
printf("%s\n%s\n", $part1, $part2);
