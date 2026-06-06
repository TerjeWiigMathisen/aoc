#!perl -w
use strict;
use Time::HiRes qw (time);
use English;
use warnings;
no warnings 'recursion';
#use bigint;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

my %robots = ();
while (<>) {
	push(@lines, $_);
	if (/p=(\d+),(\d+)\sv=(\-?\d+),(\-?\d+)/) {
		$robots{"$1,$2,$3,$4"}++;
	}
	else {
		printf("Parsing error: $_!\n");
		exit(1);
	}
}
my ($W,$H) = (11,7);
if (shift eq "input.txt") {
	($W,$H) = (101,103);
}
($W,$H) = (101,103);
my $MW = ($W)>>1;
my $MH = ($H)>>1;

my $oneline = join('',@lines);

#part1

my @q = (0,0,0,0);
my %r = ();
my $iter = 100;
foreach (keys %robots) {
	my ($x,$y,$vx,$vy) = split(/,/);
	my $nx = ($x+$vx*$iter)%$W;
	if ($nx < 0) { $nx += $W; }
	my $ny = ($y+$vy*$iter)%$H;
	if ($ny < 0) { $ny += $H; }
	$r{"$nx,$ny"}++;
	my $qx = 0; $qx++ if ($nx > $MW);
	my $qy = 0; $qy++ if ($ny > $MH);
	my $q = $qx + $qy*2;
	if ($nx == $MW || $ny == $MH) {
		#printf("Skip $nx,$ny\n");
	}
	else {
		#printf("$nx,$ny $q\n");
		$q[$q]++;
	}
}
printf("%s\n", join(", ",@q));

$part1 = $q[0]*$q[1]*$q[2]*$q[3];

#part2;
my $previter = 0;
#for (my $iter = 16930; $iter <= 16940; $iter++) {
for (my $iter = 6530; $iter <= 6540; $iter++) {
	my %r = ();
	my $show = 1;
	my $skip = 0;
	foreach (keys %robots) {
		my ($x,$y,$vx,$vy) = split(/,/);
		my $nx = ($x+$vx*$iter)%$W;
		if ($nx < 0) { $nx += $W; }
		my $ny = ($y+$vy*$iter)%$H;
		if ($ny < 0) { $ny += $H; }
		$r{"$nx,$ny"}++;
		my $qx = 0; $qx++ if ($nx > $MW);
		my $qy = 0; $qy++ if ($ny > $MH);
		my $q = $qx + $qy*2;
		if (($nx*2 + $ny) < 70) {
			$skip++;
		}
		if ($ny == 0) {
			if (abs($MW-$nx) < 5) {
				$show = 1;
			}
		}
	}
	next unless ($show && $skip<20);
	printf("Tree $iter (%d)?\n",$iter-$previter);
	$previter = $iter;
	for (my $y = 0; $y < $H; $y++) {
		for (my $x = 0; $x < $W; $x++) {
			if (defined($r{"$x,$y"})) {
				printf("%d",$r{"$x,$y"});
			}
			else {
				printf(".");
			}
		}
		printf("\n");
	}
}

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);

