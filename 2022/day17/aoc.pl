#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);
#use List::Util qw (reduce);

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

my $color = 'A';

my $inp = '';
my @wind;
foreach (<F>) {
	chomp;
	push(@wind, split(//));
}
close(F);

my $pieces = qq(####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##);
$pieces =~s/\./ /g;
my @pieces = split(/\n\n/, $pieces);

my @board = '+-------+';

my $floor = 0;

my @drop;
my $period = scalar(@wind)*scalar(@pieces);

sub simulate
{
	my ($rounds) = @_;
	@board = '+-------+';
	my $top = 0; # scalar(@board)
	my $piece = 0;
	my $wind = 0;
	my $rocks = 0;
	my $collapsed = 0;
	my $period_limit = $period;
	my $prev_top = $top;
	my $ret2022;
	my $coll = 1;
	while (1) {
		my $p = $pieces[$piece++];
		if ($wind >= scalar(@wind)) { 
			printf("Wind wrapping around after %d rocks, on piece %d\n", $rocks, $piece);
		}
		$piece = 0 if ($piece >= scalar(@pieces));

		my @p = reverse split(/\n/, $p);
		my $pl = scalar(@p);
		
		my $px = 3;
		my $py = $top + 4;
		while (scalar(@board) <= $py+$pl) {
			push(@board,'|       |');
		}
		# Start falling
		while (1) {
			if ($wind >= scalar(@wind)) { 
				if ($wind >= scalar(@wind)) { 
					printf("Wind wrapping around inside %d rocks, on piece %d\n", $rocks, $piece);
				}
				$wind = 0;
			}
			my $w = $wind[$wind++];
			#printf("Move %s\n", $w);
			my $dx = $w eq '<' ? -1 : 1;
			# Move sideways?
			if (!collide($px+$dx, $py, @p)) { 
				$px += $dx;
			}
			# Move down?
			if (collide($px, $py-1, @p)) { 
				last;
			}
			$py--;
		}
		fix($px,$py,@p);
		
		$py += $pl-1;
		if ($py > $top) {
			$top = $py;
		}

		#show();
		$rocks++;

		if ($coll > 0) {
			for (my $i = $py; $i < $py+scalar(@p); $i++) {
				if (index($board[$i],' ') < 0) { # No space, so collapse!
					#show();
					splice(@board,1,$i);
					#show();
					$top -= $i;
					$py -= $i;
					$collapsed += $i;
#					$coll--;
					#printf(STDERR "%d drop %d %d %d %d\n", $rocks, $i, $top, $top+$collapsed, scalar(@p));
					#push(@drop, $rocks, $i, $top, $top+$collapsed);
					last;
				}
			}
		}

		if ($rocks >= $period_limit) {
			my $curr_top = $top+$collapsed;
			
			printf("%3d growth %d rocks %d top\n", $curr_top - $prev_top,$rocks, $top+$collapsed);
			$period_limit += $period;
			$prev_top = $curr_top;
		}
		if ($rocks == $rounds) {
	#	if ($rocks >= 22) {
			$ret2022 = $top + $collapsed;
		}
		if ($rocks >= $rounds) {
			return $ret2022;
		}
	}
}

$part1 = simulate(2022);
$part2 = simulate(50151+1464);
simulate(100000);
printf("Board remaining: %d\n", scalar(@board));

my %drops = ();
my ($rocks0, $drop0, $top0, $tot0) = splice(@drop,0,4);
while (@drop) {
	my ($rocks, $drop, $top, $tot) = splice(@drop,0,4);
	printf("drocks %d, ddrop %d, dtot %d\n", $rocks-$rocks0, $drop-$drop0, $tot-$tot0);
	$rocks0 = $rocks;
	$drop0 = $drop;
	$tot0 = $tot;
}

my $ROUNDS = 1000000000000;
my $start_per = 50151;
my $start_top = 79826;
my $drop1414 = 1745;
my $gain1414 = 2778;

my $periods = int(($ROUNDS-$start_per)/$drop1414);
my $remainder = $ROUNDS - $start_per - $periods * $drop1414;

my $sum = $start_top + $periods * $gain1414;
$part2 += $periods * $gain1414;
printf("Sum of full periods = %d, remaining rocks = %d\n", $sum, $remainder);

sub collide
{
	my ($px, $py, @p) = @_;
	#printf("%s at (%d,%d)\n", join("\n",reverse @p), $px, $py);
	for (my $y = 0; $y < scalar(@p); $y++) {
		for (my $x = 0; $x < length($p[$y]); $x++) {
			if (substr($p[$y],$x,1) ne ' ' && substr($board[$py+$y],$px+$x,1) ne ' ') {
				return 1; # Collide here!
			}
		}
	}
	return 0;
}

sub fix
{
	my ($px, $py, @p) = @_;
	for (my $y = 0; $y < scalar(@p); $y++) {
		for (my $x = 0; $x < length($p[$y]); $x++) {
			if (substr($p[$y],$x,1) eq '#') {
				substr($board[$py+$y],$px+$x,1) = $color;
			}
		}
	}
	$color = ($color eq  'Z') ? 'A' : chr(ord($color)+1);
}

sub show
{
	for (my $y = scalar(@board)-1; $y >= 0; $y--) {
		printf("%s\n", $board[$y]);
	}
	printf("%d lines\n", scalar(@board));
}

#show();

printf(STDERR "Part1 time = %f\n", time - $start);
printf("Part1: %s\n", $part1);


printf(STDERR "Total time = %f\n", time - $start);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
