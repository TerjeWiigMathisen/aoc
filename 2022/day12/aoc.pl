#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);
#use List::Util qw (reduce);

#use bigint;

#use JSON::Parse;
no warnings 'recursion';

my $DEBUG = 0;

my $fname = shift;
$fname = 'input.txt' unless (defined($fname));
open(F ,'<',$fname);

my $start = time;

my $part1 = 0;
my $part2 = 0;

my @S;
my @E;

my @inp = ();
foreach (<F>) {
	chomp;
	push(@inp, chr(126) x (length($_)+2)) unless (scalar(@inp));
	$_ = chr(126).$_.chr(126);
	if (/E/) {
		@E = (length($PREMATCH), scalar(@inp));
		substr($_,length($PREMATCH),1) = 'z';
	}
	if (/S/) {
		@S = (length($PREMATCH), scalar(@inp));
		substr($_,length($PREMATCH),1) = 'a';
	}
	push(@inp,$_);
}
push(@inp, $inp[0]);
close(F);
#printf(STDERR "%s\n\n", join("\n", @inp));

my %visited = ();
my @solution = ();
my $bestsol = 1e8;
my @bestsol = ();
sub try
{
	my ($x0,$y0, $x, $y) = @_;
	if (defined($visited{"$x,$y"}) && $visited{"$x,$y"}<= scalar(@solution)) { return 1e8;}
	if (scalar(@solution) >= $bestsol) {return 1e8; }
	
	my $h = ord(substr($inp[$y],$x,1));
	my $h0 = ord(substr($inp[$y0],$x0,1));
	if ($h-$h0 > 1) { return 1e8; }

	$visited{"$x,$y"} = scalar(@solution);
	
	push(@solution, $x, $y);
	if ($x == $E[0] && $y == $E[1]) {
		if (scalar(@solution) < $bestsol) {
			$bestsol = scalar(@solution);
			@bestsol = @solution;
			pop(@solution);
			pop(@solution);
			return (($bestsol>>1)-1);
		}
	}
	my $len = try ($x,$y, $x+1,$y);
	my $l = try($x, $y, $x-1, $y);
	if ($l < $len) {
		$len = $l;
	}
	$l = try($x, $y, $x, $y-1);
	if ($l < $len) {
		$len = $l;
	}
	$l = try($x, $y, $x, $y+1);
	if ($l < $len) {
		$len = $l;
	}
	pop(@solution);
	pop(@solution);
	return $len;
}

$part1 = try(@S,@S);
#my $p = 0;
#foreach(@bestsol) {
#	printf(STDERR "%s%s", $_, $p ? ' ' : ',');
#	$p ^= 1;
#}
printf(STDERR "\n");

%visited = ();
@solution = ();
$bestsol = 1e8;
@bestsol = ();
foreach (@inp) {
	s/~/ /g;
}

sub tryr
{
	my ($x0,$y0, $x, $y) = @_;
	if (defined($visited{"$x,$y"}) && $visited{"$x,$y"}<= scalar(@solution)) { return 1e8;}
	if (scalar(@solution) >= $bestsol) {return 1e8; }
	
	my $h = ord(substr($inp[$y],$x,1));
	my $h0 = ord(substr($inp[$y0],$x0,1));
	if ($h-$h0 < -1) { return 1e8; }

	$visited{"$x,$y"} = scalar(@solution);
	
	push(@solution, $x, $y);
	if ($h == 97) {
		if (scalar(@solution) < $bestsol) {
			$bestsol = scalar(@solution);
			@bestsol = @solution;
			pop(@solution);
			pop(@solution);
			return (($bestsol>>1)-1);
		}
	}
	my $len = tryr ($x,$y, $x+1,$y);
	my $l = tryr($x, $y, $x-1, $y);
	if ($l < $len) {
		$len = $l;
	}
	$l = tryr($x, $y, $x, $y-1);
	if ($l < $len) {
		$len = $l;
	}
	$l = tryr($x, $y, $x, $y+1);
	if ($l < $len) {
		$len = $l;
	}
	pop(@solution);
	pop(@solution);
	return $len;
}

$part2 = tryr(@E,@E);

printf(STDERR "Total time = %f\n", time - $start);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);

printf(STDERR "Total time = %f\n", time - $start);
