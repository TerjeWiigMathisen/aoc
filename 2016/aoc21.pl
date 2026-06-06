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
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

my $part1 = "abcdefgh";
#my $part1 = "abcde";
my $part2 = 0;

sub rotate_left
{
	my ($st, $n) = @_;
	$n %= length($st);
	return substr($st,$n).substr($st,0,$n);
}

sub rotate_right
{
	my ($st, $n) = @_;
	$n %= length($st);
	return rotate_left($st,length($st)-$n);
}

sub scramble
{
	my ($part1) = @_;
	foreach (@inp) {
		if (/swap position (\S+) with position (\S+)/) {
			my ($f,$s) = (substr($part1,$1,1),substr(($part1,$2,1)));
			(substr($part1,$1,1),substr($part1,$2,1)) = ($s,$f);
		}
		elsif (/swap letter (\S+) with letter (\S+)/) {
			my $cmd = sprintf(q($part1=~tr/%s%s/%s%s/;),$1,$2,$2,$1);
			eval($cmd);
		}
		elsif (/rotate left (\S+) step/) {
			$part1 = rotate_left($part1,$1);
		}
		elsif (/rotate right (\S+) step/) {
			$part1 = rotate_right($part1, $1);
		}
		elsif (/rotate based on position of letter (\S+)/) {
			my $p = index($part1, $1);
			$part1 = rotate_right($part1, $p + 1 + ($p >= 4));
		}
		elsif (/reverse positions (\S+) through (\S+)/) {
			my ($b, $e) = ($1,$2);
			my $l = $e-$b+1;
			substr($part1,$b,$l) = reverse substr($part1,$b,$l);
		}
		elsif (/move position (\S+) to position (\S+)/) {
			my $p = substr($part1,$1,1);
			substr($part1,$1,1) = '';
			substr($part1,$2,0) = $p;
		}
		else {
			die("Bad input: $_");
		}
	}
	return $part1;
}

$part2 = "fbgdceah";
#$part2 = $part1;

sub unscramble
{
	my ($part2) = @_;
	foreach (reverse @inp) {
		printf("%s - %s\n", $part2, $_);
		if (/swap position (\S+) with position (\S+)/) {
			my ($f,$s) = (substr($part2,$1,1),substr(($part2,$2,1)));
			(substr($part2,$1,1),substr($part2,$2,1)) = ($s,$f);
		}
		elsif (/swap letter (\S+) with letter (\S+)/) {
			my $cmd = sprintf(q($part2=~tr/%s%s/%s%s/;),$1,$2,$2,$1);
			eval($cmd);
		}
		elsif (/rotate left (\S+) step/) {
			$part2 = rotate_right($part2,$1);
		}
		elsif (/rotate right (\S+) step/) {
			$part2 = rotate_left($part2, $1);
		}
		elsif (/rotate based on position of letter (\S+)/) {
			my $p = $part2;
			my $l = $1;
			my $p2;
			for (my $r = 0; $r < length($part2); $r++) {
				$part2 = rotate_left($p,$r);
				my $i = index($part2, $l);
				$i = ($i + 1 + ($i >= 4)) % length($part2);
				if ($i == $r) {
					printf("Rot = %d\n",$r);
					if (defined($p2)) {
						printf("Found another possibility!\n");
					}
					$p2 = $part2;
				}
			}
			$part2 = $p2;
		}
		elsif (/reverse positions (\S+) through (\S+)/) {
			my ($b, $e) = ($1,$2);
			my $l = $e-$b+1;
			substr($part2,$b,$l) = reverse substr($part2,$b,$l);
		}
		elsif (/move position (\S+) to position (\S+)/) {
			my $p = substr($part2,$2,1);
			substr($part2,$2,1) = '';
			substr($part2,$1,0) = $p;
		}
		else {
			die("Bad input: $_");
		}
	}
	return $part2;
}

$part1 = scramble($part1);

my $p2 = "fbgdceah";
$part2 = unscramble($p2);
my $p3 = scramble($part2);

printf("%s - %s - %s\n", $p2, $part2, $p3);

printf(STDERR "Total time = %f\n", time - $start);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);

