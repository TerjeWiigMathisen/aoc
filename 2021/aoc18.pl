#!perl -w

use strict;
use Time::HiRes qw (time);
use List::PriorityQueue;
use warnings;
use English;
#no warnings 'recursion';

my $DEBUG = 1;

my $start = time;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

sub separate
{
	my ($expr) = @_;
	return (split//, $expr);
}

sub separate1
{
	my ($expr) = @_;
	my @e = ();
	for (my $p = 0; $p < length($expr); $p++) {
		my $c = substr($expr,$p,1);
		if ($c eq '[') {
			push(@e, $c);
		}
		elsif ($c ge '0' && $c le '9') {
			my $num = $c;
			while (substr($expr,$p+1,1) =~ /\d/) {
				$num .= substr($expr,$p+1,1);
				$p++;
			}
			push(@e,$num);
		}
		elsif ($c eq ',') {
			push(@e, $c);
		}
		elsif ($c eq ']') {
			push(@e, $c);
		}
	}
	return @e;
}

sub explode
{
	my @e = @_;
	my ($nesting, $i) = (0,0);
	foreach (@e) {
		my $e = $_;
		if ($e eq '[') {
			$nesting++;
		}
		elsif ($e eq ']') {
			$nesting--;
		}
		elsif ($e eq ',') {
			# do nothing
		}
		elsif ($e =~ /^\d+$/) {
			# was this preceeded by '[' and followed by comma and another number and ']', making a pair?
			if ($e[$i-1] eq '[' && $e[$i+1] eq ',' && $e[$i+2] =~ /^\d+$/ && $e[$i+3] eq ']') {
				if ($nesting >= 5) {
					# Explode this pair, adding it left/right:
					my ($left, $right) = ($e, $e[$i+2]);
					# Seek left, add if found
					for (my $l = $i-2; $l >= 0; $l--) {
						if ($e[$l] =~ /^\d+$/) {
							$e[$l] += $left;
							last;
						}
					}
					# Seek right, add if found
					for (my $r = $i+4; $r < scalar(@e); $r++) {
						if ($e[$r] =~ /^\d+$/) {
							$e[$r] += $right;
							last;
						}
					}
					# Replace the original 5-token pair with a zero
					splice(@e,$i-1,5,"0");
					return (1, @e);
				}
			}
		}
		$i++;
	}
	return (0,@e);
}

sub splithigh
{
	my @e = @_;
	for (my $i = 0; $i < scalar(@e); $i++) {
		my $e = $e[$i];
		if ($e =~ /\d/ && $e >= 10) {
			my $l = $e >> 1;
			my $r = $e - $l;
			# Replace large number with new pair:
			splice(@e, $i,1,'[',$l,',',$r,']');
			return (1,@e);
		}
	}
	return (0,@e);
}

sub magnitude
{
	my @e = @_;
	my $e = join("", @e);
	while ($e =~/\[(\d+),(\d+)\]/) {
		$e = $PREMATCH.sprintf("%d",$1*3+$2*2).$POSTMATCH;
	}
	return $e;
}


sub add
{
	my ($a, $b) = @_;
	return ('[',@{$a},',',@{$b},']');
}

sub reduce
{
	my @r = @_;
	my $r;
	do {
		($r, @r) = explode(@r);
		($r, @r) = splithigh(@r) unless ($r);
	} while ($r);
	return @r;
}

sub dmp
{
	my ($s,@e) = @_;
	printf(STDERR "%s: %s\n", $s, join("",@e)) if ($DEBUG);
}

my @r = separate(shift @inp);
foreach (@inp) {
	my @e = separate($_);
	@r = reduce(add(\@r, \@e));
}
my $part1 = magnitude(@r);

my $part2 = 0;
for (my $s = 0; $s < scalar(@inp); $s++) {
	my @s = separate($inp[$s]);
	for (my $f = 0; $f < scalar(@inp); $f++) {
		next if ($f == $s);
	
		my @f = separate($inp[$f]);
		# add the pair, then reduce and take the magnitude
		my $m = magnitude(reduce(add(\@f,\@s)));
		$part2 = $m if ($m > $part2);
	}
}

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %d\n", $part1);
printf("Part2: %d\n", $part2);
