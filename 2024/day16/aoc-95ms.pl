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
	next if (substr($map[$y],$x,1) eq '#');

	#printf("(%3d,%3d),%d,%7d\n", $x,$y,$dir,$cost);
	my $k = join(",",$x,$y,$dir);
	next if (defined($seen{$k})); # && $seen{$k} <= $cost);
	$seen{$k} = $cost;
	if ($x == $ex && $y == $ey) {
		$part1 = $cost;
		last; # The pq makes sure that this is the least cost path
	}
	
	$k = join(",",$x,$y,($dir+1) & 3);
	next if (defined($seen{$k}) && $seen{$k} < $cost-1000);
	$k = join(",",$x,$y,($dir+3) & 3);
	next if (defined($seen{$k}) && $seen{$k} < $cost-1000);
	$k = join(",",$x,$y,($dir+2) & 3); next if (defined($seen{$k}) && $seen{$k} < $cost-2000);
	
	my ($nx,$ny) = ($x+$dx[$dir],$y+$dy[$dir]);
	my $c = $cost+1;
	if (substr($map[$ny],$nx,1) ne '#') {
		$pq->insert(join("\t",$nx,$ny,$dir,$c),$c);
	}
	$c += 999;
	# Turn right
	my $d = ($dir+1) & 3;
	($nx,$ny) = ($x+$dx[$d],$y+$dy[$d]);
	if (substr($map[$ny],$nx,1) ne '#') {
		$pq->insert(join("\t",$x,$y,$d,$c),$c);
	}
	# Turn left
	$d ^= 2;
	($nx,$ny) = ($x+$dx[$d],$y+$dy[$d]);
	if (substr($map[$ny],$nx,1) ne '#') {
		$pq->insert(join("\t",$x,$y,$d,$c),$c);
	}
}

my $used1 = time - $start;

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
	my $k = join(",",$x,$y,$dir); 
	next unless (defined($seen{$k}) && $seen{$k} == $cost);
	next if (defined($bseen{$k}));
	
	$bseen{$k}++;
	next if ($x == $sx && $y == $sy);
	
	my ($px,$py) = ($x - $dx[$dir], $y - $dy[$dir]);
	$k = join(",",$px,$py,$dir);
	if (defined($seen{$k} && $seen{$k} == $cost-1)) {
		$pq->insert(join("\t",$px,$py,$dir,$cost-1),$part1-$cost+1);
	}
	#left turn?
	my $d = ($dir+1)&3;
	$k = join(",",$x,$y,$d);
	if (defined($seen{$k} && $seen{$k} == $cost-1000)) {
		$pq->insert(join("\t",$x,$y,$d,$cost-1000),$part1-$cost+1000);
	}
	# right turn?
	$d ^= 2;
	$k = join(",",$x,$y,$d);
	if (defined($seen{$k} && $seen{$k} == $cost-1000)) {
		$pq->insert(join("\t",$x,$y,$d,$cost-1000),$part1-$cost+1000);
	}
}

my %paths = ();
foreach (keys %bseen) {
	my ($x,$y,$dir) = split(/,/);
	$paths{"$x,$y"}++;
}
$part2 = scalar(keys %paths);

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms %5.3fms\n",$used1*1000, $used*1000);

