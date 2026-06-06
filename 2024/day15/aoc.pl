#!perl -w
use strict;
use Time::HiRes qw (time);
use English;
use warnings;
no warnings 'recursion';
#use bigint;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

my %robots = ();
while (<>) {
	push(@lines, $_);
}
my $oneline = join('',@lines);

my ($map,$moves) = split(/\n\n/,$oneline);
my @map = split(/\n/,$map);
my ($W,$H) = (length($map[0])-2,scalar(@map)-2);
#printf("Map size (%d,%d)\n",$W,$H);

my ($rx,$ry) = (1,1);
for ($ry = 1; $ry <= $H; $ry++) {
	$rx = index($map[$ry],"@");
	last if ($rx > 0);
}
#printf("Robot starts at (%d,%d)\n",$rx,$ry);
my @map2 = @map;
my $rx2 = $rx*2;
my $ry2 = $ry;
foreach (@map2) {
	s/#/##/g;
	s/\./\.\./g;
	s/O/\[\]/g;
	s/@/@\./;
}
#printf("Robot 2 starts at (%d,%d)\n",$rx2,$ry2);
#printf("%s\n\n",join("\n",@map2));

#part1

my %dx = ('>',1,'v',0,'<',-1,'^',0);
my %dy = ('>',0,'v',1,'<',0,'^',-1);

foreach (split(//, $moves)) {
	#printf("Move $_\n");
	my $dx = $dx{$_};
	my $dy = $dy{$_};
	next unless (defined($dy));
	my ($x,$y) = ($rx+$dx,$ry+$dy);
	# Try to move:
	my $moves = 1;
	while (substr($map[$y],$x,1) eq 'O') {
		($x,$y) = ($x+$dx,$y+$dy);
		$moves++;
	}
	if (substr($map[$y],$x,1) eq '.') { # Move stack
		while ($moves--) {
			my ($px,$py) = ($x-$dx,$y-$dy);
			substr($map[$y],$x,1) = substr($map[$py],$px,1);
			($x,$y) = ($px,$py);
		}
		substr($map[$ry],$rx,1) = '.';
		($rx,$ry) = ($rx+$dx,$ry+$dy);
		#printf("%s\n\n",join("\n",@map));
	}
}
for (my $y = 1; $y <= $H; $y++) {
	for (my $x = 1; $x <= $W; $x++) {
		if (substr($map[$y],$x,1) eq 'O') {
			$part1 += $y*100+$x;
		}
	}
}

sub pushblock
{
	my ($x,$py,$dy) = @_;
	my $y = $py+$dy;
	my $c = substr($map[$y],$x,1);
	if ($c eq '.') { 
		substr($map[$y],$x,1) = substr($map[$py],$x,1);
		substr($map[$py],$x,1) = '.';
		return;
	}
	die("pushing #") if ($c eq '#');
	if ($c eq ']') {
		pushblock($x-1,$y,$dy);
		pushblock($x,$y,$dy);
		#substr($map[$y],$x-1,1) = substr($map[$py],$x-1,1);
		substr($map[$y],$x,1) = substr($map[$py],$x,1);
		substr($map[$py],$x,1) = '.';
		return;
	}
	if ($c eq '[') {
		pushblock($x,$y,$dy);
		pushblock($x+1,$y,$dy);
		#substr($map[$y],$x,1) = substr($map[$py],$x,1);
		substr($map[$y],$x,1) = substr($map[$py],$x,1);
		substr($map[$py],$x,1) = '.';
		return;
	}
	if ($c eq 'O') {
		pushblock($x,$y,$dy);
		#substr($map[$y],$x,1) = substr($map[$py],$x,1);
		substr($map[$y],$x,1) = substr($map[$py],$x,1);
		substr($map[$py],$x,1) = '.';
		return;
	}
}

sub canpush
{
	my ($x,$y,$dy) = @_;
	$y += $dy;
	my $c = substr($map[$y],$x,1);
	if ($c eq '.') { return 1; }
	if ($c eq '#') { return 0; }
	if ($c eq ']') {
		return canpush($x-1,$y,$dy) && canpush($x,$y,$dy);
	}
	if ($c eq '[') {
		return canpush($x,$y,$dy) && canpush($x+1,$y,$dy);
	}
	if ($c eq 'O') {
		return canpush($x,$y,$dy);
	}
	die("Should not get here in canpush");
}

#part2
@map = @map2;
($rx,$ry) = ($rx2,$ry2);
foreach (split(//, $moves)) {
	my $dx = $dx{$_};
	my $dy = $dy{$_};
	next unless (defined($dy));
	#printf("Move $_\n");
	my ($x,$y) = ($rx+$dx,$ry+$dy);
	if ($_ eq '>') {
		# Try to move right:
		my $len = 1;
		while (substr($map[$y],$x,1) eq '[') {
			$x += 2;
			$len += 2;
		}
		if (substr($map[$ry],$x,1) eq '.') { # Move stack
			my $s = substr($map[$ry],$rx,$len);
			substr($map[$ry],$rx+1,$len) = $s;
			substr($map[$ry],$rx,1) = '.';
			($rx,$ry) = ($rx+$dx,$ry+$dy);
		}
	}
	elsif ($_ eq '<') {
		# Try to move left:
		my $len = 1;
		while (substr($map[$y],$x,1) eq ']') {
			$x -= 2;
			$len += 2;
		}
		if (substr($map[$y],$x,1) eq '.') { # Move stack
			my $s = substr($map[$y],$x+1,$len);
			substr($map[$ry],$x,$len) = $s;
			substr($map[$ry],$rx,1) = '.';
			($rx,$ry) = ($rx+$dx,$ry+$dy);
		}
	}
	elsif ($_ eq '^') {
		# Try to move up:
		if (canpush($rx,$ry,-1)) {
			pushblock($rx,$ry,-1);
			$ry -= 1;
		}
	}
	else { # "v"
		if (canpush($rx,$ry,+1)) {
			pushblock($rx,$ry,+1);
			$ry += 1;
		}
	}
	#printf("%s\n\n",join("\n",@map));
}

for (my $y = 1; $y <= $H; $y++) {
	for (my $x = 2; $x <= $W*2; $x++) {
		if (substr($map[$y],$x,1) eq '[') {
			#printf("y=$y, x= $x\n");
			$part2 += $y*100+$x;
		}
	}
}


my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);

