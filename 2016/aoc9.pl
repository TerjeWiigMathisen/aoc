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

my $s = $inp[0];
my $d = '';
while ($s =~ /\((\d+)x(\d+)\)/) {
	$d .= $PREMATCH;
	$s = $POSTMATCH;
	my ($len, $rep) = ($1,$2);
	my $r = substr($s,0,$len);
	$s = substr($s,$len);
	$d .= $r x $rep;
}

sub decomplen
{
	my ($s) = @_;
	my $d = 0;
	my $i = 0;
	my $done = 0;
	while ($s =~ /\((\d+)x(\d+)\)/) {
		printf(STDERR "%6d - > %d\r", length($s), $d) if (($i++ & 0xffff) == 0);
		$d += length($PREMATCH);
		my ($len, $rep) = ($1,$2);
		$s = $POSTMATCH;
		if (length($POSTMATCH) < $len) {
			printf(STDERR "Overlapping repeat, reverting to full decode\n");
			return -1;
		}
		else {
			my $r = substr($POSTMATCH,0,$len);
			$s = substr($POSTMATCH,$len);
			my $sd = decomplen($r);
			if ($sd < 0) {
				$s = $r x $rep.$s;
			}
			else {
				$d += $rep * $sd;
			}
		}
	}
	return $d + length($s);
}

$part1 = length($d);
$part2 = decomplen($inp[0]);

printf(STDERR "Total time = %f\n", time - $start);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
