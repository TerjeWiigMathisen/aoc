#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);
use List::PriorityQueue;

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

my %board = ();

my $FAV = 1352;
my ($TX,$TY) = (31,39);

sub wall
{
	my ($x,$y) = @_;
	return $board{"$x,$y"} if (defined($board{"$x,$y"}));
	if ($x < 0 || $y < 0) {
		return '#';
	}

	my $n = $x*$x + 3*$x + 2*$x*$y + $y + $y*$y + $FAV;
	my $bits = 0;
	while ($n) {
		$bits += $n & 1;
		$n >>= 1;
	}
	return $board{"$x,$y"} = ($bits & 1) ? "#" : " ";
}

#$FAV = 10;
#($TX,$TY) = (7,4);
for (my $y = 0; $y < 40; $y++) {
	for (my $x = 0; $x < 40; $x++) {
		printf("%s",$x == $TX && $y == $TY ? 'x' : wall($x,$y));
	}
	printf("\n");
}
printf("\n");

my ($x,$y) = (1,1);

my $pq = new List::PriorityQueue;
my $dist = abs($TX-$x)+abs($TY-$y);
$pq->insert("$x,$y,0", 0); #$dist);
while (my $key = $pq->pop()) {
	my ($x,$y, $steps) = split(/,/,$key);
	$b = wall($x,$y);
	if ($b ne ' ' && $b <= $steps) {
		next;
	}
	$board{"$x,$y"} = $steps++;
	if ($x ==$TX && $y == $TY) {
		printf("Found a solution in %d steps\n", $steps-1);
		$part1 = $steps-1;
		last;
	}
	foreach ([0,-1],[-1,0],[1,0],[0,1]) {
		my ($dx,$dy) = @{$_};
		my ($nx, $ny) = ($x+$dx,$y+$dy);
		next if (wall($nx,$ny) ne ' ');
		
		$dist = abs($TX-$nx)+abs($TY-$ny);
		$pq->insert("$nx,$ny,$steps", $steps); #$dist);
	}
}

for (my $y = 0; $y <= 50; $y++) {
	for (my $x = 0; $x <= 50; $x++) {
		$b = wall($x,$y);
		if ($b =~ /\d+/) {
			$b %= 10;
		}
		printf("%s", $b);
	}
	printf("\n");
}
printf("\n");

$part2 = 0;
foreach (keys %board) {
	$b = $board{$_};
	if ($b =~ /\d+/ && $b <= 50) {
		$part2++;
	}
}

printf(STDERR "Total time = %f\n", time - $start);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
