#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);

#use bigint;

#use JSON::Parse;
#no warnings 'recursion';

my $start = time;

my $DEBUG = 0;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

my $part1 = '';
my $part2 = '';

my @freq = ();
foreach (@inp) {
	my @c = split(//);
	for (my $i = 0; $i < scalar(@c); $i++) {
		my $c = $c[$i];
		$freq[$i]->{$c}++;
	}
}

for (my $i = 0; $i < scalar(@freq); $i++) {
	my $ref = $freq[$i];
	my @l = sort {$ref->{$b} <=> $ref->{$a} } keys %{$ref};
	$part1 .= $l[0];
	$part2 .= $l[-1];
}
		
printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
