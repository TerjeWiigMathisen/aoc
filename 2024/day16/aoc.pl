#!perl -w
use strict;
use Time::HiRes qw (time);
use English;
use warnings;
#no warnings 'recursion';
#use bigint;
use List::PriorityQueue;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @map = ();

my ($sx,$sy,$ex,$ey) = (0,0,0,0);
my $y = 0;
while (<>) {
	chomp;
	push(@map, $_);
	my $x = index($_,'S');
	if ($x > 0) {
		$sy = $y;
		$sx = $x;
	}
	$x = index($_,'E');
	if ($x > 0) {
		$ey = $y;
		$ex = $x;
	}
	$y++;
}
printf("Start ($sx,$sy), end ($ex,$ey)\n");

my @dx = (1,0,-1,0);
my @dy = (0,1,0,-1);

my $pq = new List::PriorityQueue;
$pq->insert(join("\t", $sx, $sy, 0, 0), 0);
my %seen = ();

while (my $key = $pq->pop()) {
	my ($x,$y,$dir,$cost) = split(/\t/,$key);
	#printf("$key\n");
	#die("Bad $x,$y,$dir,$cost") if ($x < 0 || $x >= length($map[0]) || $y < 0 || $y >= scalar(@map) || !defined($cost));
	#next if (substr($map[$y],$x,1) eq '#');

	#printf("(%3d,%3d),%d,%7d\n", $x,$y,$dir,$cost);
	my $k = join(",",$x,$y,$dir & 1);
	next if (defined($seen{$k}) && $seen{$k} <= $cost);
	$seen{$k} = $cost;
	if ($x == $ex && $y == $ey) {
		$part1 = $cost;
		last; # The pq makes sure that this is the least cost path
	}
	
#	$k = join(",",$x,$y,($dir+1) & 3);
#	next if (defined($seen{$k}) && $seen{$k} < $cost-1000);
#	$k = join(",",$x,$y,($dir+3) & 3);
#	next if (defined($seen{$k}) && $seen{$k} < $cost-1000);
#	$k = join(",",$x,$y,($dir+2) & 3); next if (defined($seen{$k}) && $seen{$k} < $cost-2000);
	
	my ($nx,$ny) = ($x+$dx[$dir],$y+$dy[$dir]);
	my $c = $cost+1;
	if (substr($map[$ny],$nx,1) ne '#') {
		$pq->insert(join("\t",$nx,$ny,$dir,$c),$c);
	}
	$c += 1000;
	# Try to turn right
	my $d = ($dir+1) & 3;
	
	($nx,$ny) = ($x+$dx[$d],$y+$dy[$d]);
	if (substr($map[$ny],$nx,1) ne '#') { # Can we move here?
		my $k = join(",",$x,$y,$d&1);
		if (!defined($seen{$k}) || $seen{$k} >= $c-1) {
			$seen{$k} = $c-1; # Mark the spot & direction
			$pq->insert(join("\t",$nx,$ny,$d,$c),$c);
		}
	}
	# Try to turn left
	$d ^= 2;
	($nx,$ny) = ($x+$dx[$d],$y+$dy[$d]);
	if (substr($map[$ny],$nx,1) ne '#') {
		my $k = join(",",$x,$y,$d&1);
		if (!defined($seen{$k}) || $seen{$k} >= $c-1) {
			$seen{$k} = $c-1; # Mark the spot & direction
			$pq->insert(join("\t",$nx,$ny,$d,$c),$c);
		}
	}
}

my $used1 = time - $start;

printf("Part1 = $part1, time = %5.3f\n",$used1*1000);

my %bseen = ("$ex,$ey,0" => $part1);
$pq = new List::PriorityQueue;
$pq->insert(join("\t",$ex-1,$ey,0,$part1-1),1);
$pq->insert(join("\t",$ex,$ey-1,1,$part1-1),1);
$pq->insert(join("\t",$ex+1,$ey,2,$part1-1),1);
$pq->insert(join("\t",$ex,$ey+1,3,$part1-1),1);

while (my $key = $pq->pop()) {
	#printf("$key\n");
	my ($x,$y,$dir,$cost) = split(/\t/,$key);
	next if (substr($map[$y],$x,1) eq '#');
	next if ($cost < 0);
	
	#printf("$key\n");
	my $k = join(",",$x,$y,$dir & 1); 
	next unless (defined($seen{$k}) && $seen{$k} == $cost);
	next if (defined($bseen{$k}));
	
	$bseen{$k}++;
	next if ($x == $sx && $y == $sy && $dir == 0);
	
	my ($px,$py) = ($x - $dx[$dir], $y - $dy[$dir]);
	$k = join(",",$px,$py,$dir & 1);
	my $c = $cost-1;
	if (defined($seen{$k}) && $seen{$k} == $c) {
		$pq->insert(join("\t",$px,$py,$dir,$c),$part1-$c);
	}
	#left turn?
	$c -= 1000;
	my $d = ($dir+1)&3;
	($px,$py) = ($x-$dx[$d],$y-$dy[$d]);
	$k = join(",",$px,$py,$d & 1);
	if (defined($seen{$k} && $seen{$k} == $c)) {
		$pq->insert(join("\t",$px,$py,$d,$c),$part1-$c);
	}
	# right turn?
	$d ^= 2;
	($px,$py) = ($x-$dx[$d],$y-$dy[$d]);
	$k = join(",",$px,$py,$d & 1);
	if (defined($seen{$k} && $seen{$k} == $c)) {
		$pq->insert(join("\t",$px,$py,$d,$c),$part1-$c);
	}
}

my %paths = ();
foreach (keys %bseen) {
	my ($x,$y,$dir) = split(/,/);
	$paths{"$x,$y"}++;
}
$part2 = scalar(keys %paths);

my $used = time - $start;

sub show
{
    for (my $y = 0; $y < scalar(@map); $y++) {
		for (my $x = 0; $x < length($map[$y]); $x++) {
			printf("%s", defined($paths{"$x,$y"})? "O" : substr($map[$y],$x,1));
		}
		printf("\n");
	}
}

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms %5.3fms\n",$used1*1000, $used*1000);



