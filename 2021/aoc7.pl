#!perl -w

use strict;

my @inp = (<>);
chomp(@inp);

my @numbers = split(/,/,shift(@inp)); 

@numbers = sort {$a <=> $b} @numbers;
#printf("%s\n", join(",", @numbers));

my $sum = 0;
foreach (@numbers) { $sum += $_; }
my $mean = $sum / scalar(@numbers);
my $median = $numbers[scalar(@numbers) >> 1];
printf(STDERR "sum: %d, count: %d, median: %d, mean: %f\n", $sum, scalar(@numbers), $median, $mean);
my $meani = int($mean+0.5);

sub fuel
{
	my ($pos, @num) = @_;
	my $fuel = 0;
	foreach (@num) { $fuel += abs($pos-$_); }
	#printf(STDERR "Pos: %d -> fuel = %d\n", $pos, $fuel);
	return $fuel;
}

my $a = search_min(\&fuel, undef, @numbers);

printf("aoc7a: %d\n", $a);

sub search_min
{
	my ($costfunc, $hint, @num) = @_;
	my ($l, $m, $r) = (@num)[0,scalar(@num)>>1, -1]; # left, median, right
	if (defined($hint)) { $m = $hint; }
	my ($mm, $mc, $mp);
	$mc = $costfunc->($m, @num);
	while ($m > $l) {
		$mm = $costfunc->($m-1, @num);
		last if ($mm > $mc);
		$m--;
		$mc = $mm;
	}
	while ($m < $r) {
		$mp = $costfunc->($m+1, @num);
		last if ($mp > $mc);
		$m++;
		$mc = $mp;
	}
	printf(STDERR "Minimal point at: %d\n", $m);
	return $mc;
}

sub fuel2
{
	my ($pos, @num) = @_;
	my $fuel = 0;
	foreach (@num) {
		my $o = abs($pos - $_);
		$fuel += ($o * ($o+1))>> 1;
	}
#	printf(STDERR "Pos: %d -> fuel = %d\n", $pos, $fuel);
	return $fuel;
}

my $b = search_min(\&fuel2, $meani, @numbers);

printf("aoc7b: %d\n", $b);

sub fuelsq
{
	my ($pos, @num) = @_;
	my $fuel = 0;
	foreach (@num) {
		my $o = $pos - $_;
		$fuel += ($o * $o);
	}
#	printf(STDERR "Pos: %d -> fuel = %d\n", $pos, $fuel);
	return $fuel;
}

my $sq = search_min(\&fuelsq, $meani, @numbers);

printf("aoc7 squared: %d\n", $sq);
