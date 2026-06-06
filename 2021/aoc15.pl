#!perl -w

use strict;
use Time::HiRes qw (time);
use Math::BigInt;
use warnings;
no warnings 'recursion';

my $start = time;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

my %cache = ();

my $MAX = 1e38;
my $XMAX = length($inp[0])-1;
my $YMAX = scalar(@inp)-1;

# Start by subtracting the cost of the top left node!
my @try = (-substr($inp[0],0,1),0,0);

sub try
{
	while (scalar(@try)) {
		my ($risk, $x, $y) = (shift @try, shift @try, shift @try);
		$risk += substr($inp[$y],$x,1);
		my $key = "$x,$y";
		next if (defined($cache{$key}) && $risk >= $cache{$key});
		$cache{$key} = $risk;
		push(@try, $risk, $x-1, $y) if ($x > 0);
		push(@try, $risk, $x+1, $y) if ($x < $XMAX);
		push(@try, $risk, $x, $y-1) if ($y > 0);
		push(@try, $risk, $x, $y+1) if ($y < $YMAX);;
	}
}

try ();

my $part1 = $cache{"$XMAX,$YMAX"};

sub inc
{
	my ($l) = $_;
	my @o = ();
	foreach (split(//, $l)) {
		$_++;
		$_ = 1 if ($_ > 9);
		push(@o, sprintf("%d",$_));
	}
	return join("", @o);
}

sub inc_map
{
	my (@inp) = @_;
	my @out = ();
	foreach (@inp) {
		push(@out,inc($_));
	}
	return @out;
}

sub add
{
	my ($y, @map) = @_;
	if ($y >= scalar(@inp)) {
		push(@inp, @map);
	}
	else {
		foreach (@map) {
			$inp[$y++] .= $_;
		}
	}
}

my @inc = @inp;
# Top-left triangle
for (my $diag = 1; $diag <= 4; $diag++) { 
	@inc = inc_map(@inc);
	for (my $y = 0; $y <= $diag; $y++) {
		add($y*scalar(@inc), @inc);
	}
}
# Bottom-right triangle
for (my $diag = 5; $diag <= 8; $diag++) { 
	@inc = inc_map(@inc);
	for (my $y = $diag-4; $y <= 4; $y++) {
		add($y*scalar(@inc), @inc);
	}
}

printf(STDERR "%s\n\n", join("\n", @inp));
$XMAX = ($XMAX+1)*5-1;
$YMAX = ($YMAX+1)*5-1;

%cache = ();
@try = (-substr($inp[0],0,1),0,0);
try ();

my $part2 = $cache{"$XMAX,$YMAX"};

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %d\n", $part1);
printf("Part2: %d\n", $part2);

