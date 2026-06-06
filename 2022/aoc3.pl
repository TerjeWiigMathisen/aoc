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

my $part1 = 0;
my $part2 = 0;

sub pri
{
	my ($v) = @_;
	return ($v ge 'a') ? ord($_)-ord('a')+1 : ord($_)-ord('A')+27;
}

my @group = ();
foreach (<F>) {
	chomp;
	#printf(STDERR "%s\n", $_);
	my @letters = (); #(0) x 256;
	#print scalar(@letters), @letters;
	my $v;
	push(@group, $_);
	my $f = substr($_, 0, length($_)>>1);
	my $l = substr($_, length($f));
	my %found = ();
	foreach (split(//, $f)) {
		$found{$_}++;
	}
	foreach (split(//, $l)) {
		if ($found{$_}) {
			$part1 += pri($_);
			last;
		}
	}
	if (scalar(@group) == 3) {
		my %found1 = ();
		foreach (split(//, $group[0])) {
			$found1{$_}++;
		}
		my %found2 = ();
		foreach (split(//, $group[1])) {
			if ($found1{$_}) { $found2{$_}++; }
		}
		foreach (split(//, $group[2])) {
			if ($found2{$_}) {
				$part2 += pri($_);
				last;
			}
		}
		@group = ();
	}
}
close(F);

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
