#!perl -w
# 1.113 ms minimum runtime
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

while (<>) {
	chomp;
	push(@lines, $_);
}

my %nodes = ();
for (my $y = 0; $y < scalar(@lines); $y++) {
	for (my $x = 0; $x < length($lines[$y]); $x++) {
		my $c = substr($lines[$y],$x,1);
		if ($c ne '.') {
			push(@{$nodes{$c}},"$x,$y");
		}
	}
}

my $W = length($lines[0]);
my $H = scalar(@lines);
#printf("Board = %dx%d\n", $W,$H);

sub gcd {
  my ($a, $b) = @_;
  ($a,$b) = (abs($a),abs($b));
  ($a,$b) = ($b,$a) if $a > $b;
  while ($a) {
    ($a, $b) = ($b % $a, $a);
  }
  return $b;
}

# Part1 & Part2
my %anti = ();
my %anti2 = ();
foreach (sort keys %nodes) {
	my @nod = @{$nodes{$_}};
	for (my $j = 1; $j < scalar(@nod); $j++) {
		my ($x,$y);
		for (my $i = 0; $i < $j; $i++) {
			my ($x0,$y0) = split(/,/,$nod[$i]);
			my ($x1,$y1) = split(/,/,$nod[$j]);
			my ($dx,$dy) = ($x1-$x0,$y1-$y0);

			# Two antinodes for part1
			($x, $y) = ($x1+$dx,$y1+$dy);
			if ($x >= 0 && $x < $W && $y >= 0 && $y < $H) {
				$anti{"$x,$y"}++;
			}
			($x, $y) = ($x0-$dx,$y0-$dy);
			if ($x >= 0 && $x < $W && $y >= 0 && $y < $H) {
				$anti{"$x,$y"}++;
			}

			# Fill in the full line for part2:
			my $div = gcd($dx,$dy);
			if ($div > 1) {
				$dx /= $div;
				$dy /= $div;
				printf("gcd(%d,%d)=%d\n",$dx,$dy,$div);
			}

			# Going up:
			($x, $y) = ($x1,$y1);
			while ($x >= 0 && $y >= 0 && $x < $W && $y < $H) {
				$anti2{"$x,$y"}++;
				$x += $dx;
				$y += $dy;
			}
			# Going down:
			($x, $y) = ($x1-$dx,$y1-$dy);
			while ($x >= 0 && $y >= 0 && $x < $W && $y < $H) {
				$anti2{"$x,$y"}++;
				$x -= $dx;
				$y -= $dy;
			}
		}
	}
}

$part1 = scalar(keys %anti);
$part2 = scalar(keys %anti2);

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
