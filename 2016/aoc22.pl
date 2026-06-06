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
$fname = "input22.txt" unless($fname);
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

#Filesystem              Size  Used  Avail  Use%
# /dev/grid/node-x0-y2     93T   70T    23T   75%

my @used = ();
my @avail = ();

my %grid = ();

my $XMAX = 0;
my $YMAX = 0;

foreach (@inp) {
	if (/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T/) {
		my ($x,$y,$s,$u,$a) = ($1,$2,$3,$4,$5);
		$grid{"$x,$y"} = "$u,$a";

		if ($u) {
			push(@used, sprintf("%8d,%s,%s",$u,$x,$y));
		}
		else {
			printf("Node %s is unused\n", $_);
		}
		if ($a) {
			push(@avail, sprintf("%8d,%s,%s",$a,$x,$y));
		}
		else {
			printf("Node %s is full\n", $_);
		}
		if ($x > $XMAX) {$XMAX = $x; }
		if ($y > $YMAX) {$YMAX = $y; }
	}
}
@used = sort(@used);
@avail = sort @avail;

push(@used,"99999999,0,0");
#push(@avail,"       0",0,0);

my $viable = 0;
my $u = 0;
for (my $a = 0; $a < scalar(@avail); $a++) {
	my ($avail,$x,$y) = split(/,/,$avail[$a]);
	my ($used, $xu, $yu) = split(/,/, $used[$u]);
	while ($used <= $avail) {
		$u++;
		($used, $xu, $yu) = split(/,/, $used[$u]);
	} 
	$viable += $u;
}
my $part1 = $viable;
my $part2 = 0;




printf(STDERR "Total time = %f\n", time - $start);


printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);

