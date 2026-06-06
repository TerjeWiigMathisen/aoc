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

my @r = ();
foreach (@inp) {
	my ($f,$e) = split(/-/);
	push(@r,sprintf("%10d-%10d",$f,$e+1));
}
@r = sort(@r);

my @open = ([0,0xffffffff]);

my $min = 0;
foreach (@r) {
	my ($f, $e) = split(/-/);
#	printf("%10d - %10d\n", $f, $e);
	if ($f <= $min) {
		if ($e > $min) {
			$min = $e;
#			printf("Min = %d\n",$min);
		}
	}
}
$part1 = $min;

# Merge ranges:
sub merge
{
	my @r = @_;
	my $r = shift @r;
	my ($f1,$e1) = split(/-/,$r);
	my @m = ();

	foreach (@r) {
		my ($f2,$e2) = split(/-/);
		if ($f2 > $e1) {
			push(@m, "$f1-$e1");
			($f1,$e1) = ($f2,$e2);
		}
		else { # merge!
			$e1 = $e2 if ($e2 > $e1);
		}
	}
	push(@m, "$f1-$e1");
	printf("%d ranges remain\n", scalar(@m));
	return @m;
}

my @m = merge(@r);
@m = sort @m;
@m = merge(@m);

my $open = 1 << 32;
foreach (@m) {
	my ($f,$e) = split(/-/);
	printf("%s\n",$_);
	$open -= $e-$f;
}

$part2 = $open;

printf(STDERR "Total time = %f\n", time - $start);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);

