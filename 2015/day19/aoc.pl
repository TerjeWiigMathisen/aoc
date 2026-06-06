#!perl -w

# Best runtime: 71.872 ms

use strict;
use Time::HiRes qw (time);
use List::PriorityQueue;

use warnings;

my %generate = ();
my %compress = ();

my $target;
my %targets = ();

my $time = time;

while (<>) {
	chomp;
	if (/^(\S+)\s+=\>\s+(\S+)$/) {
		my ($fra, $til) = ($1, $2);
		push(@{$generate{$fra}},$til);
		$compress{$til} = $fra;
		next;
	}
	next unless ($_);
	$target = $_;
}

my %seen = ();

foreach (keys %generate) {
	my $k = $_;
	my $pos = 0;
	while (1) {
		$pos = index($target,$k,$pos);
		last if ($pos < 0);
		my @gens = @{$generate{$k}};
		foreach (@gens) {
			my $t = $target;
			substr($t,$pos,length($k), $_);
			$seen{$t}++;
		}
		$pos++;
	}
}

#foreach (keys %seen) {
#  if ($seen{$_} > 1) {
#		printf("Key $_ seen %d times\n", $seen{$_});
#	}
#}
my $part1 = scalar(keys %seen);
%seen = ();
my $best = 1e38;

my $skip = 0;
my $iter = 0;

sub searchdown
{
	my ($bottom, $state) = @_;
	my $pq = new List::PriorityQueue;
	my $levels = 0;
	my $best = 1e38;
	$pq->insert(join("\t", $levels, $state), 0);
	my @compress_by_size = keys %compress; #reverse sort {length($a) <=> length($b)} (keys %compress);
	while (my $key = $pq->pop()) {
		$iter++;
		($levels, $state) = split(/\t/, $key);
		#next if ($levels >= $best);
		if (defined($seen{$state}) && $seen{$state} <= $levels) {
			#$skip++;
			next;
		}
		$seen{$state} = $levels;
		#printf("%4d %d:%s\n", $levels, length($state), substr($state,0,30)."...".substr($state,-30)); # if (length($state) < 50);
		if ($state eq $bottom) {
#			return $levels;
			if ($levels < $best) {
				$best = $levels;
				my $pqlen = 0;
				while ($pq->pop()) { $pqlen++; }
				printf("Found solution with $levels substitutions\npq size = %d\n", $pqlen);
				return $best;
			}
			#return $levels;
		}
		$levels++;
		foreach (@compress_by_size) {
			my $k = $_;
			my $fra = $compress{$k};
			my $pos = index($state,$k,0);
			while ($pos >= 0) {
				my $t = $state;
				substr($t,$pos,length($k)) = $fra;
				$pos = index($state,$k,$pos+1);
				$pq->insert(join("\t", $levels, $t), $levels+length($t)*1000);
			}
		}
	}
}

my $part2 = searchdown("e", $target);

$time = time - $time;

printf("part1 = %d\n", $part1);
printf("part2 = %d\n", $part2);
printf("Total time: %1.3f ms\n", $time*1000.0);

#printf("Total iterations: $iter, skipped states: $skip\n");
