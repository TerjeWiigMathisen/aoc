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

sub enq
{
	my ($pq, $t, $rore, $rclay, $robs, $rgeo, $ore, $clay, $obs, $geo, @manu) = @_;
	my $dt = 24-$t;
	$dt = 2 if ($dt <= 1);
	my $pri = 1e12-($rore*$dt +$ore)*1 -($rclay*$dt+$clay)*100 - 
		($robs*$dt+$obs)*1e4 - ($rgeo*$dt+$geo)*1e6;
	$pri = $t;
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
		printf("Time %2d: %s\n", $t, join(',', $rore, $rclay, $robs, $rgeo, $ore, $clay, 		$obs, $geo, @manu)) if ($t < 17);

		if (!defined($fgeo{$geo})) {
			printf("First geo(%d)=%d\n",$geo, $t);
			$fgeo{$geo} = $t;
		}
		elsif ($t < $fgeo{$geo}) {
			printf("Faster to geo(%d)=%d\n",$geo, $t);
			$fgeo{$geo} = $t;
			next;
		}
		elsif ($t > $fgeo{$geo}+3 && $fgeo{1}) {
			#printf("Skipped late geo(%d)=%d\n",$geo, $fgeo{$geo}) if ($geo);
			next;
		}

		if (!defined($fobs{$obs})) {
			printf("First obs(%d)=%d\n",$obs, $t);
			$fobs{$obs} = $t;
		}
		elsif ($t < $fobs{$obs}) {
			printf("Faster to obs(%d)=%d\n",$obs, $t);
			$fobs{$obs} = $t;
			next;
		}
		elsif ($t > $fobs{$obs}+20 && $fobs{1}) {
			#printf("Skipped late obs(%d)=%d\n",$obs, $fobs{$obs}) if ($obs);
			next;
		}

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
		if ($rclay) { # && $rore) 
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
		{
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
			if ($hgeo > $part1) {
				printf("New max = %s\n", join(',', $rore, $rclay, $robs, $rgeo, $hore, $hclay, $hobs, $hgeo,@manu));
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
	$part1 += $quality*$blue;
	die;
}
	
printf("Part1: %s\n", $part1);
printf(STDERR "Total time = %f\n", time - $start);

printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);

