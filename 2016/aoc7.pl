#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);

#use bigint;

#use JSON::Parse;
#no warnings 'recursion';

my $start = time;

my $DEBUG = 0;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

my $part1 = 0;
my $part2 = 0;

foreach (@inp) {
	my @c = split(//);
	my @abba = ();
	for (my $i = 3; $i < scalar(@c); $i++) {
		if ($c[$i-3] ne $c[$i-2] && $c[$i-3] eq $c[$i] && $c[$i-2] eq $c[$i-1]) {
			push(@abba, $i-3);
		}
	}
	if (scalar(@abba)) {
		my $abba = 1;
		my $openbracket = index($_,'[');
		do {
			if ($openbracket >= 0) {
				my $closebracket = index($_,']',$openbracket+1);
				if ($closebracket) {
					foreach (@abba) {
						if ($_ > $openbracket && $_ < $closebracket) {
							$abba = 0;
							last;
						}
					}
				}
				$openbracket = index($_,'[',$closebracket+1);
			}
		} while ($openbracket >= 0);
		$part1 += $abba;
	}
}

foreach (@inp) {
	my @c = split(//);
	my $key = $_;
	my %aba = ();
	my %bab = ();
	my $bracket = 0;

	my $c = $c[0]; my $n = $c[1];
	if ($c eq '[') {
		$bracket = 1;
		if ($n eq ']') {
			$bracket = 0;
		}
		elsif ($n eq '[') {
			$bracket = 2;
		}
	}
	elsif ($n eq '[') {
		$bracket = 1;
	}
		
	for (my $i = 2; $i < scalar(@c); $i++) {
		my $p = $c; $c = $n; $n = $c[$i];
		if ($c eq '[') {
			$bracket++;
		}
		elsif ($c eq ']') {
			$bracket--;
		}
		elsif ($p ne $c && $p eq $n) {
			if ($bracket) {
				my $bab = $c.$p.$c;
				$bab{$bab}++;
				#printf("Found BAB: %s\n",$bab);
			}
			else {
				my $aba = $p.$c.$n;
				$aba{$aba}++;
				#printf("Found ABA: %s\n",$aba);
			}
		}
	}
	foreach (keys %aba) {
		if (defined($bab{$_})) {
			$part2++;
			#printf("%s in %s\n",$_, $key);
			last;
		}
	}
}

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
