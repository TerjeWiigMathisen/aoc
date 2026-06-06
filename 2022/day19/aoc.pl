#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);
#use List::Util qw (reduce);
use List::PriorityQueue;
#use bigint;

#use JSON::Parse;
#no warnings 'recursion';

my $DEBUG = 0;

my $fname = shift;
$fname = 'input.txt' unless (defined($fname));
open(F ,'<',$fname);

my $BUFFER = shift;
$BUFFER = 100 unless (defined($BUFFER));

my $start = time;

my $part1 = 0;
my $part2 = 0;

my %fgeo;
my %fobs;
my @inp = ();
if ($fname eq 't.txt') {
}

foreach (<F>) {
	chomp;
	push(@inp,$_);
}
close(F);

my ($blue, $oreore, $clayore, $obsore, $obsclay, $geoore, $geoobs);

sub bitcount
{
	my ($n) = @_;
	return (0,1,1,2,1,2,2,3,1,2,2,3,2,3,3,4)[$n];
}

my %cache;
my $TLIMIT;

sub enq
{
	my ($pq, $t, $rore, $rclay, $robs, $rgeo, $ore, $clay, $obs, $geo, @manu) = @_;
	my $dt = $TLIMIT-$t;
	my $pri;
	$pri = 1e12+$t*1e11-($rgeo*$dt+$geo)*1e9-($robs*$dt+$obs)*1e6-$rclay*1e5-$clay*1e4-$rore;
	$pq->insert(join(',',$t, $rore, $rclay, $robs, $rgeo, $ore, $clay, $obs, $geo, @manu), $pri);
}

sub maxgeodes
{	
	my @seed = @_;
	my $max = 0;
	my $oldt = 0;
	my $maxrobots = '  ';
	my $maxstr = '';
	$TLIMIT = $seed[0];
	$seed[0] = 0;
	my ($t, $rore, $rclay, $robs, $rgeo, $ore, $clay, $obs, $geo); # = splice(@seed,0,9);
	
	my $prio = new List::PriorityQueue;
	my $r = $prio->insert(join(',',@seed,''), 0);
	my @manu;
	my ($core, $cclay, $cobs, $cgeo);
	
	my $buffer = 1;
	my $firstgeo = $TLIMIT;
	my $lt = -1;
	while (my $item = $prio->pop()) {
		($t, $rore, $rclay, $robs, $rgeo, $ore, $clay, $obs, $geo, @manu) = split(/,/, $item);
		my $pri = sprintf("%d,%d,%d,%d", $rgeo, $robs, $rclay, $rore);
		
#		if (scalar(@manu) < $firstgeo && $geo) { $firstgeo = scalar(@manu); }
#		if (scalar(@manu) > $firstgeo+5 && $geo==0) { next; }
		
		if ($t != $lt) {
			$lt = $t;
			$buffer = $BUFFER;
		}
		if ($buffer>0) {
			$buffer--;
		}
		else {
			next;
		}
#		printf("Time %2d: %s\n", $t, join(',', $rore, $rclay, $robs, $rgeo, $ore, 
#			$clay, $obs, $geo, @manu)) if ($t < 0);

		# Pick the next robot to construct:

		my $prod = 0;
		# Try rgeo
		if ($robs) { # && $rore)
			my ($hore, $hclay, $hobs, $hgeo) = ($ore, $clay, $obs, $geo);
			for (my $tt = $t; $tt < $TLIMIT; $tt++) {
				my $ok = ($hore >= $geoore && $hobs >= $geoobs);
				$hore += $rore;
				$hclay += $rclay;
				$hobs += $robs;
				$hgeo += $rgeo;
				if ($ok) {
					$hore -= $geoore;
					$hobs -= $geoobs;
					enq($prio,$tt+1, $rore, $rclay, $robs, $rgeo+1, $hore, $hclay, $hobs, $hgeo,@manu,"RGEO");
					$prod++;
					last;
				}
			}
		}
		# Try robs
		if ($rclay) { # && $obs < $geoobs*2) { # && $rore) 
			my ($hore, $hclay, $hobs, $hgeo) = ($ore, $clay, $obs, $geo);
			for (my $tt = $t; $tt < $TLIMIT; $tt++) {
				my $ok = ($hore >= $obsore && $hclay >= $obsclay);
				$hore += $rore;
				$hclay += $rclay;
				$hobs += $robs;
				$hgeo += $rgeo;
				if ($ok) {
					$hore -= $obsore;
					$hclay -= $obsclay;
					enq($prio,$tt+1, $rore, $rclay, $robs+1, $rgeo, $hore, $hclay, $hobs, $hgeo,@manu,"ROBS");
					$prod++;
					last;
				}
			}
		}
		# Produce rclay
		#if ($clay < $obsclay*2) {
		{
			my ($hore, $hclay, $hobs, $hgeo) = ($ore, $clay, $obs, $geo);
			for (my $tt = $t; $tt < $TLIMIT; $tt++) {
				my $ok = ($hore >= $clayore);
				$hore += $rore;
				$hclay += $rclay;
				$hobs += $robs;
				$hgeo += $rgeo;
				if ($ok) {
					$hore -= $clayore;
					enq($prio,$tt+1, $rore, $rclay+1, $robs, $rgeo, $hore, $hclay, $hobs, $hgeo,@manu,"RCLAY");
					$prod++;
					last;
				}
			}
		}
		# Produce rore
		{ #if ($ore < $clayore + $geoore){
			my ($hore, $hclay, $hobs, $hgeo) = ($ore, $clay, $obs, $geo);
			for (my $tt = $t; $tt < $TLIMIT; $tt++) {
				my $ok = ($hore >= $oreore);
				$hore += $rore;
				$hclay += $rclay;
				$hobs += $robs;
				$hgeo += $rgeo;
				if ($ok) {
					$hore -= $oreore;
					enq($prio,$tt+1, $rore+1, $rclay, $robs, $rgeo, $hore, $hclay, $hobs, $hgeo,@manu,"RORE");
					$prod++;
					last;
				}
			}
		}
		if ($prod == 0) {
			my ($hore, $hclay, $hobs, $hgeo) = ($ore, $clay, $obs, $geo);
			for (my $tt = $t; $tt < $TLIMIT; $tt++) {
				$hore += $rore;
				$hclay += $rclay;
				$hobs += $robs;
				$hgeo += $rgeo;
			}
			if ($hgeo > $max) {
				#printf("New max = %d: %s\n", $hgeo, join(',', $rore, $rclay, $robs, $rgeo, $hore, $hclay, $hobs, $hgeo,@manu));
				$max = $hgeo;
			}
		}
	}
	return $max;
}

foreach (@inp) {
	chomp;
	die("Parse: $_\n") unless (/\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+/);
	($blue, $oreore, $clayore, $obsore, $obsclay, $geoore, $geoobs) = ($1,$2,$3,$4,$5,$6,$7);
	my ($rore, $rclay, $robs, $geo) = (1,0,0,0);
	my $quality = maxgeodes(24, 1,0,0,0, 0,0,0,0);
	printf("Blueprint %d max: %d\n", $blue, $quality);
	$part1 += $blue*$quality;
	#die;
}

printf("Part1: %s\n", $part1);
printf(STDERR "Total time = %f\n", time - $start);

$part2 = 1;
foreach ((@inp)[0,1,2]) {
	chomp;
	die("Parse: $_\n") unless (/\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+/);
	($blue, $oreore, $clayore, $obsore, $obsclay, $geoore, $geoobs) = ($1,$2,$3,$4,$5,$6,$7);
	my ($rore, $rclay, $robs, $geo) = (1,0,0,0);
	my $quality = maxgeodes(32, 1,0,0,0, 0,0,0,0);
	printf("Blueprint %d max: %d\n", $blue, $quality);
	$part2 *= $quality;
	#die;
}

printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);

