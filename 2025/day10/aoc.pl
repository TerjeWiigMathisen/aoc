#!perl -w
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

while (<>) {
	chomp;
	push(@lines, $_);
}

sub indre
{
	my ($s) = @_;
	$s = substr($s,1,length($s)-2);
	return $s;
}

sub list2bits
{
	my ($l) = @_;
	my $bits = 0;
	foreach (split(/,/,$l)) {
		$bits |= (1 << $_);
	}
	return $bits;

}	
sub targetbits
{
	my ($l) = @_;
	my $bits = 0;
	my $b = 1;
	foreach (split(//,$l)) {
		$bits += $b if (/#/);
		$b += $b;
	}
	return $bits;
}

sub bitcnt
{
	my ($b) = @_;
	my $cnt = 0;
	while ($b) {
		$cnt++;
		$b &= $b-1;
	}
	return $cnt;
}

#part1

sub solve
{
	my ($line) = @_;
	my @parts = split(/ /,$line);
	my $target = indre(shift @parts);
	my $len = scalar(@parts); # Number of button combinations we can flip
	$target = targetbits($target);
	my $jolts = indre(pop @parts);
	my @jolts = split(/,/,$jolts);
	my $joltparity = 0;
	foreach (reverse @jolts) {
		$joltparity += $joltparity + ($_ & 1);
	}
		
	my $p = 0;
	my @solutions = ();
	my $permutations = 1 << scalar(@parts);
	foreach (@parts) {
		#printf("$_");
		$_ = indre($_);
		$_ = list2bits($_);
		$parts[$p] = $_;
		#printf(" -> %x\n", $_);
		$p++;
	}
	my $best = 0xffff;
	for (my $t = 1; $t < $permutations; $t++) {
		my $lights = 0;
		for (my $bit = 0; $bit < scalar(@parts); $bit++) {
			if ($t & (1<<$bit)) {
				$lights ^= $parts[$bit];
			}
		}
		if ($lights == $target) {
#			printf("Possible solution: %x\n",$t);
			my $buttons = bitcnt($t);
			if ($buttons < $best) {
				$best = $buttons;
			}
		}
	}
#	printf("$line\nBest: %d\n", $best);
	return $best;
}

my @button_bits;
my @button_str;
my %cache = ();
my $cache_entries = 0;
my $cache_hits = 0;

my @keys = ();

sub solve_recurse
{
	my (@jolts) = @_;
	my $key = join(",",@jolts);
	my $r = $cache{$key};
	if (defined($r)) { 
		$cache_hits++;
		return $r;
	}
	#printf("Start $key\n");
	
	my $joltparity = 0;
	foreach (reverse @jolts) {
		$joltparity += $joltparity + ($_ & 1);
	}
	my $p = 0;
	my @solutions = ();
	my $permutations = 1 << scalar(@button_str);
	for (my $t = 0; $t < $permutations; $t++) {
		my $lights = 0;
		for (my $bit = 0; $bit < scalar(@button_bits); $bit++) {
			if ($t & (1<<$bit)) {
				$lights ^= $button_bits[$bit];
			}
		}
		if ($lights == $joltparity) {
#			printf("Partial part2 solution: %d %s\n",$t, $key);
			push(@solutions, $t);
		}
	}
	my $best = 1e9;
	my $best_start;
	foreach (@solutions) {
		my $sol = $_;
		my $start_buttons = 0;
		my @remjolts = @jolts;
		my $bit = 1;
		for (my $j = 0; $j < scalar(@button_str); $j++) {
			if ($sol & $bit) {
				$start_buttons++;
				foreach (split(/,/,$button_str[$j])) {
					$remjolts[$_]--;
				}
			}
			$bit += $bit;
		}
		my $nonzero = 0;
		my $negative = 0;
		foreach (@remjolts) {
			$negative += $_ < 0;
			$_ >>= 1;
			$nonzero += $_ != 0;
		}
		next if ($negative);
		my $res = $start_buttons;
		if ($nonzero) {
			$res += 2*solve_recurse(@remjolts);
		}
		if ($res < $best) {
			$best = $res;
			$best_start = $start_buttons;	
		}
	}
	$cache{$key} = $best;
	$cache_entries++;
	#printf("$key -> $best_start $best\n");
	return $best;
}

sub solve2
{
	my ($line) = @_;
	#printf("$line\n");
	my @parts = split(/ /,$line);
	my $target = indre(shift @parts);
	my $len = scalar(@parts); # Number of button combinations we can flip
	$target = targetbits($target);
	my $jolts = indre(pop @parts);
	my @jolts = split(/,/,$jolts);
	%cache = ();
	@button_bits = ();
	@button_str = ();
	
	my $p = 0;
	foreach (@parts) {
		$_ = indre($_);
		push(@button_str, $_);
		$_ = list2bits($_);
		push(@button_bits, $_);
	}
	
	my @solution = ();
	$p = solve_recurse(@jolts);
	#printf("$line\n$p\n");
	return $p;
}

foreach (@lines) {
	printf("$_");
	my $t0 = time;
	$part1 += solve($_);
	$part2 += solve2($_);
	$t0 = time - $t0;
	printf(" -> %5.3fms\n", $t0*1000);
}

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
printf("Total cache entries: %d, hits = %d\n", $cache_entries, $cache_hits);
