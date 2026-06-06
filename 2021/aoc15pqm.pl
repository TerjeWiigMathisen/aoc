#!perl -w

use strict;
use Time::HiRes qw (time);
use List::PriorityQueue;
use warnings;
no warnings 'recursion';

my $start = time;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

my $MANHATTAN = shift || 0;

my %cache;

my $MAX = 1e38;
my $XMAX = length($inp[0])-1;
my $YMAX = scalar(@inp)-1;

my $pq; # Priority Queue!

my $enq_cnt = 0;

sub enq
{
	my ($risk, $x, $y) = @_;
	my $key = "$x,$y";
	$risk += substr($inp[$y],$x,1);
	return if (defined($cache{$key}) && $risk >= $cache{$key});
	$cache{$key} = $risk;
	$pq->insert($key, $risk);
	$enq_cnt++;
}

sub try
{
	%cache = ("0,0" => 0);
	$pq = new List::PriorityQueue;	# Start on the top left node!
	$pq->insert("0,0",0);
	while (my $key = $pq->pop()) {
		my ($x, $y) = split(/,/, $key);
		my $risk = $cache{$key};
		if (!defined($risk)) {
			printf("%s\n", $key);
		}
		return $risk if ($x == $XMAX && $y == $YMAX);

		enq($risk+$MANHATTAN, $x-1, $y) if ($x > 0);
		enq($risk-$MANHATTAN, $x+1, $y) if ($x < $XMAX);
		enq($risk+$MANHATTAN, $x, $y-1) if ($y > 0);
		enq($risk-$MANHATTAN, $x, $y+1) if ($y < $YMAX);;
	}
	return $cache{"$XMAX,$YMAX"};
}

my $part1 = try ();

replicate5x5();



my $part2 = try ();

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %d\n", $part1);
printf("Part2: %d\n", $part2);
printf(STDERR "Manhattan = %d, %d visited cells, total enq() calls = %d\n", $MANHATTAN, scalar(keys %cache), $enq_cnt);


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

sub replicate5x5
{
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

	#printf(STDERR "%s\n\n", join("\n", @inp));
	$XMAX = ($XMAX+1)*5-1;
	$YMAX = ($YMAX+1)*5-1;
}

