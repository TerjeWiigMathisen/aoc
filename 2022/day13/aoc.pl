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
		while ($s[$i-1] =~ /\d+/ && $s[$i] =~ /\d/) {
			$s[$i-1] = $s[$i-1]*10+$s[$i];
			splice(@s,$i,1);
		}
	}
	return @s;
}

sub comp # -1, 0, 1 for left = right
{
	my ($f, $s) = @_;
	@f = joindig(split(//,$f));
	@s = joindig(split(//,$s));
	my ($x,$y)  = (0,0);
	while ($x < scalar(@f)) {
		if ($f[$x] eq '[') {
			if ($s[$y] =~ /\d+/) {
				splice(@s,$y,1,'[',$s[$y],']');
			}
			if ($s[$y] ne '[') {
				return -100-$x;
			}
			$x++; $y++;
		}
		elsif ($s[$y] eq '[') {
			if ($f[$x] =~ /\d/) {
				splice(@f,$x,1,'[',$f[$x],']');
			}
			if ($f[$x] ne '[') {
				return 100+$x;
			}
			$x++; $y++;
		}
		elsif ($f[$x] eq ']') {
			if ($s[$y] ne ']') {
				return 200+$x;
			}
			$x++; $y++;
		}
		elsif ($s[$y] eq ']') {
			return -200-$x;
		}
		elsif ($f[$x] eq ',') {
			if ($s[$y] ne ',') {
				return -400-$x;
			}
			$x++; $y++;
		}
		elsif ($s[$y] eq ',') {
			return 400+$x;
		}
		elsif ($f[$x] < $s[$y]) {
			return 300+$x;
		}
		elsif ($f[$x] > $s[$y]) {
			return -300-$x;
		}
		else {
			$x++; $y++;
		}
	}
	return $y < scalar(@s);
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
