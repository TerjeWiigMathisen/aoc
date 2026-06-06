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
open(F,$fname) || die("Open");
my @code = (<F>);
close(F);

my $part1 = 0;
my $part2 = 0;

my %body;
my %sats;

foreach (@code) {
	chomp;
	my ($body, $sat) = split(/\)/);
	$body{$sat} = $body;
	push(@{$sats{$body}}, $sat);
}

sub orbits
{
	my ($body) = @_;
	my $o = 0;
	foreach (keys %body) { # All sats
		my $s = $_;
		while ($body{$s}) {
			$o++;
			$s = $body{$s};
		}
	}
	return $o;
}

sub path_com
{
	my ($obj) = @_;
	my @p = ();
	while ($obj ne 'COM') {
		$obj = $body{$obj};
		push(@p,$obj);
	}
	return @p;
}

$part1 = orbits('COM');

my @you = reverse path_com('YOU');
my @san = reverse path_com('SAN');
my $i;
for ($i = 0; 1; $i++) {
	if ($you[$i] ne $san[$i]) {
		$part2 = scalar(@you)+scalar(@san)-$i*2;
		last;
	}
}

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
