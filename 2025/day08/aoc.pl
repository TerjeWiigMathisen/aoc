#!perl -w
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

while (<>) {
	chomp;
	push(@lines, $_);
}

#part1

sub d2
{
	my ($x,$y,$z) = @_;
	my $d2 = $x*$x+$y*$y+$z*$z;
	return $d2;
}

my %dist2 = ();
for (my $i = 0; $i < scalar(@lines)-1; $i++) {
	my ($x0,$y0,$z0) = split(/,/,$lines[$i]);
	for (my $j = $i+1; $j < scalar(@lines);$j++) {
		my ($x1,$y1,$z1) = split(/,/,$lines[$j]);
		$dist2{"$i,$j"} =  d2($x0-$x1,$y0-$y1,$z0-$z1);
	}
}
my @distances = sort {$dist2{"$a"} <=> $dist2{$b} } keys %dist2;

my %circ = ();
my %root = ();
my %sizes = ();
my @size = ();

for (my $a = 0; $a < scalar(@lines); $a++) {
	$root{$a} = $a;
	$sizes{$a} = 1;
	$circ{$a}->{$a}++;
}

my $PAIRS = (scalar(@lines) < 100) ? 10 : 357;

for (my $close = 0; $close < $PAIRS; $close++) {
	my ($a,$b) = split(/,/,$distances[$close]);
	if ($close+2 >= $PAIRS) {
		printf("%s, %s %s\n", $distances[$close], $lines[$a], $lines[$b]);
	}
	my ($ra,$rb) = ($root{$a},$root{$b});
	if ($ra == $rb) { next; } # Already connected
	if ($rb < $ra) {
		my $t = $rb;
		$rb = $ra;
		$ra = $t;
	}
	# Move circ{rb} to circ{ra}
	foreach (keys %{$circ{$rb}}) {
		$root{$_} = $ra;
		$circ{$ra}->{$_}++;
		$sizes{$ra}++;
	}
	undef(%{$circ{$rb}});
	$sizes{$rb} = 0;

#	@size = ();
#	foreach (keys %sizes) {
#		push(@size,$sizes{$_}); # if ($sizes{$_});
#	}
#	@size = sort {$b <=> $a} @size; 
#	printf("%s\n", join(' ',@size));
}

#my @circsize = sort {$sizes{$b} <=> $sizes{$a}} (keys %sizes); 
#my @ksize = sort {$sizes{$b} <=> $sizes{$a}} keys %sizes;
@size = ();
foreach (keys %sizes) {
	push(@size,$sizes{$_}) if ($sizes{$_});
}
@size = sort {$b <=> $a} @size; 
printf("%s\n", join(' ',@size));
$part1 = $size[0]*$size[1]*$size[2];

# part2
%circ = ();
%root = ();
%sizes = ();
@size = ();
for (my $a = 0; $a < scalar(@lines); $a++) {
	$root{$a} = $a;
	$sizes{$a} = 1;
	$circ{$a}->{$a}++;
}

for (my $close = 0; 1; $close++) {
	my ($a,$b) = split(/,/,$distances[$close]);
	#printf("%s, %s %s\n", $distances[$close], $lines[$a], $lines[$b]);
	my ($ra,$rb) = ($root{$a},$root{$b});
	if ($ra == $rb) { next; } # Already connected
	if ($rb < $ra) {
		my $t = $rb;
		$rb = $ra;
		$ra = $t;
	}
	# Move circ{rb} to circ{ra}
	foreach (keys %{$circ{$rb}}) {
		$root{$_} = $ra;
		$circ{$ra}->{$_}++;
		$sizes{$ra}++;
	}
	undef(%{$circ{$rb}});
	$sizes{$rb} = 0;
	if ($sizes{$ra} == scalar(@lines)) {
		printf("Last pair: %s - %s\n", $lines[$a],$lines[$b]);
		$part2 = (split(/,/,$lines[$a]))[0]*(split(/,/,$lines[$b]))[0];
		last;
	}
}

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
