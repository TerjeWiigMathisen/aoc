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
	my @map = ('#' x $SIZEX, ('#'.('.' x $RT).'#') x $BT, '#' x $SIZEX);
	substr($map[0],$SX,1) = '.';
	substr($map[$SIZEY-1],$TX,1) = '.';
	
	foreach (keys %up) {
		my ($x,$y) = split(/,/);
		#$x = ($x-1+$n) % $RT + 1;
		$y = ($y-1-$n) % $BT + 1;
		substr($map[$y],$x,1) = '^';
	}
	
	foreach (keys %dn) {
		my ($x,$y) = split(/,/);
		#$x = ($x-1+$n) % $RT + 1;
		$y = ($y-1+$n) % $BT + 1;
		substr($map[$y],$x,1) = (substr($map[$y],$x,1) eq '.')? 'v' : '1';
	}
	
	foreach (keys %lt) {
		my ($x,$y) = split(/,/);
		$x = ($x-1-$n) % $RT + 1;
		#$y = ($y-1+$n) % $BT + 1;
		substr($map[$y],$x,1) = (substr($map[$y],$x,1) eq '.')? '<' : 
			(substr($map[$y],$x,1) ge '2' && substr($map[$y],$x,1) le '9') ? substr($map[$y],$x,1)+1 : '2';
	}
	
	foreach (keys %rt) {
		my ($x,$y) = split(/,/);
		$x = ($x-1+$n) % $RT + 1;
		#$y = ($y-1+$n) % $BT + 1;
		substr($map[$y],$x,1) = (substr($map[$y],$x,1) eq '.')? '>' : 
			(substr($map[$y],$x,1) ge '2' && substr($map[$y],$x,1) le '9') ? substr($map[$y],$x,1)+1 : '2';
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

my %seen;

my $pq = new List::PriorityQueue;
$pq->insert(join(',',0,$SX,$SY,''), 0);
while (my $item = $pq->pop()) {
	my ($t,$x,$y,$moves) = split(/,/, $item);
	next if ($y < 0);
#	dmp($t,$x,$y);
	if ($x == $TX && $y == $TY) {
		$part1 = $t;
		printf("%s\n",$moves);
		last;
	}
	next if ($seen{"$t,$x,$y"}++);
	my $map = mapn($t);
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

printf("Part1: %s\n", $part1);
printf(STDERR "Total time = %f\n", time - $start);

# Reverse
$pq = new List::PriorityQueue;
$pq->insert(join(',',$part1,$TX,$TY), 0);
while (my $item = $pq->pop()) {
	my ($t,$x,$y,@moves) = split(/,/, $item);
	next if ($y > $TY);
#	dmp($t,$x,$y);
	if ($x == $SX && $y == $SY) {
		$part2 = $t;
		last;
	}
	next if ($seen{"$t,$x,$y"}++);
	my $map = mapn($t);
	my @map = split(/\n/,$map);
	next unless (substr($map[$y],$x,1) eq '.');
	# Try down:
	$pq->insert(join(',',$t+1,$x,$y+1),$t*1000 + abs($SX-$x) + abs($SY-($y+1)));
	# Try left:
	$pq->insert(join(',',$t+1,$x-1,$y),$t*1000 + abs($SX-($x-1)) + abs($SY-$y));
	# Try right:
	$pq->insert(join(',',$t+1,$x+1,$y),$t*1000 + abs($SX-($x+1)) + abs($SY-$y));
	# Try up:
	$pq->insert(join(',',$t+1,$x,$y-1),$t*1000 + abs($SX-$x) + abs($SY-($y-1)));
	# Stand still:
	$pq->insert(join(',',$t+1,$x,$y),$t*1000 + abs($SX-$x) + abs($SY-$y));
}
printf("Back at start $part2\n");

#forwards again
$pq = new List::PriorityQueue;
$pq->insert(join(',',$part2,$SX,$SY), 0);
while (my $item = $pq->pop()) {
	my ($t,$x,$y) = split(/,/, $item);
	next if ($y < 0);
#	dmp($t,$x,$y);
	if ($x == $TX && $y == $TY) {
		$part2 = $t;
		last;
	}
	next if ($seen{"$t,$x,$y"}++);
	my $map = mapn($t);
	my @map = split(/\n/,$map);
	next unless (substr($map[$y],$x,1) eq '.');
	# Try down:
	$pq->insert(join(',',$t+1,$x,$y+1),$t*1000 + abs($TX-$x) + abs($TY-($y+1)));
	# Try left:
	$pq->insert(join(',',$t+1,$x-1,$y),$t*1000 + abs($TX-($x-1)) + abs($TY-$y));
	# Try right:
	$pq->insert(join(',',$t+1,$x+1,$y),$t*1000 + abs($TX-($x+1)) + abs($TY-$y));
	# Try up:
	$pq->insert(join(',',$t+1,$x,$y-1),$t*1000 + abs($TX-$x) + abs($TY-($y-1)));
	# Stand still:
	$pq->insert(join(',',$t+1,$x,$y),$t*1000 + abs($TX-$x) + abs($TY-$y));
}

printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);

