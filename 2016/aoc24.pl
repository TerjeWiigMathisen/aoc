#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);
use List::PriorityQueue;
use Digest::MD5;

#use bigint;

#use JSON::Parse;
no warnings 'recursion';

my $start = time;

my $DEBUG = 0;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

my %dig = ();
my $y = 0;
foreach (@inp) {
	my $l = $_;
	for (my $dig = 0; $dig <= 9; $dig++) {
		my $x = index($_, $dig);
		if ($x >= 0) {
			$dig{$dig} = "$x,$y";
		}
	}
	$y++;
}

sub dijkstra
{
	my ($f, $t) = @_;
	my ($x0,$y0) = split(/,/, $dig{$f});
	my ($x1,$y1) = split(/,/, $dig{$t});
	my @b = @inp;
	my $pq = new List::PriorityQueue;
	$pq->insert("$x0,$y0,0",0);
	while (my $key = $pq->pop()) {
		my ($x,$y,$steps) = split(/,/,$key);
		if (substr($b[$y],$x,1) eq $t) {
			return $steps;
		}
		substr($b[$y],$x,1) = '#';
		foreach ("-1,0","1,0","0,-1","0,1") {
			my ($dx,$dy) = split(/,/);
			my ($nx,$ny) = ($x+$dx,$y+$dy);
			if (substr($b[$ny],$nx,1) ne '#') {
				$pq->insert(sprintf("%d,%d,%d", $nx,$ny,$steps+1),$steps+1);
			}
		}
	}
	die("No path found from $f to $t!");
}

# Dijkstra for all combinations: 0..last, 1..last, 2..last etc
my @targets = sort keys %dig;

my %route = ();

my $last = $targets[-1];
for (my $f = 0; $f<$last; $f++) {
	for (my $t = $f+1; $t <= $last; $t++) {
		$route{"$t,$f"} = $route{"$f,$t"} = dijkstra($f,$t);
	}
}

# dist[from][to]
my @dist = ();

sub routesfrom
{
	my ($from) = @_;
	my @routes = ();
	foreach (keys %route) {
		my ($f,$t) = split(/,/);
		if ($f == $from) {
			push(@routes, sprintf("%5d,%d",$route{$_}, $t));
		}
	}
	return sort @routes;
}

my $part1 = 1e38;

# Always start at '0':
sub visit
{
	my ($visited, $dist) = @_;
	return if ($dist >= $part1);

	my @visited = split(/,/, $visited);
	my $curr = $visited[-1];
	my @next = routesfrom($curr);
	my @n;
	
	foreach (@next) {
		my ($d,$n) = split(/,/);
		next if (index($visited,$n) >= 0);
		push(@n, $_);
	}
	if (scalar(@n) == 0) {
		printf("Solution found: $dist\n");
		if ($dist < $part1) {
			$part1 = $dist;
		}
		return;
	}
	foreach (@n) {	
		my ($d,$n) = split(/,/);
		visit("$visited,$n", $dist+$d);
	}
}

my $part2 = 1e38;

sub visit2
{
	my ($visited, $dist) = @_;
	return if ($dist >= $part2);

	my @visited = split(/,/, $visited);
	my $curr = $visited[-1];
	my @next = routesfrom($curr);
	my @n;
	
	foreach (@next) {
		my ($d,$n) = split(/,/);
		next if (index($visited,$n) >= 0);
		push(@n, $_);
	}
	if (scalar(@n) == 0) {
		$dist += $route{"$curr,0"};
		if ($dist < $part2) {
			printf("Solution found: $dist\n");
			$part2 = $dist;
		}
		return;
	}
	foreach (@n) {	
		my ($d,$n) = split(/,/);
		visit2("$visited,$n", $dist+$d);
	}
}


visit('0',0); # Will set $part1
visit2('0',0); # Ditto part2

printf(STDERR "Total time = %f\n", time - $start);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);

