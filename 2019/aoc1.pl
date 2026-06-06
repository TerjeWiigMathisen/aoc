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

sub fuel
{
	my ($m, $recursive) = @_;
	my $f = int($m/3)-2;
	return 0 if ($f <= 0);
	if (defined($recursive) && $recursive) {
		$f += fuel($f,1);
	}
	return $f;
}

my $part1 = 0;
my $part2 = 0;
while (<>) {
	chomp;
	my $mass = $_;
	my $fuel = int($mass/3)-2;
	$part1 += fuel($mass,0);
	$part2 += fuel($mass, 1);
}

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
