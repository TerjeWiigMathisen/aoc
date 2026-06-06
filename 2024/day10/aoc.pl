#!perl -w
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

push(@lines,'');
while (<>) {
	chomp;
	push(@lines, '0'.$_.'0');
}
push(@lines,'0' x length($lines[1]));
$lines[0] = '0' x length($lines[1]);

my $W = length($lines[1])-2;
my $H = scalar(@lines)-2;

#part1
my @heads = ();
my @tops = ();
my %seen = ();
my $tops = 0;
my @dx = (0,1,0,-1);
my @dy = (-1,0,1,0);

sub find_tops
{
	my ($x,$y,$curr) = @_;
	return 0 if (defined($seen{"$x,$y"}));
	$seen{"$x,$y"} = 1;

	if ($curr == 9) {
		return 1;
	}
	
	my $tops = 0;
	$curr++;

	foreach (0..3) {
		my ($nx,$ny) = ($x+$dx[$_],$y+$dy[$_]);
		if (substr($lines[$ny],$nx,1) == $curr) {
			$tops += find_tops($nx,$ny,$curr);
		}
	}
	return $tops;
}

for (my $y = 1; $y <= $H; $y++) {
	for (my $x = 1; $x <= $W; $x++) {
		if (substr($lines[$y],$x,1) == 0) {
			#printf("Trailhead at ($x,$y)");
			%seen = ();
			$part1 += find_tops($x,$y,0);
		}
	}
}

sub find_trails
{
	my ($x,$y,$curr) = @_;

	if ($curr == 9) {
		return 1;
	}

	my $tops = 0;
	foreach (0..3) {
		my ($nx,$ny) = ($x+$dx[$_],$y+$dy[$_]);
		my $c = substr($lines[$ny],$nx,1);
		if ($c == $curr+1) {
			$tops += find_trails($nx,$ny,$c);
		}
	}
	return $tops;
}

for (my $y = 1; $y <= $H; $y++) {
	for (my $x = 1; $x <= $W; $x++) {
		if (substr($lines[$y],$x,1) == 0) {
			$part2 += find_trails($x,$y,0);
		}
	}
}


my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
