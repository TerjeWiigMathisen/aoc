#!perl -w
use strict;
use Time::HiRes qw(time);
#use List::PriorityQueue;
use Math::Polygon::Calc;

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @inp = (<>); chomp(@inp);
my @used = (0) x scalar(@inp);

$part1 = find_qe(3, @inp);

sub find
{
	my ($w, @w) = @_;
	for ($i = scalar(@w)-1; $i >= 0; $i--) {
		my @t = @w;
		my $last = splice(@t,$i,1);
		if ($last <= $w) {
			if ($last == $w) { # Found a solution!
				push(@w,$w);
	

sub find_qe
{
	my ($parts, @weights) = @_;
	my $sum = 0;
	foreach (@inp) {$sum += $_;}

	my $target = $sum / $parts;
	my @compartment = ();
	push(@compartment, pop(@weights));

