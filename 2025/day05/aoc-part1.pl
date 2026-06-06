#!perl -w
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();
my @ids = (); 

while (<>) {
	chomp;
	last unless ($_);
	push(@lines, $_);
}

while (<>) {
	chomp;
	push(@ids, $_);
}

@ids = sort {$a <=> $b} @ids;

foreach (@ids) {
	my $ing = $_;
	my $ok = 0;
	foreach (@lines) {
		my ($f, $l) = split(/-/);
		if ($f <= $ing && $ing <= $l) {
			$part1++;
			last;
		}
	}
}
#part2

my @fresh = ();
@lines = sort {(split(/-/,$a))[0] <=> (split(/-/,$b))[0]} @lines;
#printf("%s\n\n", join("\n", @lines));
@fresh = split(/-/,shift @lines);

my $pos = 1;
foreach (@lines) {
	my ($f, $l) = split(/-/);
	for (my $p = 0; $p < scalar(@fresh); $p+=2) {
		if ($f >= $fresh[$p] && $f <= $fresh[$p+1]) {
			$f = $fresh[$p+1];
		}
	}
	if ($f <= $l) {
		push(@fresh,$f,$l);
	}
}

for (my $i = 0; $i < scalar(@fresh); $i+=2) {
	printf("%d - %d\n", $fresh[$i],$fresh[$i+1]);
	$part2 += $fresh[$i+1]-$fresh[$i]+1;
}

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
