#!perl -w

use strict;
use Time::HiRes qw (time);
use Math::BigInt;

my $start = time;

my @inp = (<>);
chomp(@inp);

my %path = ();

foreach (@inp) {
	my ($s,$e) = split(/-/);
	${$path{$s}}->{$e}++;
	${$path{$e}}->{$s}++;
}
my ($a12, $b12, $c12);
my %visits = ();

#try('start');
#%visits = ();
tryb('start',0);

printf(STDERR "Total time = %f\n", time - $start);
printf("aoc12a: %d\n", $b12-$c12);
printf("aoc12b: %d\n", $b12 );

sub try
{
	my ($node, @solution) = @_;
	$visits{$node}++;
	push(@solution, $node);
	if ($node eq 'end') {
#		printf(STDERR "%s\n", join ("-", @solution));
		$a12++;
		return;
	}
	if ($node ge 'a' && $visits{$node} > 1) {
		$visits{$node}--;
		return;
	}
	foreach (sort keys %{${$path{$node}}}) {
		try($_, @solution);
	}
	$visits{$node}--;
}

sub tryb
{
	my ($node, $twice, @solution) = @_;
	$visits{$node}++;
	push(@solution, $node);
	if ($node eq 'end') {
#		printf(STDERR "%s\n", join ("-", @solution));
		$b12++;
		$c12+= $twice;
		return;
	}
	if ($node ge 'a' && $visits{$node} > 1) {
		if ($twice > 0 || $node eq 'start') {
			$visits{$node}--;
			return;
		}
		$twice++;
	}
	foreach (sort keys %{${$path{$node}}}) {
		tryb($_, $twice, @solution);
	}
	$visits{$node}--;
}
