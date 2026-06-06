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

my (%wall, %up, %lt, %rt, %dn,%wind);
my %wdir = ('>' => 0, 'v' => 2, '<' => 4, '^' => 6);
my @ddir = (1,0, 0,1, -1,0, 0,-1);
my $y = 0;
foreach (@inp) {
	my @l = split(//);
	my $x = 0;
	foreach (@l) {
		my $k = "$x,$y";
		if ($_ eq '#') {
			$wall{$k} = 1;
		}
		elsif ($_ ne '.') { # Wind here!
			if ($_ eq 'v') {
				$dn{$k} = 1;
			}
			elsif ($_ eq '>') {
				$rt{$k} = 1;
			}
			elsif ($_ eq '<') {
				$lt{$k} = 1;
			}
			elsif ($_ eq '^') {
				$up{$k} = 1;
			}
			$wind{$k} = $_;
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
my %nextc = ('>' => '2', '<' => '2', '^' => '2', 'v' => '2', '2' => '3', '3' => '4'); 
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
		substr($map[$y],$x,1) = (substr($map[$y],$x,1) eq '.')? 'v' : $nextc{substr($map[$y],$x,1)};
	}
	foreach (keys %lt) {
		my ($x,$y) = split(/,/);
		$x = ($x-1-$n) % $RT + 1;
		substr($map[$y],$x,1) = (substr($map[$y],$x,1) eq '.')? '<' : $nextc{substr($map[$y],$x,1)};
	}
	foreach (keys %rt) {
		my ($x,$y) = split(/,/);
		$x = ($x-1+$n) % $RT + 1;
		substr($map[$y],$x,1) = (substr($map[$y],$x,1) eq '.')? '>' : $nextc{substr($map[$y],$x,1)};
	}
	
	my $map = join("\n",@map);
	$mapcache{$n} = $map;
	return $map;
}

sub mapnw
{
	my ($n) = @_;
	return $mapcache{$n} if (defined($mapcache{$n}));
	my @map = ('#' x $SIZEX, ('#'.('.' x $RT).'#') x $BT, '#' x $SIZEX); # Full '#' boundary, filled with '.'
	substr($map[$SY],$SX,1) = '.';
	substr($map[$TY],$TX,1) = '.';
	
	foreach (keys %wind) {
		my ($x,$y) = split(/,/);
		my $wc = $wind{$_};
		my $w = $wdir{$wc};
		my ($dx,$dy) = ($ddir[$w],$ddir[$w+1]);
		$x = ($x-1-$n*$dx) % $RT + 1;
		$y = ($y-1-$n*$dy) % $BT + 1;
		substr($map[$y],$x,1) = (substr($map[$y],$x,1) eq '.')? $wc : $nextc{substr($map[$y],$x,1)};
	}

	my $map = join("\n",@map);
	$mapcache{$n} = $map;
	return $map;
}

sub round
{
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

for (my $r = 0; $r < 10; $r++) {
#	dmp($r,$SX,$SY);
}

sub search
{
	my ($t,$SX,$SY,$TX,$TY) = @_;
	my %seen = ();
	my $pq = new List::PriorityQueue;
	$pq->insert(join(',',$t,$SX,$SY,''), 0);
	while (my $item = $pq->pop()) {
		my ($t,$x,$y,$moves) = split(/,/, $item);
		next if ($y < 0);
		next if ($y >= $SIZEY);
	#	dmp($t,$x,$y);
		if ($x == $TX && $y == $TY) {
			return ($t, $moves);
		}
		next if ($seen{"$t,$x,$y"}++);
		my $map = mapnw($t);
		my @map = split(/\n/,$map);
		next unless (substr($map[$y],$x,1) eq '.');
		# Try down:
		$pq->insert(join(',',$t+1,$x,$y+1,$moves.'v'),$t*1000 + abs($TX-$x) + abs($TY-($y+1)));
		# Try left:
		$pq->insert(join(',',$t+1,$x-1,$y,$moves.'<'),$t*1000 + abs($TX-($x-1)) + abs($TY-$y));
		# Try right:
		$pq->insert(join(',',$t+1,$x+1,$y,$moves.'>'),$t*1000 + abs($TX-($x+1)) + abs($TY-$y));
		# Try up:
		$pq->insert(join(',',$t+1,$x,$y-1,$moves.'^'),$t*1000 + abs($TX-$x) + abs($TY-($y-1)));
		# Stand still:
		$pq->insert(join(',',$t+1,$x,$y,$moves.'0'),$t*1000 + abs($TX-$x) + abs($TY-$y));
	}
	die("No solution found!");
}

my $moves;
($part1, $moves) = search(0,$SX,$SY,$TX,$TY);
printf("%d path: %s\n", $part1, $moves);

printf("Part1: %s\n", $part1);
printf(STDERR "Total time = %f\n", time - $start);

($part2,$moves) = search($part1,$TX,$TY,$SX,$SY);
printf("%d return path: %s\n", $part2, $moves);
printf(STDERR "Total time = %f\n", time - $start);

($part2, $moves) = search($part2,$SX,$SY,$TX,$TY);
printf("%d final path: %s\n", $part2, $moves);

printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);

