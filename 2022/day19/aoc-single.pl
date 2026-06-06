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

sub maxgeodes
{	
	my @seed = @_;
	my $max = -1;
	my $oldt = 0;
	my $maxrobots = 0;
	my $maxstr = '';
	while (scalar(@seed)) {
		my @next = ();
		while (scalar(@seed)) {
			my ($t, $rore, $rclay, $robs, $rgeo, $ore, $clay, $obs, $geo) = splice(@seed,0,9);
			if ($t != $oldt) {
				printf("%d perm at beginning of %d min: prev best %s\n", (scalar(@seed)/9)+1, 25-$t, $maxstr );
				$oldt = $t;
			}
			printf("%s\n", join(',',25-$t, reverse ($rore, $rclay, $robs, $rgeo, $ore, $clay, $obs, $geo)));

			my $pos = $rgeo*1000000+$robs*10000+$rclay*100+$rore;
			if ($pos >= $maxrobots) {
				$maxrobots = $pos;
				$maxstr = sprintf("robots(%d,%d,%d,%d) store(%d,%d,%d,%d)", $rgeo, $robs, $rclay, $rore, $geo, $obs, $clay, $ore);
			}
			if ($pos + 100*$rore < $maxrobots) {
				next;
			}
			
			if ($t <= 0) {
				if ($geo >= $max) {
					printf("New max: %d geodes\n", $geo);
					$max = $geo;
				}
				next;
			}
			# Which robots can I construct?
			
			#my @perm = ();
			my $maxp = 0;
			for (my $perm = 15; $perm >= 0; $perm--) { # As many robots as possible
				#last if ($robots < $maxrobots);

				my ($nore, $nclay, $nobs, $ngeo) = (0,0,0,0);
				#my ($dore, $dclay, $dobs, $dgeo) = (0,0,0,0);
				my ($newore, $newclay, $newobs, $newgeo) = (0,0,0,0);
				if ($perm & 1) {
					$nore += $oreore;
					$newore++;
				}
				if ($perm & 2) {
					$nore += $clayore;
					$newclay++;
				}
				if ($perm & 4) {
					$nore += $obsore;
					$nclay += $obsclay;
					$newobs++;
				}
				if ($perm & 8) {
					$nore += $geoore;
					$nobs += $geoobs;
					$newgeo++;
				}
				if ($nore <= $ore && $nclay <= $clay && $nobs <= $obs ) {
					last if (($perm & 12) < $maxp);
					$maxp = $perm & 12;
					push(@next, $t-1, $rore+$newore, $rclay+$newclay, $robs+$newobs, $rgeo+$newgeo, 
					  $ore-$nore+$rore, $clay-$nclay+$rclay, $obs-$nobs+$robs, $geo+$rgeo);
					#push(@perm,$perm);
				}
			}
		}
		@seed = @next;
	}
	return $max;
}

foreach (@inp) {
	chomp;
	die("Parse: $_\n") unless (/\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+/);
	($blue, $oreore, $clayore, $obsore, $obsclay, $geoore, $geoobs) = ($1,$2,$3,$4,$5,$6,$7);
	my ($rore, $rclay, $robs, $geo) = (1,0,0,0);
	my $quality = maxgeodes(24, 1,0,0,0,0,0,0,0);
	printf("Blueprint %d max: %d\n", $blue, $quality);
	$part1 += $quality*$blue;
}
	
printf("Part1: %s\n", $part1);
printf(STDERR "Total time = %f\n", time - $start);

printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);

