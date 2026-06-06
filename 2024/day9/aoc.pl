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
my @blocks = split(//,$lines[0]);

my $id = 0;
if (scalar(@blocks) & 1) {
	push(@blocks,0); # Tail free space if needed
}
my @map = ();
$id = 0;
for (my $i = 0; $i < scalar(@blocks); $i+=2) {
	my $use = $blocks[$i];
	for (my $u = 0; $u < $use; $u++) {
		push(@map,$id);
	}
	$id++;
	my $free = $blocks[$i+1];
	for (my $u = 0; $u < $free; $u++) {
		push(@map,-1);
	}
}
my @p2map = @map;

for (my $i = 0; $i<scalar(@map);$i++) {
	if ($map[$i] == -1) {
#		while (scalar(@map) > $i && $map[-1] == -1) {
		while ($map[-1] == -1) {
			pop(@map);
		}
		if ($i+1 >= scalar(@map)) {
			last;
		}
		$map[$i] = pop(@map);
	}
}

for (my $i = 0; $i < scalar(@map); $i++) {
	$part1 += $i*$map[$i];
}

my $used;

# part2
@map = @p2map;

my @files = ();
my @free = ();
my $pos = 0;
$id = 0;
for (my $i = 1; $i < scalar(@blocks); $i+=2) {
	my $len = $blocks[$i-1];
	push(@files,$id,$len,$pos);
	$id++;
	$pos += $len;
	my $flen = $blocks[$i];
	if ($flen) {
		push(@free,$flen,$pos);
		$pos += $flen;
	}
}

my $freescans = 0;

for (my $i = scalar(@files)-3; $i > 0; $i-=3) {
	my $id = $files[$i];
	my $len = $files[$i+1];
	my $pos = $files[$i+2];
	
	for (my $f = 0; $f < scalar(@free); $f+=2) {
		my $fpos = $free[$f+1];
		last if ($fpos >= $pos);
		$freescans++;
		my $flen = $free[$f];

		if ($flen >= $len) {
			$free[$f] -= $len;
			$free[$f+1] += $len;
			$files[$i+2] = $fpos;
			
			if ($free[$f] == 0) {
				splice(@free,$f,2);
			}
			last;
		}
	}
}

my $p2 = 0;
for (my $f = 0; $f < scalar(@files); $f+=3) {
	my $id = $files[$f];
	my $len = $files[$f+1];
	my $pos = $files[$f+2];
	for (my $p = $pos; $p < $pos+$len; $p++) {
		$p2 += $id * $p;
	}
}
$part2 = $p2;

$used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);

printf("%d free list lookups for %d files\n",$freescans, scalar(@files)/3);