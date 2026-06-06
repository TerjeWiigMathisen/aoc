#!perl -w
use strict;
use Time::HiRes qw (time);
use English;
use warnings;
#use bigint;
use v5.32;
#use experimental qw(for_list builtin);
#use builtin qw(indexed);
no warnings 'recursion';

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

while (<>) {
	chomp;
	push(@lines, $_);
}

my $oneline = join('',@lines);

my @guard = ();
my $GC = "#";
#my $linear = $GC x length($lines[0])+1;

my $H = scalar(@lines);
my $W = length($lines[0]);

my @dx = (1,0,-1,0);
my @dy = (0,1,0,-1);

push(@guard, ($GC x (length($lines[0])+2))."\n");
foreach (@lines) {
	push(@guard,$GC.$_.$GC."\n");
}
push(@guard, $guard[0]);

my @grid = @guard;

sub fill
{
	my ($x,$y,$c) = @_;
	substr($grid[$y],$x,1) = lc($c);
	my ($area, $perimeter) = (1,0);
	for (my $d = 0; $d < 4; $d++) {
		my ($nx,$ny) = ($x+$dx[$d],$y+$dy[$d]);
		if ($nx < 0 || $nx > $W+1 || $ny < 0 || $ny > $H+1) {
			printf("$nx, $ny\n");
			print @grid;
			exit(1);
		}
		my $nc = substr($grid[$ny],$nx,1);
		if ($nc eq lc($c)) {
			# Already checked out!
			next;
		}
		if ($nc eq $c) {
			my ($a,$p) = fill($nx,$ny,$c);
			$area += $a;
			$perimeter += $p;
			next;
		}
		# Real boundary with fence
		$perimeter++;
	}
	return ($area,$perimeter);
}

sub part1
{
	my $price = 0;
	for (my $y = 1; $y <= $H; $y++) {
		for (my $x = 1; $x <= $W; $x++) {
			my $c = substr($grid[$y],$x,1);
			if ($c ge 'A' && $c le 'Z') { # Start of an area, do a flood fill and calculate area and perimeter
				#printf("$c area on line $y, col $x\n");
				my ($a,$p) = fill($x,$y,$c);
				#printf("$c area at ($x,$y), area=$a, peri=$p -> %d\n", $a*$p);
				#print @grid;
				$price += $a*$p;
			}
		}
	}
	#print @grid;
	return $price;
}

my %perimeter;

sub fill2
{
	my ($x,$y,$c) = @_;
	substr($grid[$y],$x,1) = lc($c);
	my $area = 1;
	for (my $d = 0; $d < 4; $d++) {
		my ($nx,$ny) = ($x+$dx[$d],$y+$dy[$d]);
		my $nc = substr($grid[$ny],$nx,1);
		if ($nc eq lc($c)) {
			# Already checked out!
			next;
		}
		if ($nc eq $c) {
			my $a = fill2($nx,$ny,$c);
			$area += $a;
			next;
		}
		# Real boundary with fence
		$perimeter{"$x,$y"} += 1 << $d;
	}
	return $area;
}

sub perm2
{
	my %seen = %perimeter;
	my @bitdelta = ((1,0,1),(2,1,0),(4,0,1),(8,1,0));
	foreach (keys %seen) {
		my ($x,$y) = split(/,/);
		my $p = $seen{$_};
		if ($p & 1) { # Right side, try to extend
			my $ny = $y-1;
			while (defined($seen{"$x,$ny"}) && $seen{"$x,$ny"} & 1) {
				$seen{"$x,$ny"} ^= 1;
				$ny--;
			}
			$ny = $y+1;
			while (defined($seen{"$x,$ny"}) && $seen{"$x,$ny"} & 1) {
				$seen{"$x,$ny"} ^= 1;
				$ny++;
			}
		}
		if ($p & 2) { # Below, try to extend
			my $nx = $x-1;
			while (defined($seen{"$nx,$y"}) && $seen{"$nx,$y"} & 2) {
				$seen{"$nx,$y"} ^= 2;
				$nx--;
			}
			$nx = $x+1;
			while (defined($seen{"$nx,$y"}) && $seen{"$nx,$y"} & 2) {
				$seen{"$nx,$y"} ^= 2;
				$nx++;
			}
		}
		if ($p & 4) { # Left side, try to extend
			my $ny = $y-1;
			while (defined($seen{"$x,$ny"}) && $seen{"$x,$ny"} & 4) {
				$seen{"$x,$ny"} ^= 4;
				$ny--;
			}
			$ny = $y+1;
			while (defined($seen{"$x,$ny"}) && $seen{"$x,$ny"} & 4) {
				$seen{"$x,$ny"} ^= 4;
				$ny++;
			}
		}
		if ($p & 8) { # Above, try to extend
			my $nx = $x-1;
			while (defined($seen{"$nx,$y"}) && $seen{"$nx,$y"} & 8) {
				$seen{"$nx,$y"} ^= 8;
				$nx--;
			}
			$nx = $x+1;
			while (defined($seen{"$nx,$y"}) && $seen{"$nx,$y"} & 8) {
				$seen{"$nx,$y"} ^= 8;
				$nx++;
			}
		}
	}
	# Count number of bits in %seen
	my $bits = 0;
	foreach (keys %seen) {
		my $p = $seen{$_};
		if ($p) {
			#printf("$_ $p\n");
			$bits += ($p&1) + (($p>>1)&1)+ (($p>>2)&1) + (($p>>3)&1);
		}
	}
	return $bits;
}

sub perm2d
{
	my %seen = %perimeter;
	my @bitdelta = (1,0,1,2,1,0,4,0,1,8,1,0);
	my @bitcnt = (0,1,1,2,1,2,2,3,1,2,2,3,2,3,3,4);
	foreach (keys %seen) {
		my ($x,$y) = split(/,/);
		my $p = $seen{$_};
		while (scalar(@bitdelta)) {
			my ($bit,$dx,$dy) = splice( @bitdelta, 0, 3 );
			if ($p & $bit) { # Boundary found, try to extend
				my ($nx,$ny) = ($x-$dx,$y-$dy);
				while (defined($seen{"$nx,$ny"}) && $seen{"$nx,$ny"} & $bit) {
					$seen{"$nx,$ny"} ^= $bit;
					$nx -= $dx; $ny -= $dy;
				}
				$nx = $x + $dx; $ny = $y + $dy;
				while (defined($seen{"$nx,$ny"}) && $seen{"$nx,$ny"} & $bit) {
					$seen{"$nx,$ny"} ^= $bit;
					$nx += $dx; $ny += $dy;
				}
			}
		}
	}
	# Count number of bits in %seen
	my $bits = 0;
	foreach (keys %seen) { $bits += $bitcnt[$seen{$_}]; }
	return $bits;
}

sub part2
{
	my $price = 0;
	for (my $y = 1; $y <= $H; $y++) {
		for (my $x = 1; $x <= $W; $x++) {
			my $c = substr($grid[$y],$x,1);
			if ($c ge 'A' && $c le 'Z') { # Start of an area, do a flood fill and calculate area and mark perimeter sides
				#printf("$c area on line $y, col $x\n");
				%perimeter = ();
				my $a = fill2($x,$y,$c);
				my $p = perm2d($x,$y);
				#printf("$c area at ($x,$y), area=$a, peri=$p -> %d\n", $a*$p);
				#print @grid;
				$price += $a*$p;
			}
		}
	}
	#print @grid;
	return $price;
}


@grid = @guard;
$part1 = part1();
@grid = @guard;
$part2 = part2();

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);

