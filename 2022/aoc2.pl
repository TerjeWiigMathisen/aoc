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
open(F ,'<',$fname);

my $start = time;

my %x2p = ('X' => 1, 'Y' => 2, 'Z' => 3);
my %win = ('AX' => 3, 'AY' => 6, 'AZ' => 0, 'BX' => 0, 'BY' => 3, 'BZ' => 6, 'CX' => 6, 'CY' => 0, 'CZ' => 3);
my %end = ('AX' => 'Z', 'AY' => 'X', 'AZ' => 'Y', 'BX' => 'X', 'BY' => 'Y', 'BZ' => 'Z', 'CX' => 'Y', 'CY' => 'Z', 'CZ' => 'X');

my @p1 = ();
my @p2 = ();
foreach my $x ('X'..'Z') {
	push(@p1,0);
	push(@p2,0);
	foreach my $a ('A','B','C'){
		my $part1 = $x2p{$x} + $win{$a.$x};
		my $y = $end{$a.$x};
		my $part2 = $x2p{$y} + $win{$a.$y};
		push(@p1,$part1);
		push(@p2,$part2);
	}
}
push(@p1,0,0,0,0);
push(@p2,0,0,0,0);

printf("let part1tab:[u8;32] = [%s];\n", join(",", @p1, @p1));
printf("let part2tab:[u8;32] = [%s];\n", join(",", @p2, @p2));

die();

my $part1 = 0;
my $part2 = 0;

foreach (<F>) {
	chomp;
	my ($a, $x) = split;
	
	$part1 += $x2p{$x} + $win{$a.$x};
	my $y = $end{$a.$x};
	$part2 += $x2p{$y} + $win{$a.$y};
}
close(F);


printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
