#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
#use Data::Dumper;
#use Algorithm::Permute qw (permute);
#use List::Util qw (reduce);

#use bigint;

#use JSON::Parse;
#no warnings 'recursion';

my $DEBUG = 0;

my $fname = shift;
open(F ,'<',$fname);

my $start = time;

my $part1 = 0;
my $part2 = 0;

my @stacks = ();

my @lines = ();
foreach (<F>) {
	chomp;
	push(@lines, $_);
}
close(F);

my $line = 0;
foreach (@lines) {
	$line++;
#	printf(STDERR "%s\n", $_);
	last if (/^\s+1\s+2\s+3/);

	my $s = 0;
	while (length($_)) {
		my $c = substr($_,1,1);
		$_ = length($_) > 4 ? substr($_,4) : '';
		$stacks[$s] = '' unless (defined($stacks[$s]));
		if ($c ne ' ') {
			$stacks[$s] .= $c;
		}
		$s++;
	}
#	printf(STDERR "%d %s\n", scalar(@stacks), join(",", @stacks));
}
$line++; # Skip blank line

#printf(STDERR "Start moving from line $line\n");
my @save = @stacks;
foreach ((@lines)[$line..scalar(@lines)-1]) {
	if (/move\s+(\d+)\s+from\s+(\d+)\s+to\s+(\d+)/) {
		my ($n, $f, $t) = ($1,$2-1,$3-1);
		for (my $i = 0; $i < $n; $i++) {
			my $c = substr($stacks[$f],0,1);
			$stacks[$f] = substr($stacks[$f],1);
			$stacks[$t] = $c . $stacks[$t];
		}
	}
}
$part1 = ''; foreach (@stacks) { $part1 .= substr($_,0,1); }

#printf(STDERR "Start moving 9001 from line $line\n");
@stacks = @save;
foreach ((@lines)[$line..scalar(@lines)-1]) {
	if (/move\s+(\d+)\s+from\s+(\d+)\s+to\s+(\d+)/) {
		my ($n, $f, $t) = ($1,$2-1,$3-1);
		my $c = substr($stacks[$f],0,$n);
		$stacks[$f] = substr($stacks[$f],$n);
		$stacks[$t] = $c . $stacks[$t];
	}
}
$part2 = ''; foreach (@stacks) { $part2 .= substr($_,0,1); }

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
