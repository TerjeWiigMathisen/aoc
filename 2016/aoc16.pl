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

#my $fname = shift;
#open(F ,'<',$fname);
#my @inp = (<F>);
#chomp(@inp);

my $part1;
my $part2;

sub dragon
{
	my ($s, $len) = @_;
	while (length($s) < $len) {
		my $rs = reverse $s;
		$rs =~ tr/01/10/;
		$s .= '0'.$rs;
	}
	return substr($s,0,$len);
}

sub check_org
{
	my ($s) = @_;
	while ((length($s) & 1) == 0) {
		my $o = '';
		for (my $i = 1; $i < length($s); $i+=2) {
			$o .= 1 ^ substr($s,$i-1,1) ^ substr($s,$i,1);
		}
		$s = $o;
	}
	return $s;
}

# 00 -> 1
# 01 -> 0
# 10 -> 0
# 11 -> 1

# 0000 -> 1
# 0001 -> 0
# 0010 -> 0
# 0011 -> 1
# 0100 -> 0
# 0101 -> 1
# 0110 -> 1
# 0111 -> 0

# Result is 1^(parity(inp))

sub check
{
	my ($s) = @_;
	my $div = 1;
	while ((length($s) & $div) == 0) {
		$div += $div+1;
	}
	$div++;
	$div >>= 1;
	my $o = '';
	for (my $i = 0; $i < length($s); $i += $div) {
		my $p = 1;
		for (my $j = $i; $j < $i+$div; $j++) {
			$p ^= substr($s,$j,1);
		}
		$o .= $p;
	}
	return $o;
}

my $dragon = dragon("11011110011011101",272);
printf("$dragon\n");
$part1 = check($dragon);

$dragon = dragon("11011110011011101",35651584);
my $td = time;
$part2 = check($dragon);

my $tt = time;

printf(STDERR "Total time = %f\n", $tt - $start);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
printf("Part2 check = %f\n", $tt - $td);