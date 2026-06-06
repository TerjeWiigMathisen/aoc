#!perl -w
use strict;
use Time::HiRes qw(time);
use List::PriorityQueue;
#use Math::Polygon::Calc;
use Math::BigInt;

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @inp = (<>); chomp(@inp);

sub p1{
	my (@b) = @_;
}

sub new_stack
{
	my ($n, $deck_len) = @_;
	return $deck_len-$n-1;
}

sub cut_n
{
	my ($n, $cut, $deck_len) = @_;
	$n -= $cut;
	if ($deck_len <= $n) {
		$n -= $deck_len;
	}
	return $n;
}

sub deal_with_increment
{
	my ($n, $inc, $deck_len) = @_;
	$n = ($n*$inc) % $deck_len;
	return $n;
}

sub r_cut_n
{
	my ($n, $cut, $deck_len) = @_;
	$n += $cut;
	if ($deck_len <= $n) {
		$n -= $deck_len;
	}
	return $n;
}

sub r_deal_with_increment
{
	my ($n, $inc, $deck_len) = @_;
	
	for (my $a = 0; $a <= $inc; $a++) {
		my $loc = $deck_len*$a+$n;
		my $d = int($loc/$inc);
		my $r = $loc-$d*$inc;
		if ($r == 0) {
			return $d;
		}
	}
	die("No previous pos found!");
}

sub p2
{
	my ($n, $deck_len, @ops) = @_;
	foreach (reverse @ops) {
		if (/deal into new stack/) {
			$n = new_stack($n, $deck_len);
		}
		elsif (/cut ([\-]?[\d]+)$/) {
			$n = r_cut_n($n,$1,$deck_len);
		}
		elsif (/deal with increment (\d+)$/) {
			$n = r_deal_with_increment($n,$1,$deck_len);
		}
		else {
			die("Bad op'$_'");
		}
	}
	return $n;
}

$part1 = p1(@inp);

my $ITERATIONS = 101741582076661;
my $DECK_SIZE = 119315717514047;

my $x = 2020;

my $y = p2($x,$DECK_SIZE,@inp);
my $z = p2($y,$DECK_SIZE,@inp);
my $base = Math::BigInt->new($x-$y+$DECK_SIZE);
printf("$base\n");
my $a = ($y-$z) * $base->bmodinv($DECK_SIZE);
$a = $a->bmod($DECK_SIZE);
my $b = Math::BigInt->new($y-$a*$x);
$b = $b->bmod($DECK_SIZE);
printf("A=%s, B=%s\n", $a, $b);
my $p = $a->copy();
my $p = $p->bmodpow($ITERATIONS,$DECK_SIZE);
printf("A=%s, B=%s\n", $a, $b);

my $a_m1 = $a->badd(-1);
printf("A-1=%s\n", $a_m1);

$part2 = ($p*$x + ($p->badd(-1))*$a_m1->bmodinv($DECK_SIZE)*$b) % $DECK_SIZE;


my $used = time()-$t0;
printf("%s\n%s\n", $part1, $part2);
printf("Used %1.6fs\n", $used);
