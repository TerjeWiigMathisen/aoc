#!perl -w 

use strict;
use Time::HiRes qw(time);
#use bigint;

my $inp = 
q(939
7,13,x,x,59,x,31,19);
#$inp = 
#q(1
#1789,37,47,1889);

my $preamble = 25;

if (defined(shift)) {
	$inp = 
q(1004098
23,x,x,x,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,509,x,x,x,x,x,x,x,x,x,x,x,x,13,17,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29,x,401,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,19);

$preamble = 5;
}

my $t = time();
#printf("Start %s\n",$t);

my ($ts, $buses) = split(/\n/,$inp);
my @bus = ();
foreach (split(/,/,$buses)) {
	push(@bus, $_) unless ($_ eq 'x');
}

my ($bestid, $minwait) = (undef, 1e38);

foreach (@bus) {
	my $w = int(($ts+$_-1)/$_)*$_ - $ts;
	if ($w < $minwait) {
		$minwait = $w;
		$bestid = $_;
	}
}

my $part1 = $bestid*$minwait;

#printf("id = %d, wait = %d, prod = %1.0f\n", $bestid, $minwait, $bestid*$minwait);

($ts, $buses) = split(/\n/,$inp);
@bus = ();
my %off = ();
my $o = 0;
foreach (split(/,/, $buses)) {
	if ($_ ne 'x') {
		$off{$_} = $o;
		push(@bus, $_);
#		printf("%4d,%-3d", $o, $_);
	}
	$o++;
}
#printf("\n");

# Check each pair:
my $zero = $bus[0];
my $step = $zero;
my @first = ();
my $part2 = 0;
my $iter = 0;
for (my $i = 1; $i < scalar(@bus); $i++) {
	my $b = $bus[$i];
	my $o = $off{$b};
	my $o0 = $o % $b;
	for (my $j = $zero; 1; $j+=$step) {
		$iter++;
		my $n = ($j + $b - 1)/$b;
		$n = int($n)*$b;
#		printf(STDERR "%s %s %s\r", $j, $n, $n-$j);
		$n -= $j;
		if ($n == $o0) {
			$first[$i] = $j;
#			printf("%s:%s first at %s, step = %s\n", $b, $o, $j, $zero*$b);
			$zero = $j;
			$step *= $b;
			$part2 = $j;
			last;
		}
	}
}

my $t1 = time();
#printf("%s\n",$t1);

$t = $t1-$t;
printf("Total time = %s ms\n", $t*1000);
printf("Total $iter iterations\n");

exit();

