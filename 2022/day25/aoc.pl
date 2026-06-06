#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);
#use List::Util qw (reduce);
use List::PriorityQueue;
#use bigint;

#use JSON::Parse;
#no warnings 'recursion';

my $DEBUG = 0;

my $fname = shift;
$fname = 'input.txt' unless (defined($fname));
open(F ,'<',$fname);

my $BUFFER = shift;
$BUFFER = 100 unless (defined($BUFFER));

my $start = time;

my $part1 = 0;
my $part2 = 0;

my @inp;
foreach (<F>) {
	chomp;
	push(@inp,$_);
}
close(F);

my %base5 = ('1' => 1, '0' => 0, '2' => 2, '-' => -1, '=' => -2);

sub convert
{
	my ($s) = @_;
	my @s = split(//,$s);
	#my $b = 1;
	my $n = 0;
	foreach (@s) {
		$n = $n*5 + $base5{$_};
		
	}
	return $n;
}

my @b5dig = ('=','-','0','1','2');

sub tobase5
{
	my ($n) = @_;
	my $b = 1;
	my @pow5 = (1);
	do {
		$b *= 5;
		push(@pow5,$b);
	} while ($b*5 < abs($n)*2);
	my $s = '';
	$b = scalar(@pow5)-1;
	while (abs($n) > 2) {
		my $f = ($n / $pow5[$b]);
		$f = sprintf("%1.0f", $f); # Round to nearest integer, handles positive and negative!
		die("Wrong dig $f ($n, $s)") unless ($f >= -2 && $f <= 2);
		$s .= $b5dig[$f+2];
		$n -= $f*$pow5[$b];
		$b--;
	}
	$s .= $b5dig[$n+2];
	return $s;
}

sub tobase5short
{
	my ($n) = @_;
	my $b = 1;
	while ($b < abs($n)*2) { $b *= 5; }
	my $s = '';
	$b /= 5;
	do {
		my $f = ($n / $b);
		$f = sprintf("%1.0f", $f); # Round to nearest integer, handles positive and negative!
		die("Wrong dig $f ($n, $s)") unless ($f >= -2 && $f <= 2);
		$s .= $b5dig[$f+2];
		$n -= $f*$b;
		$b /= 5;
	} while ($n);
	return $s;
}

sub tobase5bottom
{
	my ($n) = @_;
	my $s = '';
	do {
		my $d = $n % 5;
		if ($d > 2) {
			$d -= 5;
		}
		$n = ($n-$d)/5;
		$s = $b5dig[$d+2].$s;
	} while ($n);
	return $s;
}

my @DIGITS;	# Array of the legal digit chars
my %VALUE;	# Hash converting digits to numeric value
my $BASE;	# Number base to use

initbase('012=-',2);

sub initbase
{
	my ($digs, $bias) = @_;
	@DIGITS = split(//,$digs);
	$BASE = length($digs);
	for (my $i = 0; $i < $BASE-$bias; $i++) {
		$VALUE{$DIGITS[$i]} = $i;		# Positive digits 0...
	}
	for (my $i = -$bias; $i < 0; $i++) {
		$VALUE{$DIGITS[$i]} = $i;		# Negative, from -bias to -1
	}
}

sub frombase
{
	my ($s) = @_;
	my $n = 0;
	foreach (split(//,$s)) {
		$n = $n*$BASE + $VALUE{$_};
	}
	return $n;
}

sub tobase
{
	my ($n) = @_;
	my $s = '';
	do {
		my $r = $n % $BASE; # Always positive remainder for positive $BASE
		my $d = $DIGITS[$r];
		$n = ($n-$VALUE{$d}) / $BASE; # Always exact division!
		$s = $d.$s;			# Prepend the current digit char
	} while ($n);
	return $s;
}

foreach (@inp) {
	my $n = frombase($_); #convert($_);
#	printf("%20s -> %20d -> %20s\n", $_, $n, tobase($n));
	$part1 += $n;
}

#printf("Part1: %s -> %s\n", $part1, tobase5bottom($part1));
printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s -> %s\n", $part1, tobase($part1));

#printf("Part2: %s\n", $part2);
#printf(STDERR "Total time = %f\n", time - $start);

