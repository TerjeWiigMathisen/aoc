#!perl -w
use strict;
use Time::HiRes qw (time);
use English;
use List::PriorityQueue;


my $part1 = 0;
my $part2 = 0;

my $start = time;

my $inp = '';
while (<>) { $inp .= $_; }

my ($patterns, $towels) = split(/\n\n/,$inp);
my @patterns = split(/, /,$patterns);
my @towels = split(/\n/,$towels);

my $maxpatlen = 0;
my %pat = ();
foreach (@patterns) {
	$pat{$_}++;
	$maxpatlen = length($_) if (length($_) > $maxpatlen);
}

my %tail;
sub ways
{
	my ($need) = @_;
	return $tail{$need} if (defined($tail{$need}));

	my $ways = 0;
	for (my $i = 1; $i <= $maxpatlen; $i++) {
		last if ($i > length($need));
		my $t = substr($need,0,$i);
		if (defined($pat{$t})) {
			my $rem = substr($need,$i);
			$ways += ways($rem);
		}
	}
	$tail{$need} = $ways;
	return $ways;
}

foreach (split(/\n/,$towels)) {
	%tail = ('',1);
	my $w = ways($_);
	$part1 += $w > 0;
	$part2 += $w;
}

my $used = time - $start;
printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
