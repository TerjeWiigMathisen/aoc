#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);
#use List::Util qw (reduce);

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

my @monkeys;
my @op;
my @divby;
my @dest;

my $inp = '';
foreach (<F>) {
	$inp .= $_;
}
close(F);

@monkeys = split(/\n\n/, $inp);
my @inspect = ();

my %divisors = ();

my @savemonkeys = @monkeys;
for (my $round = 1; $round <= 20; $round++) {
	my $mon = 0;
	foreach (@monkeys) {
		my ($mnr, $strt, $oper, $test, $ift, $iff) = split(/\n/);
		my @items;
		my ($op, $num, $div, $tt, $ft);
		if ($strt =~ /Starting items: (.*)$/) {
			@items = split(/,/,$1);
		}
		else {
			die("Starting not found $start");
		}
		if ($oper =~ /old\s+([\+\*])\s+(\S+)/) {
			($op, $num) = ($1, $2);
		}
		else {
			die("operation not found $oper");
		}
		if ($test =~ /Test: divisible by (\d+)/) {
			$div = $1;
			$divisors{$div} = 1;
		}
		else { die("Test not found $test"); }
		if ($ift =~ /If true\: throw to monkey (\d+)/) {
			$tt = $1;
		}
		else { die("If true not found $ift"); }
		if ($iff =~ /If false\: throw to monkey (\d+)/) {
			$ft = $1;
		}
		else { die("If true not found $ift"); }
		
#		printf(STDERR "Round $round Monkey $mon %s, $op, $num, $div, $tt, $ft\n", join(",",@items));
		
		foreach (@items) {
			$inspect[$mon]++;
			my $worry = $_;
			if ($num eq 'old') {
				if ($op eq '+') {
					$worry += $worry;
				}
				elsif ($op eq '*') {
					$worry *= $worry;
				}
			}
			else {
				if ($op eq '+') {
					$worry += $num;
				}
				elsif ($op eq '*') {
					$worry *= $num;
				}
			}
			$worry = int($worry/3);
			my $target = $ft;
			if ($worry % $div == 0) { 
				$target = $tt; 
			}
			my @target = split(/\n/, $monkeys[$target]);
			if ($target[1] =~ /Starting items: \d/) {
				$target[1] .= ", $worry";
			}
			else {
				$target[1] .= "$worry";
			}
			$monkeys[$target] = join("\n", @target);
		}
		$strt = "Starting items: ";
		$monkeys[$mon] = join("\n", $mnr, $strt, $oper, $test, $ift, $iff);
		$mon++;
	}
	#printf(STDERR "%s\n\n", join("\n",@monkeys));
	my $modulus = 1;
	foreach (keys %divisors){
		$modulus *= $_;
	}
#	my $m = 0;
	foreach (@monkeys) {
		my @lines = split(/\n/, $_);
		if ($lines[1] =~ /Starting items: (.*)$/) {
			my @items = split(",", $1);
			my @reduced = ();
			foreach (@items) {
				push(@reduced, $_ % $modulus);
			}
			$lines[1] = "Starting items: ".join(", ", @reduced);
		}
		$_ = join("\n", @lines);
#		printf("%s\n", $inspect[$m]);
#		$m++;
	}
}

printf(STDERR "Part1 time = %f\n", time - $start);

@inspect = sort {$b <=> $a} @inspect;

$part1 = $inspect[0]*$inspect[1];

@monkeys = @savemonkeys;
for (my $round = 1; $round <= 10000; $round++) {
	my $mon = 0;
	foreach (@monkeys) {
		my ($mnr, $strt, $oper, $test, $ift, $iff) = split(/\n/);
		my @items;
		my ($op, $num, $div, $tt, $ft);
		if ($strt =~ /Starting items: (.*)$/) {
			@items = split(/,/,$1);
		}
		else {
			die("Starting not found $start");
		}
		if ($oper =~ /old\s+([\+\*])\s+(\S+)/) {
			($op, $num) = ($1, $2);
		}
		else {
			die("operation not found $oper");
		}
		if ($test =~ /Test: divisible by (\d+)/) {
			$div = $1;
			$divisors{$div} = 1;
		}
		else { die("Test not found $test"); }
		if ($ift =~ /If true\: throw to monkey (\d+)/) {
			$tt = $1;
		}
		else { die("If true not found $ift"); }
		if ($iff =~ /If false\: throw to monkey (\d+)/) {
			$ft = $1;
		}
		else { die("If true not found $ift"); }
		
#		printf(STDERR "Round $round Monkey $mon %s, $op, $num, $div, $tt, $ft\n", join(",",@items));
		
		foreach (@items) {
			$inspect[$mon]++;
			my $worry = $_;
			if ($num eq 'old') {
				if ($op eq '+') {
					$worry += $worry;
				}
				elsif ($op eq '*') {
					$worry *= $worry;
				}
			}
			else {
				if ($op eq '+') {
					$worry += $num;
				}
				elsif ($op eq '*') {
					$worry *= $num;
				}
			}
			$worry = int($worry/3);
			my $target = $ft;
			if ($worry % $div == 0) { 
				$target = $tt; 
			}
			my @target = split(/\n/, $monkeys[$target]);
			if ($target[1] =~ /Starting items: \d/) {
				$target[1] .= ", $worry";
			}
			else {
				$target[1] .= "$worry";
			}
			$monkeys[$target] = join("\n", @target);
		}
		$strt = "Starting items: ";
		$monkeys[$mon] = join("\n", $mnr, $strt, $oper, $test, $ift, $iff);
		$mon++;
	}
	#printf(STDERR "%s\n\n", join("\n",@monkeys));
	my $modulus = 1;
	foreach (keys %divisors){
		$modulus *= $_;
	}
#	my $m = 0;
	foreach (@monkeys) {
		my @lines = split(/\n/, $_);
		if ($lines[1] =~ /Starting items: (.*)$/) {
			my @items = split(",", $1);
			my @reduced = ();
			foreach (@items) {
				push(@reduced, $_ % $modulus);
			}
			$lines[1] = "Starting items: ".join(", ", @reduced);
		}
		$_ = join("\n", @lines);
#		printf("%s\n", $inspect[$m]);
#		$m++;
	}
}

printf(STDERR "Total time = %f\n", time - $start);

@inspect = sort {$b <=> $a} @inspect;

$part2 = $inspect[0]*$inspect[1];
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);
