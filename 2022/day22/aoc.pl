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

foreach (<F>) {
	chomp;
	push(@inp,$_);
}
close(F);

my $dirs = pop(@inp);
die("Bad input") unless( pop(@inp) eq '');

my @inp3d = @inp;
my $dirs3d = $dirs;

my $maxlen = 0;
foreach (@inp) {
	if (length($_) > $maxlen) {$maxlen = length($_); }
}

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
	substr($inp[$y],$x,1) = $dc;
	for (my $i = 0; $i < $len; $i++) {
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
		substr($inp[$y],$x,1) = $dc;
	}
	substr($inp[$y],$x,1) = $dc;
	return ($x, $y);
}

my $dir = 0;
my $x = index($inp[0],'.');
my $y = 0;

sub dmp
{
	my ($head) = @_;
	printf("%s\n",$head);
	foreach (@inp) {
		printf("%s\n",$_);
	}
	printf("\n");
}

#dmp();

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
	else {
		printf("Final move!\n");
	}
#	dmp();
}
#dmp();
$part1 = 1000*($y+1) + 4*($x+1) + $dir;

printf("Part1: %s\n", $part1);
printf(STDERR "Total time = %f\n", time - $start);

$dirs = $dirs3d;
@inp = @inp3d;
$dir = 0;
$x = index($inp[0],'.');
$y = 0;
my @fold = ("001","234","0056");
my %wrap = (10 => 62, 11 => 41, 12 => 31, 13 => 21, 
		20 => 30, 21 => 53, 22 => 63, 23 => 11, 
		30 => 40, 31 => 50, 32 => 22, 33 => 10,
		40 => 61, 41 => 51, 42 => 32, 43 => 13,
		50 => 60, 51 => 23, 52 => 33, 53 => 43,
		60 => 12, 61 => 20, 62 => 52, 63 => 42);
my @pos = (2,0, 0,1, 1,1, 2,1, 2,2, 3,2);
my $cube = 4;
if ($fname eq 'input.txt') {
	@fold = ("012","03","45","6");
	%wrap = (10 => 20, 11 => 31, 12 => 40, 13 => 60, 
			20 => 52, 21 => 32, 22 => 12, 23 => 63, 
			30 => 23, 31 => 51, 32 => 41, 33 => 13,
			40 => 50, 41 => 61, 42 => 10, 43 => 30,
			50 => 22, 51 => 62, 52 => 42, 53 => 33,
			60 => 53, 61 => 21, 62 => 11, 63 => 43);
	@pos = (1,0, 2,0, 1,1, 0,2, 1,2, 0,3);
	$cube = 50;
}

my $dchar = '>v<^';
my @dxdy = (1,0, 0,1, -1,0, 0,-1);
sub move3d
{
	my ($x, $y, $dir, $len) = @_;
	my ($px,$py,$pd) = ($x,$y,$dir);
	substr($inp[$y],$x,1) = substr($dchar,$dir,1);
	for (my $i = 0; $i < $len; $i++) {
		my ($mapx, $mapy) = (int($x / $cube), int($y/ $cube));
		my @lim = ($mapx*$cube,$mapy*$cube, ($mapx+1)*$cube,($mapy+1)*$cube);
		my ($dx, $dy) = ($dxdy[$dir*2],$dxdy[$dir*2+1]);
		$x = ($x+$dx);
		$y = ($y+$dy);
		if ($x < $lim[0] || $y < $lim[1] || $x >= $lim[2] || $y >= $lim[3] ) { # Wrapping!
			my $map = substr($fold[$mapy], $mapx,1);
			my ($m,$d) = split(//,$wrap{$map.$dir});
			my $ddir = $dir.$d;
			if ($ddir eq "00") {
				($x,$y) = ($x,$y);
			}
			elsif ($ddir eq "01") {
				($x,$y) = (-$y-1,0);
			}
			elsif ($ddir eq "02") {
				($x,$y) = (-1,-$y-1);
			}
			elsif ($ddir eq "03") {
				($x,$y) = ($y,-1);
			}
			if ($ddir eq "10") {
				($x,$y) = (0,-$x-1);
			}
			elsif ($ddir eq "11") {
				($x,$y) = ($x,$y);
			}
			elsif ($ddir eq "12") {
				($x,$y) = (-1,$x);
			}
			elsif ($ddir eq "13") {
				($x,$y) = ($x-1,-1);
			}
			if ($ddir eq "20") {
				($x,$y) = (0,-$y-1);
			}
			elsif ($ddir eq "21") {
				($x,$y) = ($y,0);
			}
			elsif ($ddir eq "22") {
				($x,$y) = ($x,$y);
			}
			elsif ($ddir eq "23") {
				($x,$y) = (-$y-1,-1);
			}
			if ($ddir eq "30") {
				($x,$y) = (0,$x);
			}
			elsif ($ddir eq "31") {
				($x,$y) = (-$x-1,0);
			}
			elsif ($ddir eq "32") {
				($x,$y) = (-$y-1,$x);
			}
			elsif ($ddir eq "33") {
				($x,$y) = ($x,$y);
			}
			($x,$y) = ($x % $cube, $y % $cube);
			($x,$y) = ($x + $pos[$m*2-2]*$cube, $y + $pos[$m*2-1]*$cube);
			$dir = $d;
		}
		if (substr($inp[$y],$x,1) eq '#') {
			($x,$y,$dir) = ($px,$py,$pd); # Stop on previous tile!
			last;
		}
		die("Hit space!") if (substr($inp[$y],$x,1) eq ' ');
		($px,$py,$pd) = ($x,$y,$dir);
		substr($inp[$y],$x,1) = substr($dchar,$dir,1);
	}
	substr($inp[$y],$x,1) = substr($dchar,$dir,1);
#	dmp("($x,$y,$dir)");
	return ($x, $y, $dir);
}

my $map = 1;
#dmp('start');
while ($dirs =~ /^(\d+)([LR]?)/) {
	my ($len, $d) = ($1,$2);
	$dirs = substr($dirs,length($len)+length($d));
	($x,$y,$dir) = move3d($x,$y,$dir,$len);
	if ($d) {
		#printf("Turn $d\n");
	}
	if ($d eq 'R') {
		$dir = ($dir+1) % 4;
	}
	elsif ($d eq 'L') {
		$dir = ($dir-1) % 4;
	}
	else {
		printf("Final move!\n");
	}
}
$part2 = 1000*($y+1) + 4*($x+1) + $dir;

printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);

