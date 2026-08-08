#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);
#use List::Util qw (reduce);

#use bigint;

#use JSON::Parse;q
#no warnings 'recursion';

my $DEBUG = 0;

my $fname = shift;
open(F ,'<',$fname);

my $start = time;

my $part1 = 0;
my $part2 = 0;

my @stacks = ();

my @lines = ();
my @c;
my @CD = ();
my %size = ();
my %subdirs = ();
foreach (<F>) {
	chomp;
	if (/^\$ cd (\S+)$/) {
		my $d = $1;
		if ($d eq "\/") {
			@CD = ("\/");
		}
		elsif ($d eq "\.\.") {
			@CD = (@CD)[0..scalar(@CD)-2] if (scalar(@CD) > 1);
		}
		else {
			my @D = split(/\//, $d);
			push(@CD, @D);
		}
		printf(STDERR "CD = %s\n", join(",", @CD));
	}
	elsif (/^\$ ls/) {
		# Listing follows
		#$list = 1;
	}
	elsif (/^(\d+) (\S+)$/) {
		$size{$CD[0]} += $1;
		for (my $d = 1; $d < scalar(@CD); $d++) {
			my $curdir = join(",",@CD[0..$d]);
			$size{$curdir} += $1;
		}
	}
	elsif (/^dir (\S+)$/) {
		push(@{$subdirs{join(",",@CD)}}, $1);
	}
}
close(F);

my @dirbysize = sort {$size{$a} <=> $size{$b}} (keys %size);
my $sum = 0;
foreach (@dirbysize) {
	my $s = $size{$_};
	printf(STDERR "%10d %s\n", $s, $_);
	if ($s <= 100000) {
		$sum += $s;
	}
}
$part1 = $sum;

my $totalspace = 70000000;
my $used = $size{"\/"};
my $free = $totalspace - $used;
my $need =       30000000 - $free;
printf(STDERR "Must free up at least $need\n");

foreach (reverse @dirbysize) {
	my $s = $size{$_};
	printf(STDERR "%10d %s\n", $s, $_);
	if ($s >= $need) {
		$part2 = $s;
	}
}

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
