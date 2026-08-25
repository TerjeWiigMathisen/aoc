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

my $iterations = 0;
do {
	$iterations++;
	my @next = ();
	foreach (@inp) {
		chomp;
		die("Parse: $_\n") unless (/^(\D+): (.+)$/);
		my ($target, $operation) = ($1,$2);
		if ($operation =~ /^\d+$/) {
			$fixed{$target}++;
			printf("\tlet $target = $operation;\n");
			next;
		}
		die("Parse op: $operation") unless ($operation =~ /^(\D+) (\D) (\D+)$/);
		my ($a, $op, $b) = ($1, $2, $3);
		if (defined($fixed{$a}) && defined($fixed{$b})) {
			$fixed{$target}++;
			printf("\tlet $target = $operation;\n");
			next;
		}
		push(@next, $_);
	}
	@inp = @next;
	printf(STDERR "%d statements remain\n", scalar(@inp));
} while (scalar(@inp));

printf(STDERR "%d total iterations to evaluate all statements\n", $iterations);