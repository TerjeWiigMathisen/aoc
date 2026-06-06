#!perl -w

use strict;
use Time::HiRes qw (time);
use Math::BigInt;

my $start = time;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

my $temp = shift @inp;
shift @inp;

my %pair;
foreach (@inp) {
	die ("Bad input") unless (/^(\S\S) \-\> (\S)$/);
	$pair{$1} = $2;
}

my $part1 = expand2($temp,10);

sub expand2
{
	my ($temp, $iter) = @_;
	my %alfa = ();
	foreach (split(//, $temp)) {
		$alfa{$_}++;
	}
	my %pat = ();
	for (my $i = 1; $i < length($temp); $i++) {
		my $key = substr($temp,$i-1,2);
		$pat{$key}++;
	}

	for (my $s = 1; $s <= $iter; $s++) {
		my @keys = keys %pat;
		my %p = ();
		foreach (@keys) {
			my $cnt = $pat{$_}; # How many times do we have this pair?
			if ($cnt && defined($pair{$_})) {
				my ($f,$l) = (substr($_,0,1).$pair{$_}, $pair{$_}.substr($_,1,1));
				$p{$f} += $cnt; # Insert the two new patterns
				$p{$l} += $cnt;
				$p{$_} -= $cnt; # and remove the old!
				$alfa{$pair{$_}} += $cnt; # Increase the count of the inserted character
			}
		}
		foreach (keys %p) { # Add the new pattern counts to the previous counts
			$pat{$_} += $p{$_};
		}
	}
	
	my @keys = sort {$alfa{$a} <=> $alfa{$b}} (keys  %alfa);
	return $alfa{$keys[-1]}-$alfa{$keys[0]};
}

my $iter = shift;
$iter = 40 unless (defined($iter));
my $part2 = expand2($temp,$iter);

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %d\n", $part1);
printf("Part2: %d\n", $part2);

