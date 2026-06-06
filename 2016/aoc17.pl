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

my $part1;
my $part2 = 0;

my $PASSCODE = "ihgpwlah";

sub path
{
	my ($PASSCODE) = @_;
	my $pq = new List::PriorityQueue;
	$pq->insert("0,0,",0);
	while (my $key = $pq->pop()) {
		my ($x,$y,$moves) = split(/,/,$key);
		my $h = substr(Digest::MD5::md5_hex(sprintf("%s%s",$PASSCODE, $moves)),0,4);
#		printf(STDERR "%s %s\n", $key, $h);
		if ($x == 3 && $y == 3) {
			$part1 = $moves unless (defined($part1));
			if (length($moves) > $part2) {
				$part2 = length($moves);
#				printf("part1 = %s, part2 = %d\n",$part1, $part2);
			}
			next;
		}
		if ($y > 0 && substr($h,0,1) ge 'b') {
			$pq->insert(sprintf("%d,%d,%s",$x,$y-1,$moves.'U'),length($moves)+1);
		}
		if ($y < 3 && substr($h,1,1) ge 'b') {
			$pq->insert(sprintf("%d,%d,%s",$x,$y+1,$moves.'D'),length($moves)+1);
		}
		if ($x > 0 && substr($h,2,1) ge 'b') {
			$pq->insert(sprintf("%d,%d,%s",$x-1,$y,$moves.'L'),length($moves)+1);
		}
		if ($x < 3 && substr($h,3,1) ge 'b') {
			$pq->insert(sprintf("%d,%d,%s",$x+1,$y,$moves.'R'),length($moves)+1);
		}
	}
	printf("No more paths found!\n");
}

#path("ihgpwlah"); # 370
$part2 = 0;
path("qtetzkpl");

printf(STDERR "Total time = %f\n", time - $start);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
