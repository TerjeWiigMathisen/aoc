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

my %elf;
my $y = 0;
foreach (@inp) {
	my @l = split(//);
	my $x = 0;
	foreach (@l) {
		if ($_ eq '#') {
			$elf{"$x,$y"} = 1;
		}
		$x++;
	}
	$y++;
}
my %trymove;
my @dirs = (-1,-1, 0,-1, 1,-1,  # N
            -1, 1, 0, 1, 1, 1,  # S
			-1,-1,-1, 0,-1, 1,	# W
			 1,-1, 1, 0, 1, 1); # E
my $dir = 0;

sub free
{
	my ($x,$y) = @_;
	my $k = "$x,$y";
	return ($elf{$k}) ? 0 : 1;
}

sub round
{
	# First half
	my @elves = ();
	my @stay = ();
	%trymove = ();
	foreach (keys %elf) {
		my ($x,$y) = split(/,/);
		if (free($x-1,$y-1) && free($x,$y-1) && free($x+1,$y-1) &&
		    free($x-1,$y)                    && free($x+1,$y) &&
			free($x-1,$y+1) && free($x,$y+1) && free($x+1,$y+1)) {
			push(@stay, $x,$y);
			next;
		}
		push(@elves,$x,$y);
		my $dd = $dir;
		my $ok = 0;
		for (my $d = 0; $d < 4; $d++) {
			if (free($x+$dirs[$dd], $y+$dirs[$dd+1]) && 
				free($x+$dirs[$dd+2], $y+$dirs[$dd+3]) &&
				free($x+$dirs[$dd+4], $y+$dirs[$dd+5])) {
				my $k = sprintf("%d,%d",$x+$dirs[$dd+2], $y+$dirs[$dd+3]);
				$trymove{$k}++;
				$ok++;
				last;
			}
			$dd += 6; if ($dd >= scalar(@dirs)) { $dd = 0; }
		}
	}
	return 0 if (scalar(@elves) == 0);
	# second half
	while (my ($x,$y) = splice(@elves,0,2)) {
		my $dd = $dir;
		my $ok = 0;
		for (my $d = 0; $d < 4; $d++) {
			if (
				free($x+$dirs[$dd], $y+$dirs[$dd+1]) && 
				free($x+$dirs[$dd+2], $y+$dirs[$dd+3]) &&
				free($x+$dirs[$dd+4], $y+$dirs[$dd+5])) {
				my $k = sprintf("%d,%d",$x+$dirs[$dd+2], $y+$dirs[$dd+3]);
				if (defined($trymove{$k}) && $trymove{$k} == 1) {
					push(@stay,split(/,/,$k));
				}
				else {
					push(@stay,$x,$y);
				}
				$ok = 1;
				last;
			}
			$dd += 6; if ($dd >= scalar(@dirs)) { $dd = 0; }
		}
		push(@stay,$x,$y) unless ($ok);
	}
	%elf = ();
	while (my ($x,$y) = splice(@stay,0,2)) {
		$elf{"$x,$y"}++;
	}
	$dir += 6;
	if ($dir >= scalar(@dirs)) { $dir = 0; }
	return 1;
}

sub smallest
{
	my ($minx,$miny,$maxx,$maxy) = (0,0,0,0);
	foreach (keys %elf) {
		my ($x,$y) = split(/,/);
		$minx = $x if ($x < $minx);
		$miny = $y if ($y < $miny);
		$maxx = $x if ($x > $maxx);
		$maxy = $y if ($y > $maxy);
	}
	return ($minx,$miny,$maxx,$maxy);
}

sub dmp
{
	my ($minx,$miny,$maxx,$maxy) = smallest();
	printf("(%d,%d) - (%d,%d)\n", $minx,$miny,$maxx,$maxy);
	for (my $y = $miny; $y <= $maxy; $y++) {
		for (my $x = $minx; $x <= $maxx; $x++) {
			printf("%s",free($x,$y) ? "." : "#");
		}
		printf("\n");
	}
	printf("\n");
}

printf("Found %d elves\n", scalar(keys %elf));
#dmp();
my $rnd = 0;
while (1) {
	$rnd++;
	my $moves = round();
	#printf("== End of round %d == \r",$rnd); 
	#dmp();
	last unless ($moves);
	if ($rnd == 10) {
		my ($x0,$y0,$x1,$y1) = smallest();
		$part1 = ($x1-$x0+1)*($y1-$y0+1) - scalar(keys %elf);
		printf("Part1: %s\n", $part1);
		printf(STDERR "Total time = %f\n", time - $start);
	}
}

$part2 = $rnd;

printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);

