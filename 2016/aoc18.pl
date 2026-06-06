#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);
use List::PriorityQueue;
use Digest::MD5;

#use bigint;

#use JSON::Parse;
no warnings 'recursion';

my $start = time;

my $DEBUG = 0;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

my $part1;
my $part2 = 0;

my $tiles = "..^^.";

sub next_line
{
	my ($tiles) = @_;
	my $t = '.'.$tiles.'.';
	my $n = '';
	for (my $i = 0; $i < length($tiles); $i++) {
		my $prev = substr($t,$i,3);
		$n .= index("^^. .^^ ^.. ..^", $prev) >= 0 ? "^" : ".";
	}
	return $n;
}

my @map = @inp;
while (scalar(@map) < 40) {
	push(@map,next_line($map[-1]));
}

sub safe
{
	my @map = @_;
	my $safe = join("", @map);
	$safe =~ s/\^//g;
	return length($safe);
}

$part1 = safe(@map);

my $line = $inp[0];
$part2 = safe($line);
for (my $l = 1; $l < 400000; $l++) {
	$line = next_line($line);
	$part2 += safe($line);
}

printf(STDERR "Total time = %f\n", time - $start);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);

printf("%s\n\n", join("\n", @map));

