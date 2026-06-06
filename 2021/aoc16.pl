#!perl -w

use strict;
use Time::HiRes qw (time);
use List::PriorityQueue;
use warnings;
no warnings 'recursion';

my $start = time;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

my $DEBUG = 0;

my %h2b = ();
my $h = 0;
foreach ("0000","0001","0010","0011","0100","0101","0110","0111",
"1000","1001","1010","1011","1100","1101","1110","1111") {
	$h2b{sprintf("%X",$h++)} = $_;
}

my $bits;

my @stack = ();

my $part1 = 0;
my $part2;

sub b2d
{
	my ($b) = @_;
	my $d = 0;
	foreach (split(//,$b)) {
		$d = $d*2+$_;
	}
	return $d;
}

sub version
{
	my ($bpos) = @_;
	my $v = b2d(substr($bits,$bpos,3));
	$bpos += 3;
	printf(STDERR "V:%d ", $v) if ($DEBUG);
	$part1 += $v;
	return ($v, $bpos);
}

sub tid
{
	my ($bpos) = @_;
	my $p = b2d(substr($bits,$bpos,3));
	$bpos += 3;
	printf(STDERR "T:%d ",$p) if ($DEBUG);
	return ($p, $bpos);
}

sub lit
{
	my ($bpos) = @_;
	my ($lit, $l) = (0);
	do {
		$l = b2d(substr($bits,$bpos,5));
		$bpos += 5;
		$lit = ($lit << 4) + ($l & 15);
	} while ($l & 16);
	printf(STDERR "LIT: %d\n",$lit) if ($DEBUG);
	push(@stack, $lit);
	return ($lit, $bpos);
}

sub op
{
	my ($tid, $bpos) = @_;
	my $lid = substr($bits,$bpos++,1);
	my $opc = scalar(@stack);
	if ($lid == 0) { # 15-bit immediate
		my $sublen = b2d(substr($bits,$bpos,15));
		$bpos += 15;
		for (my $b = $bpos; $b < $bpos + $sublen; ) {
			$b = decode_packet($b);
		}
		$bpos += $sublen;
	}
	else { # 11-bit sub-packet count
		my $subcnt = b2d(substr($bits,$bpos,11));
		$bpos += 11;
		for (my $s = 0; $s < $subcnt; $s++) {
			$bpos = decode_packet($bpos);
		}
	}
	my $r;
	if ($tid == 0) { # Sum
		$r = pop(@stack);
		while (scalar(@stack) > $opc) {
			$r += pop(@stack);
		}
	}
	elsif ($tid == 1) { # Product
		$r = pop(@stack);
		while (scalar(@stack) > $opc) {
			$r *= pop(@stack);
		}
	}
	elsif ($tid == 2) { # Minimum
		$r = pop(@stack);
		while (scalar(@stack) > $opc) {
			my $p = pop(@stack);
			$r = $p if ($p < $r);
		}
	}
	elsif ($tid == 3) { # Maximum
		$r = pop(@stack);
		while (scalar(@stack) > $opc) {
			my $p = pop(@stack);
			$r = $p if ($p > $r);
		}
	}
	elsif ($tid == 5) { # Greater than
		my $s = pop(@stack);
		my $f = pop(@stack);
		$r = $f > $s ? 1 : 0;
	}
	elsif ($tid == 6) { # Less than
		my $s = pop(@stack);
		my $f = pop(@stack);
		$r = $f < $s ? 1 : 0;
	}
	elsif ($tid == 7) { # Equal?
		my $s = pop(@stack);
		my $f = pop(@stack);
		$r = $f == $s ? 1 : 0;
	}
	push(@stack, $r);
	return $bpos;
}

sub decode_packet
{
	my ($bpos) = @_;
	my ($v, $tid, $lit);

	($v, $bpos) = version($bpos);
	($tid, $bpos) = tid($bpos);
	if ($tid == 4) { # Literal
		($lit, $bpos) = lit($bpos);
	}
	else { # Operator packet!
		$bpos = op($tid, $bpos);
	}
	return $bpos;
}


foreach (@inp) {
	$bits = "";
	foreach (split(//)) {
		$bits .= $h2b{$_};
	}
	my $bpos = decode_packet(0);
}

$part2 = pop(@stack);

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %d\n", $part1);
printf("Part2: %d\n", $part2);

