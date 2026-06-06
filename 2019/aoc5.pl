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
open(F,$fname) || die("Open");
my @code = (<F>);
close(F);

@code = split(/,/, $code[0]);
my $ip = 0;

my $part1 = 0;
my $part2 = 0;

my @inputs;
while (my $arg = shift) {
	push(@inputs,$arg);
}

my @outputs;

sub runcode
{
	my (@code) = @_;
	$ip = 0;
	while (1) {
		# printf(STDERR "%d: %s\n", $ip, join(',',@code));
		my $op = '000'.$code[$ip];
		my $mode = substr($op,0,length($op)-2);
		$op = substr($op,-2,2);
		last if ($op == 99);

		my $s1 = $code[$ip+1];
		my $s2 = $code[$ip+2];
		my $d = $code[$ip+3];
		
		printf("IP=%3d, OP=%2d, MODE=%02d, S1=%d, S2=%d, D=%d\n", $ip, $op, $mode, $s1, $s2, $d);
		if ($op == 1) {
			$code[$d] = (substr($mode,-1,1) == 0 ? $code[$s1] : $s1) + 
						(substr($mode,-2,1) == 0 ? $code[$s2] : $s2);
			$ip += 4;
		}
		elsif ($op == 2) {
			$code[$d] = (substr($mode,-1,1) == 0 ? $code[$s1] : $s1) * 
						(substr($mode,-2,1) == 0 ? $code[$s2] : $s2);
			$ip += 4;
		}
		elsif ($op == 3) {
			$code[$s1] = shift @inputs;
			$ip += 2;
		}
		elsif ($op == 4) {
			push(@outputs, $code[$s1]);
			printf("Output: %d\n", $outputs[-1]);
			$ip += 2;
		}
		elsif ($op == 5) { # JT
			if ((substr($mode,-1,1) == 0 ? $code[$s1] : $s1)) {
				$ip = (substr($mode,-2,1) == 0 ? $code[$s2] : $s2);
			}
			else {
				$ip += 3;
			}
		}
		elsif ($op == 6) { # JF
			if ((substr($mode,-1,1) == 0 ? $code[$s1] : $s1)==0) {
				$ip = (substr($mode,-2,1) == 0 ? $code[$s2] : $s2);
			}
			else {
				$ip += 3;
			}
		}
		elsif ($op == 7) { # LT
			$code[$d] = (substr($mode,-1,1) == 0 ? $code[$s1] : $s1) < 
						(substr($mode,-2,1) == 0 ? $code[$s2] : $s2);
			$ip += 4;
		}
		elsif ($op == 8) { # EQ
			$code[$d] = (substr($mode,-1,1) == 0 ? $code[$s1] : $s1) ==
						(substr($mode,-2,1) == 0 ? $code[$s2] : $s2);
			$ip += 4;
		}
		else {
			die("Bad opcode $op at IP=$ip");
		}
	}
	return $code[0];
}

runcode(@code);
$part1 = $outputs[-1];

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
