#!perl -w
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

my $inp = '';

while (<>) {
	$inp .= $_;
	chomp;
	push(@lines, $_);
}

#part1
my @parts = split(/\n\n/,$inp);
my $tasks = pop @parts;
my @presents = ();
foreach (@parts) {
	my ($index, @lines) = split(/\n/);
	chop($index);
	my $hash = tr/#//;
	my $size = $hash;
	my %p = ();
	$p{size} = $size;
	$p{part} = @lines;
	$presents[$index] = \%p;
}

sub fit
{
	my ($x,$y,@counts) = @_;
	my $space = $x*$y;
	my $spaceneeded = 0;
	for (my $p = 0; $p < scalar(@counts); $p++) {
		$spaceneeded += ($presents[$p]->{size} * $counts[$p]);
	}
	printf("Total size: %d, minimum needed: %d, fill rate: %1.2f%%\n",
		$space, $spaceneeded, $spaceneeded*100.0/$space);
	return $spaceneeded <= $space;
}

my @tasks = split(/\n/,$tasks);
foreach (@tasks) {
	my ($layout, @counts) = split;
	chop($layout);
	my ($x,$y) = split(/x/,$layout);
	if (fit($x,$y,@counts)) {
		$part1++;
	}
}

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
