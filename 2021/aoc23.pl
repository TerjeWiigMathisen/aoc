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

my $DEBUG = 0;
my $part1;
my $part2;

my %visited;

my $ROWS;
my $COLS;

my $target;
my $moves = 0;

sub maketarget
{
	my @t = @inp;
	$ROWS = scalar(@inp)-2;
	$COLS = length($inp[0])-2;
	$moves = 0;
	%visited = ();
	my $l = 'A';
	for (my $x = 3; $x <= 9; $x+=2) {
		for (my $y = 2; $y <= $ROWS; $y++) {
			substr($t[$y],$x,1) = $l;
		}
		$l++;
	}
	$target = join("\n",@t);
}

my $solution;

#dmp(0,0,@t);

my %energy = ('A'=>1,'B'=>10,'C'=>100,'D'=>1000);
my %target = ('A'=>3,'B'=>5,'C'=>7,'D'=>9);

sub dmp
{
	my ($lvl, $energy, @inp) = @_;
	printf(STDERR "%5d, %5d\n",$lvl,$energy);
	printf(STDERR "%s\n\n",join("\n",@inp));
}

sub move
{
	my ($lvl, $energy, $sol, @inp) = @_;
	$moves++;
	dmp($lvl,$energy,@inp) if ($lvl < 1);
	my $key = join("\n",@inp);
	return if (defined($visited{$key}) && $energy >= $visited{$key});

	if ($key eq $target) {
		$solution = $sol;
	}
	$visited{$key} = $energy;
	
	# Find a movable piece;
	for (my $y = 1; $y <= $ROWS; $y++) {
		for (my $x = 1; $x <= $COLS; $x++) {
			my $piece = substr($inp[$y],$x,1);
			if ($piece ge "A") {
				my $e = $energy{$piece};
				my $t = $target{$piece};
				# Where can it move?
				if ($y > 1) { # Try to move to one of the top row spots:
					my $my;
					for ($my = $y-1; $my > 1 && 
							substr($inp[$my],$x,1) eq '.'; $my--) {}
					next if ($my > 1); # Blocked going up
					# Already correct?
					if ($x == $t) {
						for ($my = $y+1; $my <= $ROWS && 
								substr($inp[$my],$x,1) eq $piece; $my++) {}
						next if ($my > $ROWS); # Already OK!
					}

					if ($piece le 'B') {
						# try left first:
						for (my $mx = $x-1; $mx >= 1; $mx--) {
							last if (substr($inp[1],$mx,1) ne '.');
							next if ($mx & 1 && $mx >= 3);

							my @m = @inp;
							substr($m[$y],$x,1) = '.';
							substr($m[1],$mx,1) = $piece;
							my $s = $sol.sprintf(" %s%d%d->%d%d",$piece,$x,$y,$mx,1);
							move($lvl+1, $energy + $e*($y-1 + $x-$mx), $s, @m);
						}
						# try right:
						for (my $mx = $x+1; $mx <= $COLS; $mx++) {
							last if (substr($inp[1],$mx,1) ne '.');
							next if ($mx & 1 && $mx < 10);

							my @m = @inp;
							substr($m[$y],$x,1) = '.';
							substr($m[1],$mx,1) = $piece;
							my $s = $sol.sprintf(" %s%d%d->%d%d",$piece,$x,$y,$mx,1);
							move($lvl+1, $energy + $e*($y-1 + $mx-$x), $s, @m);
						}
					}
					else {
						# try right first:
						for (my $mx = $x+1; $mx <= $COLS; $mx++) {
							last if (substr($inp[1],$mx,1) ne '.');
							next if ($mx & 1 && $mx < 10);

							my @m = @inp;
							substr($m[$y],$x,1) = '.';
							substr($m[1],$mx,1) = $piece;
							my $s = $sol.sprintf(" %s%d%d->%d%d",$piece,$x,$y,$mx,1);
							move($lvl+1, $energy + $e*($y-1 + $mx-$x), $s, @m);
						}
						# try left:
						for (my $mx = $x-1; $mx >= 1; $mx--) {
							last if (substr($inp[1],$mx,1) ne '.');
							next if ($mx & 1 && $mx >= 3);

							my @m = @inp;
							substr($m[$y],$x,1) = '.';
							substr($m[1],$mx,1) = $piece;
							my $s = $sol.sprintf(" %s%d%d->%d%d",$piece,$x,$y,$mx,1);
							move($lvl+1, $energy + $e*($y-1 + $x-$mx), $s, @m);
						}
					}
				}
				else { # Must move to destination!
					# Is the target available, i.e. only correct pieces?
					my $r = $ROWS;
					while (substr($inp[$r],$t,1) eq $piece) { $r--;}
					next if (substr($inp[$r],$t,1) ne '.');

					# Is the horizontal path free?
					my $dx = $t > $x ? 1 : -1;
					my $sx;
					for ($sx = $x+$dx; $sx != $t && substr($inp[1],$sx,1) eq '.'; 		$sx+= $dx) {}
					next unless ($sx == $t);
					
					my @m = @inp;
					substr($m[$y],$x,1) = ".";
					substr($m[$r],$t,1) = $piece;
					my $s = $sol.sprintf(" %s%d%d->%d%d",$piece,$x,$y,$t,$r);
					move($lvl+1, $energy + $e*(abs($t-$x)+($r-1)), $s, @m);
				}
			}
		}
	}
}

maketarget();
move(0,0,'',@inp);
$part1 = $visited{$target};
my $sol1 = $solution;

splice(@inp,3,0,"  #D#C#B#A#","  #D#B#A#C#");
maketarget();
move(0,0,'',@inp);
$part2 = $visited{$target};

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %d\n", $part1);
printf("Part2: %1.0f\n", $part2);
printf("%s\n", $solution);

printf(STDERR "Move calls: %d\n", $moves);