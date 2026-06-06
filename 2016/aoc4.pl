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
my $part2;

sub decrypt
{
	my ($n, $r) = @_;
	my $d = "";
	foreach (split(//,$n)) {
		$d .= chr((ord($_)-ord('a') + $r) % 26 + ord('a'));
	}
	return $d;
}

foreach (@inp) {
	s/\-//g;
	if (/^(\D+)(\d+)\[(\S+)\]$/) {
		my ($name, $sector,$check) = ($1,$2,$3);
		my %letters = ();
		foreach (split(//,$name)) {
			$letters{$_}++;
		}
		my $key = join("", sort { sprintf("%3d%s", 1000-$letters{$a},$a) cmp sprintf("%3d%s", 1000-$letters{$b},$b) } keys %letters);
		if (substr($key,0,5) eq $check) {
			$part1 += $sector;
			my $rot = $sector % 26;
			my $decrypt = decrypt($name,$rot);
			#printf("%s %d %s\n", $decrypt, $sector, $check);
			if ($decrypt =~ /northpole/i) {
				$part2 = $sector;
			}
		}
	}
}
		
printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
