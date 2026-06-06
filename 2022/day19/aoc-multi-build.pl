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

my $start = time;

my $part1 = 0;
my $part2 = 0;


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

sub enq
{
	my ($pq, $t, $rore, $rclay, $robs, $rgeo, $ore, $clay, $obs, $geo, @manu) = @_;
	my $dt = 24-$t;
	$dt = 2 if ($dt <= 1);
	my $pri = 1e12-($rore*$dt +$ore +$core)*1 -($rclay*$dt+$clay+$cclay)*100 - 
		($robs*$dt+$obs+$cobs)*1e4 - ($rgeo*$dt+$geo+$cgeo)*1e6;
	$pq->insert(join(',',$t, $rore, $rclay, $robs, $rgeo, $ore, $clay, $obs, $geo, @manu), $pri);
}

sub maxgeodes
{	
	my @seed = @_;
	my $max = -1;
	my $oldt = 0;
	my $maxrobots = '  ';
	my $maxstr = '';
	my $TLIMIT = $seed[0];
	$seed[0] = 0;
	my ($t, $rore, $rclay, $robs, $rgeo, $ore, $clay, $obs, $geo); # = splice(@seed,0,9);

	my $prio = new List::PriorityQueue;
	my $r = $prio->insert(join(',',@seed,''), 0);
	my @manu;
	my ($core, $cclay, $cobs, $cgeo);
	
	while (my $item = $prio->pop()) {
		($t, $rore, $rclay, $robs, $rgeo, $ore, $clay, $obs, $geo, @manu) = split(/,/, $item);
		my $pri = sprintf("%d,%d,%d,%d", $rgeo, $robs, $rclay, $rore);
#		printf("Time %2d: %s\n", $t, join(',', $rore, $rclay, $robs, $rgeo, $ore, $clay, $obs, $geo,$core, $cclay, $cobs, $cgeo, @manu));
		if (defined($cache{$pri}) && ($cache{$pri}+2 < $t)) {
			printf("Skipped cache ($pri seen earlier)\n");
			$cache{$pri} = $t if ($cache{$pri} < $t);
			next;
		}
		$cache{$pri} = $t;

		# Pick the next robot to construct:

my $prod = 0;
		# Try rgeo
		if ($robs) { # && $rore)
			my ($dore, $dclay, $dobs, $dgeo) = ($core, $cclay, $cobs, $cgeo);
			my ($hore, $hclay, $hobs, $hgeo) = ($ore, $clay, $obs, $geo);
			my ($rrore, $rrclay, $rrobs, $rrgeo) = ($rore, $rclay, $robs, $rgeo);
			for (my $tt = $t; $tt < $TLIMIT; $tt++) {
				my $ok = ($hore >= $geoore && $hobs >= $geoobs);
				if ($ok) {
					$hore -= $geoore;
					$hobs -= $geoobs;
					enq($prio,$tt, $rrore, $rrclay, $rrobs, $rrgeo, $hore, $hclay, $hobs, $hgeo, $dore, $dclay, $dobs, $dgeo+1,@manu,"RGEO");
					$prod++;
					last;
				}
				$hore += $rrore;
				$hclay += $rrclay;
				$hobs += $rrobs;
				$hgeo += $rgeo;
				
				$rrore += $dore; $dore = 0;
				$rrclay += $dclay; $dclay = 0;
				$rrobs += $dobs; $dobs = 0;
				$rrgeo += $dgeo; $dgeo = 0;
			}
		}
		# Try robs
		if ($rclay) { # && $rore) 
			my ($dore, $dclay, $dobs, $dgeo) = ($core, $cclay, $cobs, $cgeo);
			my ($hore, $hclay, $hobs, $hgeo) = ($ore, $clay, $obs, $geo);
			my ($rrore, $rrclay, $rrobs, $rrgeo) = ($rore, $rclay, $robs, $rgeo);
			for (my $tt = $t; $tt < $TLIMIT; $tt++) {
				my $ok = ($hore >= $obsore && $hclay >= $obsclay);
				if ($ok) {
					$hore -= $obsore;
					$hclay -= $geoobs;
					enq($prio,$tt, $rrore, $rrclay, $rrobs, $rrgeo, $hore, $hclay, $hobs, $hgeo, $dore, $dclay, $dobs+1, $dgeo,@manu,"ROBS");
					$prod++;
					last;
				}
				$hore += $rrore;
				$hclay += $rrclay;
				$hobs += $rrobs;
				$hgeo += $rgeo;
				
				$rrore += $dore; $dore = 0;
				$rrclay += $dclay; $dclay = 0;
				$rrobs += $dobs; $dobs = 0;
				$rrgeo += $dgeo; $dgeo = 0;
			}
		}
		# Produce rclay
		{
			my ($dore, $dclay, $dobs, $dgeo) = ($core, $cclay, $cobs, $cgeo);
			my ($hore, $hclay, $hobs, $hgeo) = ($ore, $clay, $obs, $geo);
			my ($rrore, $rrclay, $rrobs, $rrgeo) = ($rore, $rclay, $robs, $rgeo);
			for (my $tt = $t; $tt < $TLIMIT; $tt++) {
				my $ok = ($hore >= $clayore);
				if ($ok) {
					$hore -= $clayore;
					enq($prio,$tt, $rrore, $rrclay, $rrobs, $rrgeo, $hore, $hclay, $hobs, $hgeo, $dore, $dclay+1, $dobs, $dgeo,@manu,"RCLAY");
					$prod++;
					last;
				}
				$hore += $rrore;
				$hclay += $rrclay;
				$hobs += $rrobs;
				$hgeo += $rgeo;
				
				$rrore += $dore; $dore = 0;
				$rrclay += $dclay; $dclay = 0;
				$rrobs += $dobs; $dobs = 0;
				$rrgeo += $dgeo; $dgeo = 0;
			}
		}
		# Produce rore
		{
			my ($dore, $dclay, $dobs, $dgeo) = ($core, $cclay, $cobs, $cgeo);
			my ($hore, $hclay, $hobs, $hgeo) = ($ore, $clay, $obs, $geo);
			my ($rrore, $rrclay, $rrobs, $rrgeo) = ($rore, $rclay, $robs, $rgeo);
			for (my $tt = $t; $tt < $TLIMIT; $tt++) {
				my $ok = ($hore >= $oreore);
				if ($ok) {
					$hore -= $oreore;
					enq($prio,$tt, $rrore, $rrclay, $rrobs, $rrgeo, $hore, $hclay, $hobs, $hgeo, $dore+1, $dclay, $dobs, $dgeo,@manu,"ROBS");
					$prod++;
					last;
				}
				$hore += $rrore;
				$hclay += $rrclay;
				$hobs += $rrobs;
				$hgeo += $rgeo;
				
				$rrore += $dore; $dore = 0;
				$rrclay += $dclay; $dclay = 0;
				$rrobs += $dobs; $dobs = 0;
				$rrgeo += $dgeo; $dgeo = 0;
			}
		}
		if ($prod == 0) {
			my ($dore, $dclay, $dobs, $dgeo) = ($core, $cclay, $cobs, $cgeo);
			my ($hore, $hclay, $hobs, $hgeo) = ($ore, $clay, $obs, $geo);
			my ($rrore, $rrclay, $rrobs, $rrgeo) = ($rore, $rclay, $robs, $rgeo);
			for (my $tt = $t; $tt < $TLIMIT; $tt++) {
				$hore += $rrore;
				$hclay += $rrclay;
				$hobs += $rrobs;
				$hgeo += $rgeo;
				
				$rrore += $dore; $dore = 0;
				$rrclay += $dclay; $dclay = 0;
				$rrobs += $dobs; $dobs = 0;
				$rrgeo += $dgeo; $dgeo = 0;
			}
			if ($geo >= $part1) {
				printf("New max = $geo\n");
				$part1 = $geo;
			}
		}
	}
}

foreach (@inp) {
	chomp;
	die("Parse: $_\n") unless (/\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+/);
	($blue, $oreore, $clayore, $obsore, $obsclay, $geoore, $geoobs) = ($1,$2,$3,$4,$5,$6,$7);
	my ($rore, $rclay, $robs, $geo) = (1,0,0,0);
	my $quality = maxgeodes(24, 1,0,0,0, 0,0,0,0);
	printf("Blueprint %d max: %d\n", $blue, $quality);
	die;
	$part1 += $quality*$blue;
}
	
printf("Part1: %s\n", $part1);
printf(STDERR "Total time = %f\n", time - $start);

printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);

