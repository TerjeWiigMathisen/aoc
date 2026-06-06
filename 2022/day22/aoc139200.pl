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

my $start = time;

my $part1 = 0;
my $part2 = 0;

my @inp = ();
if ($fname eq 't.txt') {
}

my $maxlen = 0;
foreach (<F>) {
	chomp;
	push(@inp,$_);
	if (length($_) > $maxlen) {$maxlen = length($_); }
}
close(F);

my $dirs = pop(@inp);
die("Bad input") unless( pop(@inp) eq '');

foreach (@inp) {
	while(length($_) < $maxlen) {
		$_.= ' ';
	}
}

my $LX = length($inp[0]);
my $LY = scalar(@inp);

sub move
{
	my ($x, $y, $dir, $len) = @_;
	my ($dx, $dy);
	my $dc;
	if ($dir == 3) { # up
		($dx,$dy) = (0,-1);
		$dc = '^';
	}
	elsif ($dir == 0) { # right
		($dx,$dy) = (1,0);
		$dc = '>';
	}
	elsif ($dir == 1) { # down
		($dx,$dy) = (0,1);
		$dc = 'v';
	}
	elsif ($dir == 2) { # left
		($dx,$dy) = (-1,0);
		$dc = '<';
	}
	else {
		die("Bad dir: $dir");
	}
	my ($px,$py) = ($x,$y);
	for (my $i = 0; $i < $len; $i++) {
		substr($inp[$y],$x,1) = $dc;
		$x = ($x+$dx+$LX) % $LX;
		$y = ($y+$dy+$LY) % $LY;
		if (substr($inp[$y],$x,1) eq '#') {
			($x,$y) = ($px,$py); # Stop on previous tile!
			last;
		}
		if (substr($inp[$y],$x,1) eq ' ') {
			$i--; # Free move!
			next;
		}
		($px,$py) = ($x,$y);
	}
	substr($inp[$y],$x,1) = $dc;
	return ($x, $y);
}

my $dir = 0;
my $x = index($inp[0],'.');
my $y = 0;

sub dmp
{
	printf("%s\n\n",join("\n",@inp));
}

dmp();

while ($dirs =~ /^(\d+)([LR]?)/) {
	my ($len, $d) = ($1,$2);
	$dirs = substr($dirs,length($len)+length($d));
	($x,$y) = move($x,$y,$dir, $len);
	if ($d eq 'R') {
		$dir = ($dir+1) % 4;
	}
	elsif ($d eq 'L') {
		$dir = ($dir+3) % 4;
	}
	#dmp();
}
$part1 = 1000*($y+1) + 4*($x+1) + $dir;

printf("Part1: %s\n", $part1);
printf(STDERR "Total time = %f\n", time - $start);

printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);

