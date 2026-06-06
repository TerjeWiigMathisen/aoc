#!perl -w

use strict;
use Time::HiRes qw (time);
use List::PriorityQueue;
use warnings;
#no warnings 'recursion';

my $start = time;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

my $rule = shift (@inp);

$rule =~ tr/.#/01/;
my @rule = split(//,$rule);
shift @inp;

my %board = ();
for (my $y = 0; $y < scalar(@inp); $y++) {
	$inp[$y] =~tr/.#/01/;
	my $x = 0;
	foreach (split(//,$inp[$y])) {
		$board{"$x,$y"}++ if ($_);
		$x++;
	}
}

my $border = ' ';
my ($x0,$y0,$x1,$y1) = (0,0,length($inp[0])-1,scalar(@inp)-1);

sub cell
{
	my ($x, $y, $default) = @_;
	my $key = "$x,$y";
	my $r = $default;
	$r ^= 1 if (defined($board{$key}));
	return $r;
}

sub g3x3
{
	my ($x, $y, $default) = @_;
	my $sample = 0;
	for (my $cy = $y-1; $cy <= $y+1; $cy++) {
		for (my $cx = $x-1; $cx <= $x+1; $cx++) {
			$sample = $sample*2+cell($cx,$cy, $default);
		}
	}
	return $sample;
}

sub gen2
{
	my ($x0,$y0,$x1,$y1, $default) = @_;
	my %b = ();
	my $d = $rule[0] ? $default ^ 1 : $default;
	for (my $y = $y0; $y <= $y1; $y++) {
		for (my $x = $x0; $x <= $x1; $x++) {
		
			my $sample = g3x3($x,$y, $default);
			my $out = $rule[$sample];
			$b{"$x,$y"}++ if ($out == $d);
		}
	}
	return %b;
}

sub dmp
{
	for (my $y = $y0; $y <= $y1; $y++) {
		my $line = "";
		for (my $x = $x0; $x <= $x1; $x++) {
			$line .= cell($x,$y) ? '#' : ' ';
		}
		printf(STDERR "%s\n",$line);
	}
	printf(STDERR "\n");
}

dmp();

my $default = 0;
for (my $iter = 0; $iter < 2; $iter++) {
	($x0,$y0,$x1,$y1) = ($x0-1,$y0-1,$x1+1,$y1+1);
	%board = gen2($x0,$y0,$x1,$y1,$default);
	$default ^= 1;
}

dmp();

my $part1 = scalar(keys %board);

for (my $iter = 2; $iter < 50; $iter++) {
	($x0,$y0,$x1,$y1) = ($x0-1,$y0-1,$x1+1,$y1+1);
	%board = gen2($x0,$y0,$x1,$y1,$default);
	$default ^= 1;
}
dmp();

my $part2 = scalar(keys %board);

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %d\n", $part1);
printf("Part2: %d\n", $part2);
