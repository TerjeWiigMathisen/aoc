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

my @stacks = ();

my @lines = ();
my @c;
foreach (<F>) {
	chomp;
	@c = split(//);
}
close(F);

sub unique
{
	my ($len, @c) = @_; # Unique length to search for, followed by character array
	my @last = (-1) x 256; # Marker so show none of the letters have been seen.
	my $lastduplicate = -1;
	for (my $i = 0; $i <= scalar(@c); $i++) {
		my $c = ord($c[$i]); # Character id
		my $l = $last[$c];
		if ($l > $lastduplicate) {
			$lastduplicate = $l;
		}
		$last[$c] = $i;
		if ($i - $lastduplicate >= $len) {
			return $i+1;
		}
	}
}

$part1 = unique(4,@c);
$part2 = unique(14,@c);

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
