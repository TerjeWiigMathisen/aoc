#!perl -w
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @board = ('');

my $x = 0;
my $y = 0;
while (<>) {
	chomp;
	#printf("%s\n",$_);
	if (index($_,"^") >= 0) {
		$x = index($_,"^");
		$y = scalar(@board);
		substr($_,$x,1) = '.';
		$x++;
		#printf("Guard at $x,$y\n");
	}
	push(@board,'*'.$_.'*');
}
$board[0] = '*' x length($board[1]);
push(@board,$board[0]);

my @dx = (0,1,0,-1);
my @dy = (-1,0,1,0);

my %jump = ();
my %seen = ();

sub part1
{
	my ($x,$y,@board) = @_;
	my $dir = 0;
	$seen{"$x;$y"} = $dir;
	my $loop = 0;
	while (1) {
		#printf("%2d,%2d,%2d\n",$x,$y,$dir);
		my ($nx,$ny) = ($x+$dx[$dir],$y+$dy[$dir]);
		my $c = substr($board[$ny],$nx,1);
		last if ($c eq '*');
		if ($c eq '#') {
			$dir = ($dir+1) & 3;
			# How far can we move from here?
			my ($mx,$my) = ($x+$dx[$dir],$y+$dy[$dir]);
			while (substr($board[$my],$mx,1) eq '.') {
				$seen{"$mx;$my"} = $dir unless (defined($seen{"$mx;$my"}));
				($mx,$my) = ($mx+$dx[$dir],$my+$dy[$dir]);
			}
			($mx,$my) = ($mx-$dx[$dir],$my-$dy[$dir]);
			$jump{"$x;$y;$dir"} = "$mx;$my";
			($x,$y) = ($mx,$my);
		}
		else {
			$x = $nx; $y = $ny;
			$seen{"$x;$y"} = $dir unless (defined($seen{"$x;$y"}));
		}
	}
	return (scalar(keys %seen), %seen);
}

sub part2isloop
{
	my ($x,$y,$dir,$bx,$by) = @_;
	my %upright = ();
	substr($board[$by],$bx,1) = '#';
	while (1) {
		my ($ddx,$ddy) = ($dx[$dir],$dy[$dir]);
		my ($nx,$ny) = ($x+$ddx,$y+$ddy);
		my $c = substr($board[$ny],$nx,1);
		while ($c eq '.') {
			($x,$y) = ($nx,$ny);
			($nx,$ny) = ($x+$ddx,$y+$ddy);
			$c = substr($board[$ny],$nx,1);
		}
		if ($c eq '#') {
			if ($dir == 0) {
				if (defined($upright{"$x;$y"})) {
					substr($board[$by],$bx,1) = '.';
					return 1;
				}
				$upright{"$x;$y"}++;
			}
			$dir = ($dir+1) & 3;
		}
		elsif ($c eq '*') {
			substr($board[$by],$bx,1) = '.';
			return 0;
		}
	}
	die;
}

sub part2
{
	my ($ox,$oy,@board) = @_;
	my $part2 = 0;
	my ($part1,%visited) = part1($ox,$oy,@board);
	foreach (keys %visited) {
		next if ($_ eq "$ox;$oy");
		my ($x,$y) = split(/;/);
		substr($board[$y],$x,1) = '#';
		my $p2 = part2isloop($ox,$oy,0,$x,$y,@board);
		if ($p2) {
			$part2++;
			#printf("Found loop at %d,%d\n",$x,$y);
		}
		substr($board[$y],$x,1) = '.';
	}
	return $part1, $part2;
}

sub part2combo
{
	my ($x,$y) = @_;
	my $part2 = 0;
	my $part1 = 0;
	my $dir = 0;
	my %seen = ();
	while (1) {
		$seen{"$x;$y"}++;
		my ($ddx,$ddy) = ($dx[$dir],$dy[$dir]);
		my ($nx,$ny) = ($x+$ddx,$y+$ddy);
		my $c = substr($board[$ny],$nx,1);
		while ($c eq '.') {
			my $p2 = part2isloop($x,$y,$dir,$nx,$ny);
			if ($p2) {
				$part2++;
			}
			($x,$y) = ($nx,$ny);
			$seen{"$x;$y"}++;
			($nx,$ny) = ($x+$ddx,$y+$ddy);
			$c = substr($board[$ny],$nx,1);
		}
		if ($c eq '#') {
			$dir = ($dir+1) & 3;
		}
		elsif ($c eq '*') {
			return (scalar(keys %seen),$part2);
		}
	}
}

my $a = q(
sub part2f
{
	my ($ox,$oy,@board) = @_;
	my %wall;
	for (my $y = 1; $y < scalar(@board)-1; $y++) {
		my $x = 0;
		while (1) {
			$x = index($board[$y],'#',$x);
			last if ($x < 0);
			$wall{"$x;$y"}++;
			$x++;
		}
	}
	printf("Found %d walls\n", scalar(keys %wall));
	substr($board[$oy],$ox,1) = '.';
	my %next = ();
	foreach (keys %wall) {
		my ($wx,$wy) = split(/;/);
		for (my $d = 0; $d < 4; $d++) {
			my ($ix,$iy) = ($wx-$dx[$d],$wy-$dy[$d]);
			$d = ($d+1)&3;
			while (substr($board[$iy],$ix,1) eq '.') {
				$ix += $dx[$d]; $dy += $dy[$d];
			}
			$next{"$wx,$wy"} = "$ix;$iy;$d";
		}
	}
}
);

my %v;
#($part1,$part2,%v) = part1($x,$y,@board);
#part2f($x,$y,@board);
($part1, $part2) = part2combo($x,$y);

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
