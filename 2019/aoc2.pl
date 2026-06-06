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

my @code = (<>);
@code = split(/,/, $code[0]);
my $ip = 0;

my $part1 = 0;
my $part2 = 0;

sub runcode
{
	my ($noun, $verb, @code) = @_;
	$code[1] = $noun; $code[2] = $verb;
	$ip = 0;
	while (1) {
		# printf(STDERR "%d: %s\n", $ip, join(',',@code));
		my $op = $code[$ip];
		last if ($op == 99);
		my $s1 = $code[$ip+1];
		my $s2 = $code[$ip+2];
		my $d = $code[$ip+3];
		if ($op == 1) {
			$code[$d] = $code[$s1]+$code[$s2];
		}
		elsif ($op == 2) {
			$code[$d] = $code[$s1]*$code[$s2];
		}
		else {
			die("Bad opcode $op at IP=$ip");
		}
		$ip += 4;
	}
	return $code[0];
}

$part1 = runcode(12,2,@code);
for (my $v = 0; $v <= 99; $v++) {
	my $res;
	for (my $n = 0; $n <= 99; $n++) {
		my $res = runcode($n, $v, @code);
		printf(STDERR "%3d %3d %d\t", $n, $v, $res);
		if ($res == 19690720) {
			$part2 = $n*100+$v;
			last;
		}
	}
	last if ($part2);
}

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
