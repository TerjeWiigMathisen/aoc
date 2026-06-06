#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
#use Algorithm::Permute qw (permute);

#use bigint;

#use JSON::Parse;
#no warnings 'recursion';

my $start = time;

my $DEBUG = 0;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
my $inp = join("",@inp);
my @pairs = split("\n\n", $inp);

my $part1 = 0;
my $part2 = 0;

sub trans
{
	my ($s)=@_;
	my $sep = 'a';
	my @out = ();
	foreach(split(//,$s)){
		if ($_ eq '[') {
			$sep++;
		}
		elsif ($_ eq ']') {
			$sep--;
		}
		elsif ($_ eq ',') {
			push(@out,$sep);
		}
		else {
			push(@out,$_);
		}
	}
	return join("",@out);
}

sub comp # -1, 0, 1 for left = right
{
	my ($f, $s) = @_;
}	

my $pair = 0;
foreach (@pairs) {
	$pair++;
	my ($f,$s) = split(/\n/);
	my ($l,$r) = (trans($f),trans($s));
	printf(STDERR "%s %s\n%s %s\n,%d\n", $f, $l, $s, $r, $l lt $r);
	$part1 += $l lt $r;
}

printf(STDERR "Total time = %f\n", time - $start);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
