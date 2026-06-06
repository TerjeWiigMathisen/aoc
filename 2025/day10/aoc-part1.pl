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

sub solve
{
	my ($line) = @_;
	my @parts = split(/ /,$line);
	my $target = indre(shift @parts);
	my $len = scalar(@parts); # Number of button combinations we can flip
	$target = targetbits($target);
	my $jolts = indre(pop @parts);
	my $p = 0;
	foreach (@parts) {
		#printf("$_");
		$_ = indre($_);
		$_ = list2bits($_);
		$parts[$p] = $_;
		#printf(" -> %x\n", $_);
		$p++;
	}
	#printf("line:   $line\ntarget: %x\nbuttons = (%s)\n", $target, join(',', map {sprintf("%x",$_)} @parts));
	# Try all permutations of 0 or 1 press:
	my $max = 1 << ($len-1);
	
	my $best = 1e38;
	for (my $i = 0; $i < $max; $i++) {
		my $res = 0;
		my $bit = 1;
		for (my $j = 0; $j < $len; $j++) {
			if ($i & $bit) {
				$res ^= $parts[$j];
			}
			$bit += $bit;
		}
		if ($res == $target) {
			my $l = bitcnt($i);
			if ($l < $best) { $best = $l; }
		}
	}
	return $best;
}	

#part1
foreach (@lines) {
	my $shortest = solve($_);
	#printf("$_ -> $shortest\n");
	$part1 += $shortest;
}

sub gen
{
	my ($s, $len) = @_;
	my @arr = (0) x $len;
	my @s = split(/,/,$s);
	foreach(@s) {
		$arr[$_]++;
	}
	while (scalar(@arr) < $len) { push(@arr,0);}
	return @arr;
}

sub tgen
{
	my ($s) = @_;
	my @s = split(/,/,$s);
	return @s;
}

my @sb;

#my $best = 1e38;

my %cache = ();
sub try
{
	my ($keys, $best, @jolts) = @_;
	my $key = join(",",@_);
	if (defined($cache{$key})) {
		return $cache{$key};
	}
	#printf("%3d -> %s\n",$keys, join(",",@jolts));
#	if (join('',@jolts) eq '0001') {
#		printf("Almost\n");
#	}
	$keys++;
	return $keys if ($keys >= $best);
	foreach (@sb) {
		my @rest = ();
		my @keys = @{$_};
		my $nonzero = 0;
		my $negative = 0;
		for (my $i = 0; $i < scalar(@jolts); $i++) {
			my $diff = $jolts[$i]-$keys[$i];
			$negative += ($diff < 0);
			$nonzero += $diff != 0;
			$rest[$i] = $diff;
		}
		if ($nonzero == 0) {
			$best = $keys;
			$cache{$key} = $best;
			return $best;
		}
		next if ($negative);
		my $t = try($keys,$best, @rest);
		if ($t < $best) {$best = $t;}
	}
	$cache{$key} = $best;
	return $best;
}

sub solve2
{
	my ($line) = @_;
	my @parts = split(/ /,$line);
	my $target = indre(shift @parts);
	#$target = targetbits($target);
	my @jolts = tgen(indre(pop @parts));
	my $len = scalar(@parts);
	my $keycnt = scalar(@jolts);
	my @buttons = ();
	my $b = 0;
	my @sort = ();
	foreach (@parts) {
		#printf("$_");
		my @b = gen(indre($_),$keycnt);
		push(@buttons, \@b);
		push(@sort,sprintf("%3d,%2d",scalar(@b),$b));
		$b++;
	}
	@sort = reverse sort @sort;
	@sb = ();
	foreach (@sort) {
		my ($items, $idx) = split(/,/);
		push(@sb, $buttons[$idx]);
	}
	#$best = 1e38;
	%cache = ();
	my $best = try(0,1e38, @jolts);
	printf("$best: $line\n");
	return $best;
}

use Heap::Priority;

sub sum
{
	my $sum = 0;
	foreach (@_) {
		$sum += $_;
	}
	return $sum;
}

sub subtract
{
	my ($b, @jolts) = @_;
	my @rest = ();
	my $sum = 0;
	for (my $i = 0; $i < scalar(@jolts); $i++) {
		my $diff = $jolts[$i] - $b->[$i];
		if ($diff < 0) {
			return $diff;
		}
		$sum += $diff;
		push(@rest,$diff);
	}
	return ($sum, @rest);
}		

sub dfs
{
	my ($line) = @_;
	my @parts = split(/ /,$line);
	my $target = indre(shift @parts);
	my @jolts = tgen(indre(pop @parts));
	my $len = scalar(@parts);
	my $keycnt = scalar(@jolts);
	my @buttons = ();
	my $b = 0;
	my @sort = ();
	my $max = 0;
	foreach (@parts) {
		#printf("$_");
		my @b = gen(indre($_),$keycnt);
		my $m = sum(@b);
		if ($m > $max) {$max = $m;}
		push(@buttons, \@b);
		$b++;
	}
	my $imax = 1.0 / $max;
	my $pq = Heap::Priority->new();
	$pq->lowest_first();
	$pq->add(join(',', 0, @jolts),sum(@jolts));
	my $best = 1e38;
	printf("$line\n");
	while (my $next = $pq->pop()) {
		printf("$next\n");
		my ($steps,@jolts) = split(',',$next);
		$steps++;
		last if ($steps >= $best);
		foreach (@buttons) {
			my ($dist, @rest) = subtract($_, @jolts);
			if ($dist == 0) {
				$best = $steps;
				printf("$line\nFound new best path with %d steps\n",$steps);
				next;
			}
			next if ($dist < 0);
			next if ($steps + $imax*($best-$steps) > $best); 
			$pq->add(join(',',$steps,@rest),$dist);
		}
	}
	return $best;
}	

use Math::Matrix;

sub math
{
	my ($line) = @_;
	my @parts = split(/ /,$line);
	my $target = indre(shift @parts);
	my @jolts = tgen(indre(pop @parts));
	my $len = scalar(@parts);
	my $keycnt = scalar(@jolts);
	my @buttons = ();
	my $b = 0;
	my @sort = ();
	my $max = 0;
	foreach (@parts) {
		#printf("$_");
		my @b = gen(indre($_),$keycnt);
		my $m = sum(@b);
		if ($m > $max) {$max = $m;}
		push(@buttons, \@b);
		$b++;
	}
	my $A = Math::Matrix->new(@buttons);
	$A->print("A\n");
	my $B = $A -> transpose;
	$B->print("A transposed\n");
	my $x = Math::Matrix->new(\@jolts);
	#$x -> print("x");
	my $y = $x -> transpose;
	#$y -> print("y");
	#my $V = $A -> concat($x);
	#$V -> print("Equation system\n");
	my $E = $B -> concat($y);
	$E -> print("Equation system\n");
	
	# Compute the solution.
#	my $s = $V -> solve;
#	$s -> print("Solutions s\n");
	my $s = $E -> solve;
	$s -> print("Solutions s\n");

#	$A->print;
}

#part2
foreach (@lines) {
	my $shortest = math($_);
	#printf("$_ -> $shortest\n");
	dfs($_);
	die;
	$part2 += $shortest;
}


my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
