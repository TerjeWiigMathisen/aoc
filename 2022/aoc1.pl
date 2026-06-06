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
open(F ,'<',$fname);

my $start = time;

my $q = q(
my $file;
sysread(F, $file, 1e9);
my @elves = split(/\r?\n\r?\n/, $file);
my @cal;
foreach (@elves) {
	push(@cal, reduce {$a+$b} 0,split(/\r?\n/));
}
@cal = reverse sort {$a<=>$b} @cal;
);

#my $q = q(
my @cal = ();
my $cal = 0;
foreach (<F>) {
	chomp;
	if (/^\s*$/) {
		push(@cal, $cal);
		$cal = 0;
		next;
	}
	$cal += $_;
}
push(@cal, $cal);
@cal = reverse sort {$a<=>$b} @cal;
#);

my $part1 = $cal[0];
my $part2 = $cal[0]+$cal[1]+$cal[2];

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
