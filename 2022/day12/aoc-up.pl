#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);
#use List::Util qw (reduce);

#use bigint;

#use JSON::Parse;
no warnings 'recursion';

my $DEBUG = 0;

my $fname = shift;
$fname = 'input.txt' unless (defined($fname));
open(F ,'<',$fname);

my $start = time;

my $part1 = 0;
my $part2 = 0;

my @S;
my @E;

my @inp = ();
foreach (<F>) {
	chomp;
	push(@inp, chr(126) x (length($_)+2)) unless (scalar(@inp));
	$_ = chr(126).$_.chr(126);
	if (/E/) {
		@E = (length($PREMATCH), scalar(@inp));
		substr($_,length($PREMATCH),1) = 'z';
	}
	if (/S/) {
		@S = (length($PREMATCH), scalar(@inp));
		substr($_,length($PREMATCH),1) = 'a';
	}
	push(@inp,$_);
}
push(@inp, $inp[0]);
close(F);
#printf(STDERR "%s\n\n", join("\n", @inp));

my %visited = ();

my @queue = ();

sub plot
{
	my ($x0,$y0,$x1,$y1) = (1e8,1e8,-1e8,-1e8);
	foreach (keys %visited) {
		my ($x,$y) = split(/,/,$_);
		$x0 = $x if ($x < $x0);
		$y0 = $y if ($y < $y0);
		$x1 = $x if ($x > $x1);
		$y1 = $y if ($y > $y1);
	}
	for (my $y = $y0; $y <= $y1; $y++) {
		my @line = ();
		for (my $x = $x0; $x <= $x1; $x++) {
			my $s = defined($visited{"$x,$y"}) ? $visited{"$x,$y"} : 0;
			push(@line,$s);
		}
		printf("%s\n", join(',', @line));
	}
	printf("\n");
}

sub height
{
	my ($x, $y) = @_;
	my $h = ord(substr($inp[$y],$x,1));
	return $h;
}

sub try_up
{
	my ($x0, $y0) = @_;
	@queue = ($x0, $y0, 0);
	while (scalar(@queue)) {
		my ($x, $y, $steps) = splice(@queue,0,3);
		next if (defined($visited{"$x,$y"}));
		$visited{"$x,$y"} = $steps;
	
		if ($x == $E[0] && $y == $E[1]) {
			return $steps;
		}
		$steps++;
		my $h = height($x,$y);
		push(@queue, $x-1, $y, $steps) if (height($x-1,$y) <= $h+1);
		push(@queue, $x+1, $y, $steps) if (height($x+1,$y) <= $h+1);
		push(@queue, $x, $y-1, $steps) if (height($x,$y-1) <= $h+1);
		push(@queue, $x, $y+1, $steps) if (height($x,$y+1) <= $h+1);
	} 
	return 1e8;
}

#$part1 = try_up(@S);
#printf(STDERR "Total time = %f\n", time - $start);
#plot();

%visited = ();
foreach (@inp) { s/~/ /g; } # Change boundary guard from '~' to ' '!

sub try_reverse
{
	my ($x0, $y0) = @_;
	@queue = ($x0, $y0, 0);
	my $part2 = 0;
	while (scalar(@queue)) {
		my ($x, $y, $steps) = splice(@queue,0,3);
		next if (defined($visited{"$x,$y"}));
		$visited{"$x,$y"} = $steps;
	
		my $h = height($x,$y);
		if ($h <= ord('a')) { # Any 'a' is at starting elevation, so OK!
			if ($part2 == 0) { $part2 = $steps; }
			if ($x == $S[0] && $y == $S[1]) {
				return ($steps, $part2);
			}
		}
		$steps++;
		$h--;
		push(@queue, $x-1, $y, $steps) if (height($x-1,$y) >= $h);
		push(@queue, $x+1, $y, $steps) if (height($x+1,$y) >= $h);
		push(@queue, $x, $y-1, $steps) if (height($x,$y-1) >= $h);
		push(@queue, $x, $y+1, $steps) if (height($x,$y+1) >= $h);
	}
	return (1e8,1e8);
}

($part1, $part2) = try_reverse(@E);
#plot();

my $tid = time - $start;

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %1.3f ms\n", $tid*1000);
