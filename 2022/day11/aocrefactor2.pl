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
my $modulus;
my @op;
my @divby;
my @dest;

my $inp = '';
foreach (<F>) {
	$inp .= $_;
}
close(F);

sub parse
{
	@monkeys = split(/\n\n/, $inp);
	# Parse the input and simplify it
	my %divisors = ();
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
		else { die("If false not found $ift"); }
		
		if ($num eq 'old') {
			$num = '$worry';
		}
		my $operation = sprintf(q($worry%s=%s;),$op, $num);
		$_ = join(",",join("\t",@items),$operation,$div,$tt,$ft);
#		printf("%s\n",$_);
	}
	$modulus = 1;
	foreach (keys %divisors){
		$modulus *= $_;
	}
}

parse();

my @inspect = ();

sub doround
{
	my ($divisor, $reduce) = @_;
	
	my $mon = 0;
	foreach (@monkeys) {
		my @items;
		my ($items,$operation, $div, $tt, $ft) = split(/,/);
		@items = split(/\t/,$items);
		
		foreach (@items) {
			$inspect[$mon]++;
			my $worry = $_;
			eval($operation);
			if ($divisor > 1) {
				$worry = int($worry/$divisor);
			}
			else {
				$worry = $worry % $modulus;
			}
			my $target = $ft;
			if ($worry % $div == 0) { 
				$target = $tt; 
			}
			my @target = split(/,/, $monkeys[$target]);
			my @titems = (split(/\t/,$target[0]),$worry);
			$target[0] = join("\t",@titems);
			$monkeys[$target] = join(",", @target);
		}
		$monkeys[$mon] = join(",", "", $operation, $div, $tt, $ft);
		$mon++;
	}
}

my @savemonkeys = @monkeys;
for (my $round = 1; $round <= 20; $round++) {
	doround(3,0);
}
@inspect = sort {$b <=> $a} @inspect;
$part1 = $inspect[0]*$inspect[1];

printf(STDERR "Part1 time = %f\n", time - $start);

@monkeys = @savemonkeys;
@inspect = ();
for (my $round = 1; $round <= 10000; $round++) {
	doround(1,1);
}
@inspect = sort {$b <=> $a} @inspect;
$part2 = $inspect[0]*$inspect[1];

printf(STDERR "Total time = %f\n", time - $start);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
