#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);
use List::PriorityQueue;

#use bigint;

#use JSON::Parse;
no warnings 'recursion';

my $start = time;

my $DEBUG = 0;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

my $q = q(The first floor contains a strontium generator, a strontium-compatible microchip, a plutonium generator, and a plutonium-compatible microchip.
The second floor contains a thulium generator, a ruthenium generator, a ruthenium-compatible microchip, a curium generator, and a curium-compatible microchip.
The third floor contains a thulium-compatible microchip.
The fourth floor contains nothing relevant.);

my $mv = 0;
my $el = 0;
@inp = ("sg,sm,pg,pm","tg,rg,rm,cg,cm","tm","");

if ($DEBUG) {
	$mv = 4;
	$el = 2;
	@inp = ("pm,sm","cg,cm,rg,rm,sg","tg,tm","pg");
}

#@inp = ("hm,lm","hg","lg","");

foreach (@inp) {
	my @items = reverse sort split(/,/);
	$_ = join(",", @items);
}

my %cost = ();

my $part1 = 1e38;
my $part2 = 0;

sub collection_ok
{
	my @items = @_;
	return 1 if (scalar(@items) <= 1);

	my %m = ();
	my %g = ();
	foreach (@items) { 
		if (substr($_,1,1) eq 'm') {
			$m{substr($_,0,1)}++;
		}
		else {
			$g{substr($_,0,1)}++;
		}
	}
	return 1 if (scalar(keys %m) == 0 || scalar(keys %g) == 0);
	
	foreach (keys %m) {
		return 0 unless (defined($g{$_}));
	}
	return 1;
}

my $calls = 0;

sub moves
{
	my ($moves, $ele, $solution, @floors) = @_;
	$calls++;
	my $c = $calls;
	my $key = join(";",$ele,@floors);
#	printf("%s%6d %d %s %s", "" x $moves, $moves, $ele, join(" / ",@floors), $solution);
	if (defined($cost{$key})) {
		if ($cost{$key} <= $moves) {
#			printf(" Previously seen!\n");
			return -1;
		}
	}
	$cost{$key} = $moves;
	if ($floors[0] eq '' && $floors[1] eq '' && $floors[2] eq '') {
		printf(" Solution found: %d\n", $moves) ;
		if ($moves < $part1) {
			$part1 = $moves ;
			printf("******** New best solution found: %d\n", $moves) ;
		}
		return 1;
	}
	# Try to move one or two items away from floors[ele]
	my @items = split(/,/,$floors[$ele]);
	if (!collection_ok(@items)) {
		printf(" Invalid!\n");
		return -1;
	}
#	printf("\n");
	my @moves = ();
	for (my $first = 0; $first < scalar(@items); $first++) {
		for (my $second = $first; $second < scalar(@items); $second++) {
			my @rest = @items;
			my @pick = splice(@rest,$second,1);
			if ($second != $first) { # More than one to be moved?
				unshift(@pick, splice(@rest,$first,1));
			}
			push(@moves, join(",",@pick));
		}
	}
#	printf("%d Alternative moves from %d: %s\n", $c, $ele, join(" / ",@moves));
	foreach (@moves) {
		my @pick = split(/,/);
		my @rest = ();
		foreach (@items) {
			if ($_ ne $pick[0] && $_ ne $pick[-1]) {
				push(@rest,$_);
			}
		}
#		printf("%d Trying to move %s from %d: %s\n", $c, join(",", @pick), $ele, $floors[$ele]);
		if (collection_ok(@rest) && collection_ok(@pick)) {
			if ($ele < 3) { # && scalar(@pick) == 2) { # and substr($pick[0],0,1) eq 't') {
#				printf("%d Trying to move %s up from %d: %s\n", $c, join(",", @pick), $ele, $floors[$ele]);
				my @next = reverse sort (split(/,/,$floors[$ele+1]), @pick);
				if (collection_ok(@next)) {
					my @f = @floors;
					$f[$ele] = join(",",@rest);
					$f[$ele+1] = join(",",@next);
					moves($moves+1, $ele+1,$solution."(+".join(",",@pick).")", @f);
				}
			}
			if ($ele > 0) { # && (substr($pick[0],0,1) eq 't' || substr($pick[-1],0,1) eq 't')) { # Only T can act as ferry man!
#				printf("%d Trying to move %s down from %d: %s\n", $c, join(",", @pick), $ele, $floors[$ele]);
				my @next = reverse sort (split(/,/,$floors[$ele-1]), @pick);
				if (collection_ok(@next)) {
					my @f = @floors;
					$f[$ele] = join(",",@rest);
					$f[$ele-1] = join(",",@next);
					moves($moves+1, $ele-1,$solution."(-".join(",",@pick).")", @f);
				}
			}
		}
	}
	return 0;
}

sub movestack
{
	my ($ele, @floors) = @_;
	$calls++;
	my $moves = 0;
	my @solution = ();
	my $pq = new List::PriorityQueue;
	my $c = $calls;
	my $key = join(";",$ele,@floors);
	my $dist = scalar(split(/,/,$floors[0]))*3+scalar(split(/,/,$floors[1]))*2+scalar(split(/,/,$floors[2]));
	$pq->insert(join("\t", $moves, $dist, $key), $dist);
	while ($key = $pq->pop()) {
#		printf("%s retrieved\n", $key);
		($moves, $dist, $key) = split(/\t/,$key);
		($ele, @floors) = split(/;/, $key);
		$floors[3] .= "";
#		printf("%3d %d %s\n", $moves, $ele, join(" / ",@floors)); #, join("-", @solution));
		if (defined($cost{$key})) {
			if ($cost{$key} <= $moves) {
#				printf(" Previously seen!\n");
				next;
			}
		}
		$cost{$key} = $moves;
		if ($floors[0] eq '' && $floors[1] eq '' && $floors[2] eq '') {
			$part1 = $moves if ($moves < $part1);
			printf("%d Solution found!\n", $part1);
			next;
		}
		# Try to move one or two items away from floors[ele]
		my @items = split(/,/,$floors[$ele]);
		my @moves = ();
		for (my $first = 0; $first < scalar(@items); $first++) {
			for (my $second = $first; $second < scalar(@items); $second++) {
				my @rest = @items;
				my @pick = splice(@rest,$second,1);
				if ($second != $first) { # More than one to be moved?
					unshift(@pick, splice(@rest,$first,1));
				}
				push(@moves, join(",",@pick));
			}
		}
#		printf("%d Alternative moves from %d: %s\n", $c, $ele, join(" / ",@moves));
		foreach (@moves) {
			my @pick = split(/,/);
			my @rest = ();
			foreach (@items) {
				if ($_ ne $pick[0] && $_ ne $pick[-1]) {
					push(@rest,$_);
				}
			}
#			printf("%d Trying to move %s from %d: %s\n", $c, join(",", @pick), $ele, $floors[$ele]);
			if (collection_ok(@rest) && collection_ok(@pick)) {
				if ($ele < 3) { # && scalar(@pick) == 2) { # and substr($pick[0],0,1) eq 't') {
					my @next = sort (split(/,/,$floors[$ele+1]), @pick);
					if (collection_ok(@next)) {
						my @f = @floors;
						$f[$ele] = join(",",@rest);
						$f[$ele+1] = join(",",@next);
						my $q = join("\t", $moves+1,$dist-scalar(@pick),join(";",$ele+1,@f));
#						printf("%s insert\n", $q);
						$pq->insert($q, $dist-scalar(@pick));
					}
				}
				if ($ele > 0) { # && (substr($pick[0],0,1) eq 't' || substr($pick[-1],0,1) eq 't')) { # Only T can act as ferry man!
					my @next = sort (split(/,/,$floors[$ele-1]), @pick);
					if (collection_ok(@next)) {
#						printf("%d Trying to move %s down from %d: %s\n", $c, join(",", @pick), $ele, $floors[$ele]);
						my @f = @floors;
						$f[$ele] = join(",",@rest);
						$f[$ele-1] = join(",",@next);
						my $q = join("\t", $moves+1,$dist+scalar(@pick),join(";",$ele-1,@f));
#						printf("%s insert\n", $q);
						$pq->insert($q, $dist+scalar(@pick));
					}
				}
			}
		}
	}
	return 0;
}

#moves($mv,$el,'',@inp);
movestack(0,@inp);

printf(STDERR "Total time = %f\n", time - $start);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
