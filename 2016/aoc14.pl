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
my $part2;

my $salt = "ahsbgdzn"; # "abc";

my @hash = ();

sub triple
{
	my ($s) = @_;
	for (my $i = 2; $i < length($s); $i++) {
		if (substr($s,$i-2,3) eq (substr($s,$i,1) x 3)) {
			return substr($s,$i,1); # Return first triple combo!
		}
	}
	return '';
}

sub hash
{
	my ($idx) = @_;
	if (defined($hash[$idx])) {
		return $hash[$idx];
	}
	return $hash[$idx] = Digest::MD5::md5_hex(sprintf("%s%d",$salt, $idx));
}

my @keys = ();

for (my $idx = 0; scalar(@keys) < 64 ; $idx++) {
	my $hash = hash($idx);
	if (my $t = triple($hash)) {
		$t = $t x 5;
		for (my $n = $idx+1; $n <= $idx+1000; $n++) {
			my $h = hash($n);
			if (index($h, $t) >= 0) {
				push(@keys, $hash);
				printf("Key %d for idx = %d, $t in %d (%s - %s)\n", scalar(@keys), $idx, $n, $hash, $h);
				$part1 = $idx;
				last;
			}
		}
	}
}

@hash = ();

sub hash2016
{
	my ($idx) = @_;
	if (defined($hash[$idx])) {
		return $hash[$idx];
	}
	my $h = Digest::MD5::md5_hex(sprintf("%s%d",$salt, $idx));
	for (my $i = 0; $i < 2016; $i++) {
		$h = Digest::MD5::md5_hex($h);
	}
	$hash[$idx] = $h;
}

my @keys = ();

for (my $idx = 0; scalar(@keys) < 64 ; $idx++) {
	my $hash = hash($idx);
	if (my $t = triple($hash)) {
		$t = $t x 5;
		for (my $n = $idx+1; $n <= $idx+1000; $n++) {
			my $h = hash2016($n);
			if (index($h, $t) >= 0) {
				push(@keys, $hash);
				printf("Key %d for idx = %d, $t in %d (%s - %s)\n", scalar(@keys), $idx, $n, $hash, $h);
				$part2 = $idx;
				last;
			}
		}
	}
}

printf(STDERR "Total time = %f\n", time - $start);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
