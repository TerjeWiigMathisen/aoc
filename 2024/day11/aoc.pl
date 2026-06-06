#!perl -w
use strict;
use Time::HiRes qw (time);
use English;
use warnings;

#use bigint;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

while (<>) {
	chomp;
	push(@lines, $_);
}

my @stones = split(/ /, $lines[0]);

sub step
{
	my (%stones) = @_;
	my %out = ();
	foreach (keys %stones) {
		my $cnt = $stones{$_};
		my $l = length($_);
		if (($l & 1) == 0) {
			$l >>= 1;
			$out{substr($_,0,$l)} += $cnt;
			$out{substr($_,$l)+0} += $cnt; # Removes leading zeroes!
		}
		elsif ($_ == 0) {
			$out{1} += $cnt;
		}
		else {
			$out{$_*2024} += $cnt;
		}
	}
	return %out;
}

my %inp = ();
foreach (@stones) { $inp{$_}++; }

for (my $b = 1; $b <= 75; $b++) {
	%inp = step(%inp);
	if ($b == 25 || $b == 75) {
		my $s = 0;
		foreach (keys %inp) { $s += $inp{$_}; }
		if ($b == 25) { $part1 = $s; }
		$part2 = $s;
	}
}

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);

my $max = 0;
for (my $s = 1; $s < 1; $s++) {
	my $r = int(rand(1e12));
	%inp = ($r,1);
	my $prev = 0;
	for (my $b = 1; $b < 200; $b++) {
		%inp = step(%inp);
		my $sum_keys = 0;
		foreach (keys %inp) { $sum_keys += $_; }
		if ($sum_keys == $prev) {
			printf("%15d: %d stone values after %d generations\r", 
				$r, scalar(keys %inp), $b);
			if ($b > $max) {
				printf("MAX %15d: %d stone values after %d generations\n", 
				$r, scalar(keys %inp), $b);
				$max = $b;
			}
			last;
		}
		$prev = $sum_keys;
	}
}

printf("%d total stone values\n", scalar(keys %inp));

my @sorted_stones = sort {$a <=> $b} keys %inp;
#printf("%s\n",join(" ",@sorted_stones));

my $W = 14;
my $W2 = $W*2;

my $XXH_PRIME32_2 = 0x85EBCA77;
my $XXH_PRIME32_3 = 0xC2B2AE3D;

sub hash
{
	my ($n) = @_;
	
	#$n = ($n & 0xfffff)*$XXH_PRIME32_2 ^ ($n >> $W);
	$n ^= ($n >> $W) + ($n >> $W2);
	$n &= (1<<$W)-1;

#    $n ^= $n >> 15;
#	$n &= 0xffffff;
#	$n *= $XXH_PRIME32_2;
#	$n ^= $n >> 13;
#	$n *= $XXH_PRIME32_3;
#	$n ^= $n >> 16;
#	$n &= (1 << $W) - 1;

	return $n;
}

my %seen = ();
my $coll = 0;

foreach (@sorted_stones) {
	my $n = $_;
	my $h = hash($n);
	if (defined($seen{$h})) {
#		printf("Duplicate $n and %d -> $h\n",$seen{$h});
		$coll++;
	}
	$seen{$h} = $n;
}
printf("Found %d unique hash results ($coll collisions)\n",scalar(keys %seen));
	