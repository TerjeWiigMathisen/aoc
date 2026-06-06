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

my @f;
my @s;

sub joindig
{
	my @s = @_;
	for (my $i = 1; $i < scalar(@s); $i++) {
		if ($s[$i-1] =~ /\d/) {
			while ($s[$i] =~ /\d/) {
				splice(@s,$i-1,2,$s[$i-1].$s[$i]);
			}
		}
	}
	return @s;
}

sub comp # negative, zero, positive for left <= right
{
	my ($f, $s) = @_;
	@f = joindig(split(//,$f));
	@s = joindig(split(//,$s));
	my $x;
	for ($x = 0; $x < scalar(@f); $x++) {
		my ($a, $b) = ($f[$x], $s[$x]);
		if ($a eq '[') {
			if ($b =~ /\d+/) { # list vs number?
				splice(@s,$x,1,'[',$b,']'); # Wrap in brackets
				$b = '[';
			}
			if ($b ne '[') {
				return -100-$x;
			}
		}
		elsif ($b eq '[') {
			if ($a =~ /\d/) {
				splice(@f,$x,1,'[',$a,']');
				$a = '[';
			}
			if ($a ne '[') {
				return 100+$x;
			}
		}
		elsif ($a eq ']') {
			if ($b ne ']') {
				return 200+$x;
			}
		}
		elsif ($b eq ']') {
			return -200-$x;
		}
		elsif ($a eq ',') {
			if ($b ne ',') {
				return -400-$x;
			}
		}
		elsif ($b eq ',') {
			return 400+$x;
		}
		elsif ($a < $b) {
			return 300+$x;
		}
		elsif ($a > $b) {
			return -300-$x;
		}
	}
	return $x < scalar(@s);
}	

my @all = ();
my $pair = 0;
my ($to, $seks) = (1,1);
foreach (@pairs) {
	$pair++;
	my ($f,$s) = split(/\n/);
	my $c = comp($f,$s);
#	printf("%s\n%s\n%s\n%s\n%d\n", $f, $s, join("",@f),join("",@s),$c);
	$part1 += $pair if ($c >= 0);
	push(@all,$f,$s);
}
printf(STDERR "Total time = %f\n", time - $start);
push(@all,'[[2]]','[[6]]');

@all = sort {comp($b,$a)} @all;
#printf("%s\n",join("\n",@all));
my $pos = 1;
foreach (@all) {
	if ($_ eq '[[2]]') {
		$to = $pos;
	}
	elsif  ($_ eq '[[6]]') {
		$seks = $pos;
	}
	$pos++;
}
$part2 = $to*$seks;

printf(STDERR "Total time = %f\n", time - $start);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
