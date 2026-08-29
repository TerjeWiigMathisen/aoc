	#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);
#use List::Util qw (reduce);
use List::PriorityQueue;
#use bigint;

#use JSON::Parse;
#no warnings 'recursion';

my $DEBUG = 0;

my $fname = shift;
$fname = 'input.txt' unless (defined($fname));
open(F ,'<',$fname);

my $start = time;

my $part1 = 0;
my $part2 = 0;


my @inp = ();
if ($fname eq 't.txt') {
}

foreach (<F>) {
	chomp;
	push(@inp,$_);
}
close(F);

my %fixed;
my %operation;

foreach (@inp) {
	chomp;
	die("Parse: $_\n") unless (/^(\D+): (.+)$/);
	my ($target, $operation) = ($1,$2);
	$operation{$target} = $operation;
	if ($operation =~ /^\d+$/) {
		$fixed{$target} = $operation;
	}
}

sub evaluate
{
	my ($tg) = @_;
	if (defined($fixed{$tg})) {
		return $fixed{$tg};
	}
	my $op = $operation{$tg};
	if ($op =~ /^\d+$/) {
		$fixed{$tg} = $op;
		return $op;
	}
	die("Bad operation: $op") unless ($op =~ /^(\S+) (\S) (\S+)$/);
	my ($a,$o,$b) = ($1,$2,$3);
	my ($r, $ar,$br);
	$ar = evaluate($a);
	$br = evaluate($b);
	if ($o eq '+') {
		$r = $ar + $br;
	}
	elsif ($o eq '-') {
		$r = $ar - $br;
	}
	elsif ($o eq '*') {
		$r = $ar * $br;
	}
	elsif ($o eq '/') {
		$r = $ar / $br;
	}
	else {
		die("Bad op: $o");
	}
	$fixed{$tg} = $r;
#	printf("%s fixed at %d\n", $tg, $r);
	return $r;
}
	
sub evaluate_fixed
{
	my $fixed = 0;
	do {
		$fixed = 0;
		foreach (keys %operation) {
			my $tg = $_;
			next if ($tg eq 'humn');
			next if (defined($fixed{$tg}));
			my $op = $operation{$tg};
			if ($op =~ /^\d+$/) {
				$fixed{$tg} = $op;
				$fixed++;
			}

			die("Bad operation: $op") unless ($op =~ /^(\S+) (\S) (\S+)$/);
			my ($a,$o,$b) = ($1,$2,$3);
			my ($r, $ar,$br) = (0,$fixed{$a},$fixed{$b});
			next unless (defined($ar) && defined($br));
			if ($o eq '+') {
				$r = $ar + $br;
			}
			elsif ($o eq '-') {
				$r = $ar - $br;
			}
			elsif ($o eq '*') {
				$r = $ar * $br;
			}
			elsif ($o eq '/') {
				$r = $ar / $br;
			}
			else {
				die("Bad op: $o");
			}
			$fixed{$tg} = $r;
			$fixed++;
			printf("%s fixed at %d\n", $tg, $r);
		}
	} while ($fixed);
	printf("Total ops: %d, fixed: %d, remain: xx\n", scalar(@inp), scalar(keys %fixed));
	return scalar(keys %fixed);
}
	
$part1 = evaluate('root');
printf("Part1: %s\n", $part1);
printf(STDERR "Total time = %f\n", time - $start);

#evaluate_fixed();
#my %f = %fixed;

sub test
{
	my ($h) = @_;
	%fixed = ();
	%operation = ();
	foreach (@inp) {
		chomp;
		die("Parse: $_\n") unless (/^(\D+): (.+)$/);
		my ($target, $operation) = ($1,$2);
		$operation{$target} = $operation;
		if ($operation =~ /^\d+$/) {
			$fixed{$target} = $operation;
		}
	}
	$operation{root} =~ s/ \S / - /;
	$fixed{humn} = $h;
	my $r = evaluate('root');
#	printf("$h -> %s\n", $r);
	if ($r == 0) {
		$part2 = $h;
	}
	return $r;
}

my $lo = 0;
my $lor = test($lo);
my $hi = 1e14;
my $hir;
while (($hir=test($hi))*$lor > 0) {
	$lo = $hi;
	$lor = $hir;
	$hi *= 10;
}
while (abs($hi-$lo) > 1) {
	my $mid = $lo + int(($hi-$lo)*$hir/($hir-$lor));
	my $mir = test($mid);
	if ($mir == 0) {
		$part2 = $mid;
		last;
	}
	if ($mir*$lor > 0) {
		$lo = $mid;
		$lor = $mir;
	}
	else {
		$hi = $mid;
		$hir = $mir;
	}
}

printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);

