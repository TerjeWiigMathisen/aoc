#!perl -w
use strict;
use warnings;
use Time::HiRes qw(time);

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @inp = (<>); chomp(@inp);
my $MX = length($inp[0]);
my $MY = scalar(@inp);
my @b = '.' x ($MX+2);
foreach (@inp) { push(@b, '.'.$_."."); }
push(@b,'.' x ($MX+2));
#printf("%s\n",join("\n",@b));

sub bits
{
	my (@b) = @_;
	my $bit = 1;
	my $bits = 0;
	for (my $y = 1; $y <= $MY; $y++) {
		for (my $x = 1; $x <= $MX; $x++) {
			$bits += $bit if (substr($b[$y],$x,1) eq '#');
			$bit *= 2;
		}
	}
	return $bits;
}

sub gen
{
	my (@b) = @_;
	my @n = @b;
	my @next = ('.','#');
	for (my $y = 1; $y <= $MY; $y++) {
		for (my $x = 1; $x <= $MX; $x++) {
			my $nbrs = (substr($b[$y-1],$x,1) eq '#')+(substr($b[$y],$x-1,1) eq '#')+
				(substr($b[$y],$x+1,1) eq '#')+(substr($b[$y+1],$x,1) eq '#');
			my $curr = (substr($b[$y],$x,1) eq '#');
			my $c = ($curr == 0 && ($nbrs == 1 || $nbrs == 2)) ||
					($curr == 1 && $nbrs == 1);
			substr($n[$y],$x,1) = $next[$c];
		}
	}
	return @n;
}
	
my %seen = ();

sub p1
{
	my (@b) = @_;
	my $bits = bits(@b);
	$seen{$bits} = 1;
	my $gen = 1;
	while (1) {
		$gen++;
		@b = gen(@b);
#		printf("%d\n%s\n",$gen-1,join("\n",@b));
		$bits = bits(@b);
		if ($seen{$bits}) {
			printf("Found a loop with length %d in generation %d\n", $gen-$seen{$bits}, $gen);
			return $bits;
		}
		$seen{$bits} = $gen;
	}
}

my @levels = (0) x 400;
my $L0 = 199;
my $L1 = 201;

sub b
{
	my (@bits) = @_;
	my $n = 0;
	foreach (@bits) {
		$n |= 1 << ($_-1);
	}
	return $n;
}

my @omask = (b(8,12),b(8),b(8),b(8),b(8,14),
			 b(12),0,0,0,b(14),
			 b(12),0,0,0,b(14),
			 b(12),0,0,0,b(14),
			 b(12,18),b(18),b(18),b(18),b(14,18));
my @cmask = (b(2,6),b(1,3,7),b(2,4,8),b(3,5,9),b(4,10),
			 b(1,7,11),b(2,6,8,12),b(3,7,9),b(4,8,10,14),b(5,9,15),
			 b(6,12,16),b(7,11,17),0,b(9,15,19),b(10,14,20),
			 b(11,17,21),b(12,16,18,22),b(17,19,23),b(14,18,20,24),b(15,19,25),
			 b(16,22),b(17,21,23),b(18,22,24),b(19,23,25),b(20,24));
my @imask = (0,0,0,0,0,
			0,0,b(1,2,3,4,5),0,0,
			0,b(1,6,11,16,21),0,b(5,10,15,20,25),0,
			0,0,b(21,22,23,24,25),0,0,
			0,0,0,0,0);
			
sub bitcount
{
	my ($b) = @_;
	my $c = 0;
	while ($b) {
		$c += $b & 1;
		$b >>= 1;
	}
	return $c;
}

sub neighbors
{
	my ($l,$x,$y) = @_;
	my $idx = $y*5+$x;
	my $ncnt = 0;
	my ($out,$cur, $inn) = ($omask[$idx],$cmask[$idx],$imask[$idx]);
	$ncnt += bitcount($levels[$l-1]&$out)+bitcount($levels[$l]&$cur)+bitcount($levels[$l+1]&$inn);
	return $ncnt;
}

sub gen2
{
	my @n = (0) x 400;
	my $cnt = 0;
	for (my $l = $L0; $l <= $L1; $l++) {
		for (my $y = 0; $y < 5; $y++) {
			for (my $x = 0; $x < 5; $x++) {
				my $idx = $y*5+$x;
				my $curr = ($levels[$l] >> $idx) & 1;
				my $nbrs = neighbors($l,$x,$y);
				my $c = ($curr == 0 && ($nbrs == 1 || $nbrs == 2)) ||
					($curr == 1 && $nbrs == 1);
				$n[$l] |= $c << $idx;
				$cnt += $c;
			}
		}
	}
	@levels = @n;
	if ($levels[$L0]) {$L0--;}
	if ($levels[$L1]) {$L1++;}
	return $cnt;
}

sub p2
{
	my ($gens, @b) = @_;
	my $bits = bits(@b);
	$levels[200] = $bits;

	my $bugs;
	for (my $g = 0; $g < $gens; $g++) {
		$bugs = gen2();
	}
	return $bugs;
}

sub disp
{
	my ($l,$b) = @_;
	my @d = ('.','#');
	printf("%d\n",$l);
	for (my $y = 0; $y < 5; $y++) {
		for (my $x = 0; $x < 5; $x++) {
			printf("%s",$d[$b & 1]);
			$b >>= 1;
		}
		printf("\n");
	}
	printf("\n");
}

$part1 = p1(@b);
$part2 = p2(200,@b);

#for (my $l = $L0+1;$l < $L1; $l++) {
#	disp($l-200,$levels[$l]);
#}

my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.6fs\n", $used);
