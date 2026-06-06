#!perl -w

use strict;
use Time::HiRes qw (time);
use List::PriorityQueue;
use warnings;
#no warnings 'recursion';

my $start = time;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

my $scanner;
my @beacons;
foreach (@inp) {
	if (/ scanner (\d+) /) {
		$scanner = $1;
	}
	elsif (/\d/) {
		my ($a,$b,$c) = split(/,/);
		push(@{$beacons[$scanner]}, $a, $b, $c);
	}
}

my $scanners = $scanner+1;

printf(STDERR "Found %d scanners\n", $scanners);

# calculate x^2
sub sq {$_[0]*$_[0];}

my %dcache = ();
sub deltas # Memoizing calculation of all internal distances
{
	my ($scanner) = @_;
	if (defined($dcache{$scanner})) { return @{$dcache{$scanner}}; }
	
	my @s0 = @{$beacons[$scanner]};
	my @delta = ();
	for (my $i = 5; $i < scalar(@s0); $i+=3) {
		my ($x0,$y0,$z0) = (@s0)[$i-2,$i-1,$i];
		for (my $j = 2; $j < $i; $j += 3) {
			my ($x1,$y1,$z1) = (@s0)[$j-2,$j-1,$j];
			push(@delta,sq($x0-$x1)+sq($y0-$y1)+sq($z0-$z1));
		}
	}
	@delta = sort {$a <=> $b} @delta;
	# Add guard:
	push(@delta, 1e12); # Larger than all possible distances
	$dcache{$scanner} = \@delta;
	return @delta;
}

# Check how many squared distances match between two scanners
sub match
{
	my ($s0, $s1) = @_;
	my @s0 = deltas($s0);
	my @s1 = deltas($s1);
	my ($i,$j, $m) = (0,0,0);
	for ($i = 0; $i < scalar(@s0)-1; $i++) {
		while ($s1[$j] < $s0[$i]) { $j++; }
		if ($s1[$j] == $s0[$i]) {
			$m++; # Found an identical match
		}
	}
	return $m;
}

my %fixed = ();
my %base = ();

sub fix0
{
	my @base = @{$beacons[0]};
	for (my $bs = 2; $bs < scalar(@base); $bs += 3) {
		my ($rx, $ry, $rz) = (@base)[$bs-2,$bs-1,$bs];
		$base{"$rx,$ry,$rz"}++;
	}
	$fixed{0}++;
}

my @scanpos = (0,0,0);

sub orient
{
	my ($base, $s) = @_;
	my @base = @{$beacons[$base]};
	my @s = @{$beacons[$s]};

	my @found = ();
	for (my $bs = 2; $bs < scalar(@base); $bs += 3) {
		my ($rx, $ry, $rz) = (@base)[$bs-2,$bs-1,$bs];
		
		for (my $x = -2; $x <= 0; $x++) {
			foreach (-1,1) {
				my $dx = $_;
				for (my $y = -2; $y <= 0; $y++) {
					next if ($x == $y);
					foreach (-1,1) {
						my $dy = $_;
						for (my $z = -2; $z <= 0; $z++) {
							next if ($x == $z || $y == $z);
							foreach (-1,1) {
								my $dz = $_;
								
								for (my $start = 2; $start < scalar(@s); $start+=3) {
									my ($ox, $oy, $oz) = ($s[$start+$x]*$dx, $s[$start+$y]*$dy, $s[$start+$z]*$dz);
									($ox,$oy, $oz) = ($rx-$ox,$ry-$oy,$rz-$oz);
									my $found = 0;
									for (my $t = 2; $t < scalar(@s); $t+=3) {
										my ($tx, $ty, $tz) = ($s[$t+$x]*$dx+$ox, $s[$t+$y]*$dy+$oy, $s[$t+$z]*$dz+$oz);
										if (defined($base{"$tx,$ty,$tz"})) {
											$found++;
										}
									}
									push(@found,sprintf("%4d,%s", $found, join(",",$x, $y, $z, $dx,$dy,$dz,$ox,$oy,$oz)));
									goto funnet if ($found >= 12);
									#printf(STDERR "%4d,%s", $found, join(",",$x, $y, $z, $dx,$dy,$dz,$ox,$oy,$oz));
								}
							}
						}
					}
				}
			}
		}
	}
funnet:
	@found = reverse sort @found;
	my ($found, $x, $y, $z, $dx,$dy,$dz,$ox,$oy,$oz) = split(/,/, $found[0]);
	printf(STDERR "%d: %s\n", $s, $found[0]);
	for (my $t = 2; $t < scalar(@s); $t+=3) {
		my ($tx, $ty, $tz) = ($s[$t+$x]*$dx+$ox, $s[$t+$y]*$dy+$oy, $s[$t+$z]*$dz+$oz);
		splice(@s,$t-2,3,$tx, $ty, $tz); # Fixup this entry
		$base{"$tx,$ty,$tz"}++;
	}
	@{$beacons[$s]} = @s;
	$fixed{$s}++;
	push(@scanpos, $ox,$oy,$oz);
	return 1;
}

my @m = ();
for (my $s = 1; $s < $scanners; $s++) {
	for (my $f = 0; $f < $s; $f++) {
		my $m = match($f, $s);
		push(@m, sprintf("%4d,%d,%d",$m,$f,$s)) if ($m > 50);
	}
}
printf(STDERR "Pairing time = %f\n", time - $start);
@m = sort @m;
foreach (@m) {
	printf("%s\n", $_);
}

my $part1 = 0;

my $cnt = 0;
fix0();

do {
	$cnt = scalar(keys %fixed);
	my $n = 0;
	foreach (@m) {
		my ($found, $base, $sec) = split(/,/);
		next if (defined($fixed{$base}) == defined($fixed{$sec}));
		if (defined($fixed{$base})) {
			orient($base, $sec);
		}
		else {
			orient($sec, $base);
		}
	}
} while ($cnt < scalar(keys %fixed));

$part1 = scalar(keys %base);

my $part2 = 0;

for (my $s = 5; $s < scalar(@scanpos); $s+=3) {
	for (my $f = 2; $f < $s; $f+=3) {
		my $man = abs($scanpos[$s-2]-$scanpos[$f-2])+abs($scanpos[$s-1]-$scanpos[$f-1])+abs($scanpos[$s]-$scanpos[$f]);
		if ($man > $part2) {
			$part2 = $man;
		}
	}
}

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %d\n", $part1);
printf("Part2: %d\n", $part2);
