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

my (%wall, %up, %lt, %rt, %dn);
my $y = 0;
foreach (@inp) {
	my @l = split(//);
	my $x = 0;
	foreach (@l) {
		if ($_ eq '#') {
			$wall{"$x,$y"} = 1;
		}
		elsif ($_ eq 'v') {
			$dn{"$x,$y"} = 1;
		}
		elsif ($_ eq '>') {
			$rt{"$x,$y"} = 1;
		}
		elsif ($_ eq '<') {
			$lt{"$x,$y"} = 1;
		}
		elsif ($_ eq '^') {
			$up{"$x,$y"} = 1;
		}
		$x++;
	}
	$y++;
}

my ($SX,$SY) = (index($inp[0],'.'),0);
my ($TX,$TY) = (index($inp[-1],'.'),scalar(@inp)-1);
my ($SIZEX, $SIZEY) = (length($inp[0]),scalar(@inp));
my ($RT,$BT) = ($SIZEX-2,$SIZEY-2);

my %mapcache;
sub mapn
{
	my ($n) = @_;
	return $mapcache{$n} if (defined($mapcache{$n}));
	my @map = ('#' x $SIZEX, ('#'.('.' x $RT).'#') x $BT, '#' x $SIZEX); # Full '#' boundary, filled with '.'
	substr($map[$SY],$SX,1) = '.';
	substr($map[$TY],$TX,1) = '.';
	
	foreach (keys %up) {
		my ($x,$y) = split(/,/);
		$y = ($y-1-$n) % $BT + 1;
		substr($map[$y],$x,1) = '^';
	}
	foreach (keys %dn) {
		my ($x,$y) = split(/,/);
		$y = ($y-1+$n) % $BT + 1;
		substr($map[$y],$x,1) = (substr($map[$y],$x,1) eq '.')? 'v' : '2';
	}
	foreach (keys %lt) {
		my ($x,$y) = split(/,/);
		$x = ($x-1-$n) % $RT + 1;
		substr($map[$y],$x,1) = (substr($map[$y],$x,1) eq '.')? '<' : 
			(substr($map[$y],$x,1) ge '2' && substr($map[$y],$x,1) le '9') ? substr($map[$y],$x,1)+1 : '2';
	}
	foreach (keys %rt) {
		my ($x,$y) = split(/,/);
		$x = ($x-1+$n) % $RT + 1;
		substr($map[$y],$x,1) = (substr($map[$y],$x,1) eq '.')? '>' : 
			(substr($map[$y],$x,1) ge '2' && substr($map[$y],$x,1) le '9') ? substr($map[$y],$x,1)+1 : '2';
	}
	
	my $map = join("\n",@map);
	$mapcache{$n} = $map;
	return $map;
}

sub dmp
{
	my ($n,$x,$y) = @_;
	printf("%s\n", $n);
	my $map = mapn($n);
	my @map = split(/\n/,$map);
	substr($map[$y],$x,1) = 'E';
	printf("%s\n\n",join("\n",@map));
}

sub search
{
	my ($t,$SX,$SY,$TX,$TY) = @_;
	my %seen = ();
	my $pq = new List::PriorityQueue;
	my ($best, $bestmoves) = (1e10,'');
	$pq->insert(join(',',$t,$SX,$SY,''), 0);
	while (my $item = $pq->pop()) {
		my ($t,$x,$y,$moves) = split(/,/, $item);
		next if ($y < 0);
		next if ($y >= $SIZEY);
		
		my $manh = abs($TX-$x) + abs($TY-$y);
		next if ($t+$manh >= $best);

		my $map = mapn($t);
		my @map = split(/\n/,$map);
		next unless (substr($map[$y],$x,1) eq '.');

		if ($x == $TX && $y == $TY) {
			($best, $bestmoves) = ($t, $moves);
			printf("Found %d moves\n",$t);
			next;
		}
		next if ($seen{"$t,$x,$y"}++);

		# Try down:
		$pq->insert(join(',',$t+1,$x,$y+1,$moves.'v'),$t*0.001 + abs($TX-$x) + abs($TY-$y-1));
		# Try left:
		$pq->insert(join(',',$t+1,$x-1,$y,$moves.'<'),$t*0.001 + abs($TX-$x+1) + abs($TY-$y));
		# Try right:
		$pq->insert(join(',',$t+1,$x+1,$y,$moves.'>'),$t*0.001 + abs($TX-$x-1) + abs($TY-$y));
		# Try up:
		$pq->insert(join(',',$t+1,$x,$y-1,$moves.'^'),$t*0.001 + abs($TX-$x) + abs($TY-$y+1));
		# Stand still:
		$pq->insert(join(',',$t+1,$x,$y,$moves.'0'),$t*0.001 + $manh);
	}
	return ($best, $bestmoves);
}

my $moves;
($part1, $moves) = search(0,$SX,$SY,$TX,$TY-1);
$part1++; $moves .= 'v';
printf("%d path: %s\n", $part1, $moves);

printf("Part1: %s\n", $part1);
printf(STDERR "Total time = %f\n", time - $start);

($part2,$moves) = search($part1,$TX,$TY,$SX,$SY+1);
$part2++; $moves .= '^';
printf("%d return path: %s\n", $part2, $moves);
printf(STDERR "Total time = %f\n", time - $start);

($part2, $moves) = search($part2,$SX,$SY,$TX,$TY);
$part2++; $moves .= 'v';
printf("%d final path: %s\n", $part2, $moves);

printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);

