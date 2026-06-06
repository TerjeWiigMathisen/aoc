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

my @nodes = ();
for (my $y = 0; $y < scalar(@map); $y++) {
	for (my $x = 0; $x < length($map[$y]); $x++) {
		my $c = substr($map[$y],$x,1);
		if ($c ne '#') {
			my $walls = substr($map[$y],$x-1,1) == '#' +
				substr($map[$y],$x-1,1) == '#' +
				substr($map[$y],$x-1,1) == '#' +
				substr($map[$y],$x-1,1) == '#';
			$c = 'o';
		}
		printf("%s",$c);
	}
	printf("\n");
}


my $pq = new List::PriorityQueue;
$pq->insert(join("\t", $sx, $sy, 0, 0), 0);
my %seen = ();
my %bestpath = ();
while (my $key = $pq->pop()) {
	my ($x,$y,$dir,$cost,@path) = split(/\t/,$key);
	next if (substr($map[$y],$x,1) eq '#');
	my $k = join(",",$x,$y,$dir);
	next if (defined($seen{$k}));
	$seen{$k} = $cost;
	push(@path,$k);
		
	# While single path straight ahead from here
	while (1) {
		my ($lx,$ly) = ($x+$dx[($dir+3)&3],$y+$dy[($dir+3)&3]);
		my ($rx,$ry) = ($x+$dx[($dir+1)&3],$y+$dy[($dir+1)&3]);
		if (substr($map[$ly],$lx,1) ne '#' || substr($map[$ly],$lx,1) ne '#') {
			last;
		}
		$cost++;
		($x,$y) = ($x+$dx[$dir],$y+$dy[$dir]);
		# if cul-de-sac?
		if (substr($map[$y],$x,1) eq '#') {
			$dir ^= 2;
			# Backtrack and fill
			while (1) {
				($x,$y) = ($x+$dx[$dir],$y+$dy[$dir]);
				($lx,$ly) = ($x+$dx[($dir+3)&3],$y+$dy[($dir+3)&3]);
				($rx,$ry) = ($x+$dx[($dir+1)&3],$y+$dy[($dir+1)&3]);
				if (substr($map[$ly],$lx,1) ne '#' || substr($map[$ly],$lx,1) ne '#') {
					last;
				}
				substr($map[$y],$x,1) = '#';
			}
			last;
		}
			
		push(@path,"$x,$y,$dir");
		if ($x == $ex && $y == $ey) {
			last;
		}
	}
	if ($x == $ex && $y == $ey) {
		$part1 = $cost;
		foreach (@path) {
			$bestpath{$_}++;
		}
		# Setup best path cache for part2:
		last; # The pq makes sure that this is the least cost path
	}

		
	my ($nx,$ny) = ($x+$dx[$dir],$y+$dy[$dir]);
	$cost++;
	$pq->insert(join("\t",$nx,$ny,$dir,$cost,@path),$cost);
	$cost += 999;
	# Turn left
	$pq->insert(join("\t",$x,$y,($dir+3)&3,$cost,@path),$cost);
	# Turn right
	$pq->insert(join("\t",$x,$y,($dir+1)&3,$cost,@path),$cost);
}

printf("part1 = $part1 (%d cells)\n", scalar keys %bestpath);
my %r = ();
foreach (keys %bestpath) {
	my ($x,$y,$d) = split(/,/);
	$r{"$x,$y"}++;
}
for (my $y = 0; $y < scalar(@map); $y++) {
	for (my $x = 0; $x < length($map[$y]); $x++) {
		my $c = substr($map[$y],$x,1);
		if (defined($r{"$x,$y"})) {
			$c = 'o';
		}
		printf("%s",$c);
	}
	printf("\n");
}
printf("\n");


#part2
$pq = new List::PriorityQueue;
$pq->insert(join("\t", $sx, $sy, 0, 0, 1), 0);
%seen = ();
my %rest = %bestpath;
my $bestskip = 0;
while (my $key = $pq->pop()) {
	my ($x,$y,$dir,$cost,$prev,@path) = split(/\t/,$key);

	next if (substr($map[$y],$x,1) eq '#');
	last if ($cost > $part1);

	my $k = join(",",$x,$y,$dir);
	push(@path,$k);
	if ($x == $ex && $y == $ey) {
		printf("Found path with cost $cost\n");
		die("Bad cost path") unless ($cost == $part1);
		foreach (@path) {
			$rest{$_}++;
		}
		next;
	}


	my $s = $seen{$k};
	if (defined($s)) {
		next if ($cost>$s);
		die("Bad cost!") if ($cost < $s);
		if (defined($bestpath{$k})) {
			foreach (@path) {
				$rest{$_}++;
			}
			$bestskip++;
			next;
		}
	}
	$seen{$k}=$cost;
		
	# Try straight from here:
	my ($nx,$ny) = ($x+$dx[$dir],$y+$dy[$dir]);
	$cost++;
	$pq->insert(join("\t",$nx,$ny,$dir,$cost,$prev,@path),$cost);
	$cost += 999;
	# Turn left
	$pq->insert(join("\t",$x,$y,($dir+3)&3,$cost,$prev,@path),$cost);
	# Turn right
	$pq->insert(join("\t",$x,$y,($dir+1)&3,$cost,$prev,@path),$cost);
}

printf("Skip after rejoining best path: %d\n",$bestskip);

%r = ();
#printf("%s\n",join(" ",keys %rest));
foreach (keys %rest) {
	my ($x,$y,$d) = split(/,/);
	$r{"$x,$y"}++;
}
$part2 = scalar(keys %r);
#printf("%s\n",join(" ",sort keys %rest));
for (my $y = 0; $y < scalar(@map); $y++) {
	for (my $x = 0; $x < length($map[$y]); $x++) {
		my $c = substr($map[$y],$x,1);
		if (defined($r{"$x,$y"})) {
			$c = 'O';
		}
		printf("%s",$c);
	}
	printf("\n");
}
printf("\n");


my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);

