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

sub maxgeodes
{	
	my @seed = @_;
	my $max = -1;
	my $oldt = 0;
	my $maxrobots = '  ';
	my $maxstr = '';
	my $TLIMIT = $seed[0];
	$seed[0] = 0;
	while (scalar(@seed)) {
		my @next = ();
		my @next1 = ();
		while (scalar(@seed)) {
			my ($t, $rore, $rclay, $robs, $rgeo, $ore, $clay, $obs, $geo, $core, $cclay, $cobs, $cgeo) = splice(@seed,0,13);
			my $pri = sprintf("robots(GOCO) %d,%d,%d,%d stock(GOCO) %d,%d,%d,%d, construction(%d,%d,%d,%D)\n", $rgeo, $robs, $rclay, $rore, 
				reverse($ore, $clay, $obs, $geo), reverse($core, $cclay, $cobs, $cgeo));
			printf("Time %2d %s\n", $t, $pri);
			if (defined($cache{$pri}) && ($cache{$pri} < $t)) {
				printf("Skipped cache (time)\n");
				next;
			}
			$cache{$pri} = $t;
			if (defined($cache{$t}) && ($cache{$t} gt $pri)) {
				printf("Skipped cache (%s > %s)\n", $cache{$t}, $pri);
				next;
			}
			$cache{$t} = $pri;
			
			# Which robots can I construct before stepping the clock?

			my $add = 0;
			for (my $perm = 15; $perm > 0; $perm--) { # As many robots as possible
					#last if ($robots < $maxrobots);

				my ($aore, $aclay, $aobs, $ageo) = (0,0,0,0);
				my ($nore, $nclay, $nobs, $ngeo) = (0,0,0,0);
				if ($perm & 1) {
					$nore += $oreore;
					$aore++;
				}
				if ($perm & 2) {
					$nore += $clayore;
					$aclay++;
				}
				if ($perm & 4) {
					$nore += $obsore;
					$nclay += $obsclay;
					$aobs++;
				}
				if ($perm & 8) {
					$nore += $geoore;
					$nobs += $geoobs;
					$ageo++;
				}
				if ($nore <= $ore && $nclay <= $clay && $nobs <= $obs ) {
					push(@next, $t, $rore, $rclay, $robs, $rgeo, 
						$ore-$nore, $clay-$nclay, $obs-$nobs, $geo, 
						$core+$aore, $cclay+$aclay, $cobs+$aobs, $cgeo+$ageo);
					$add++;
				}
			}
			#next if ($add); # We could create new robots before stepping the clock!
			
			# Try to step forward:
			$ore += $rore;
			$clay += $rclay;
			$obs += $robs;
			$geo += $rgeo;
			$t++;
			# Add the newly constructed robots
			$rore += $core;
			$rclay += $cclay;
			$robs += $cobs;
			$rgeo += $cgeo;
			$core = $cclay = $cobs = $cgeo = 0;
			
			if ($t >= $TLIMIT) {
				if ($geo >= $max) {
					printf("New max: %d geodes\n", $geo);
					$max = $geo;
				}
				next;
			}

			my $pos = sprintf("%2d,%2d,%2d,%2d,%2d,%2d,%2d,%2d", $geo,$obs,$rgeo,$robs,$clay,$rclay,$ore,$rore);
			if (substr($pos,0,11) lt substr($maxrobots,0,11)) {
				printf("Skipping $pos\n");
				next;
			}
			if ($pos ge $maxrobots) {
				$maxrobots = $pos;
				$maxstr = $pos;
				printf("Max = %s\n", $maxrobots);
			}
			push(@next1, $t, $rore, $rclay, $robs, $rgeo, 
						$ore, $clay, $obs, $geo, 
						0,0,0,0);
		}
		@seed = (@next,@next1);
	}
	return $max;
}

foreach (@inp) {
	chomp;
	die("Parse: $_\n") unless (/\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+/);
	($blue, $oreore, $clayore, $obsore, $obsclay, $geoore, $geoobs) = ($1,$2,$3,$4,$5,$6,$7);
	my ($rore, $rclay, $robs, $geo) = (1,0,0,0);
	my $quality = maxgeodes(24, 1,0,0,0, 0,0,0,0, 0,0,0,0);
	printf("Blueprint %d max: %d\n", $blue, $quality);
	die;
	$part1 += $quality*$blue;
}
	
printf("Part1: %s\n", $part1);
printf(STDERR "Total time = %f\n", time - $start);

printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);

