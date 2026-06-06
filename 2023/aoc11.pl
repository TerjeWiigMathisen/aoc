#!perl -w
use strict;
use Time::HiRes qw(time);

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @inp = (<>); chomp(@inp);
#printf("Input:\n%s\n\n", join("\n",@inp));

my @vert = ();
# expand vertically where?
my $blank = '.' x length($inp[0]);
for (my $y = 0; $y < scalar(@inp); $y++) {
	my $v = 0;
	if ($inp[$y] eq $blank) {
		$v = 0xffffffff;
	}
	push(@vert, $v);
}

#printf("Expand:\n%s\nVert: %s\n\n", join("\n",@ey), join(" ",sort {$a <=> $b} keys %vert));

my @hori = ();
# expand horizontally if a vertical line is empty
for (my $x = 0; $x < length($inp[0]); $x++) {
	my $blank = 1;
	for (my $y = 0; $y < scalar(@inp); $y++) {
		my $c = substr($inp[$y],$x,1);
		if ($c ne '.') {
			$blank = 0;
		}
	}
	my $h = 0;
	if ($blank) {
		$h = 0xffffffff;
	}
	push(@hori,$h);
}

my @gal = ();

# Find the galaxies
for (my $y = 0; $y < scalar(@inp); $y++) {
	for (my $x = 0; $x < length($inp[0]); $x++) {
		my $c = substr($inp[$y],$x,1);
		if ($c eq '#') {
			push(@gal,"$x;$y");
		}
	}
}

my $setup = time-$t0;
printf("Setup time:%1.6f\n",$setup);
#printf("%s\n\%s\%s\n", join("\n",@gal),join(" ",keys %hori), join(" ", keys %vert));

sub manhattan_with_expansion
{
	my ($sx,$sy, $x,$y, $exp) = @_;
	
	my ($a,$b) = ($x >= $sx)? ($sx,$x):($x,$sx);
	my $len = 0;
	$exp--;
	while ($a != $b) {
		$len+= 1 + ($exp&$hori[$a]);
		$a++;
	}
	($a,$b) = ($y >= $sy)? ($sy,$y):($y,$sy);
	while ($a != $b) {
		$len+= 1 + ($exp&$vert[$a]);
		$a++;
	}
	
	return $len;
}

for (my $first = 1; $first < scalar(@gal); $first++) {
	my ($sx,$sy) = split(/;/,$gal[$first-1]);
	for (my $sec = $first; $sec < scalar(@gal); $sec++) {
		my ($x,$y) = split(/;/,$gal[$sec]);
		my $man = abs($sx-$x)+abs($sy-$y);
		my $exp = manhattan_with_expansion($sx,$sy, $x,$y, 2);
		$part1 += $exp;
		$part2 += $man + ($exp-$man)*(999999);
	}
}

my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.6fs\n%d galaxies in a (%d,%d) universe", $used,scalar(@gal),length($inp[0]),scalar(@inp));
