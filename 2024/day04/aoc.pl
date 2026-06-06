#!perl -w
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ('');

while (<>) {
	chomp;
	push(@lines, '*'.$_.'*');
}

$lines[0] = '*' x length($lines[1]);

push(@lines,$lines[0]);

#part1
for (my $y = 1; $y < scalar(@lines)-1; $y++) {
	for (my $x = 1; $x < length($lines[1])-1; $x++) {
		if (substr($lines[$y],$x,1) eq 'X') {
			if (substr($lines[$y],$x,4) eq 'XMAS') {
				$part1++;
			}
			if (substr($lines[$y],$x-3,4) eq 'SAMX') {
				$part1++;
			}
			# up
			if (substr($lines[$y-1],$x,1) eq 'M' &&
			   substr($lines[$y-2],$x,1) eq 'A' &&
			   substr($lines[$y-3],$x,1) eq 'S') {
				$part1++;
			}
			# down
			if (substr($lines[$y+1],$x,1) eq 'M' &&
			   substr($lines[$y+2],$x,1) eq 'A' &&
			   substr($lines[$y+3],$x,1) eq 'S') {
				$part1++;
			}
			# up-left
			if (substr($lines[$y-1],$x-1,1) eq 'M' &&
			   substr($lines[$y-2],$x-2,1) eq 'A' &&
			   substr($lines[$y-3],$x-3,1) eq 'S') {
				$part1++;
			}
			# dn-left
			if (substr($lines[$y+1],$x-1,1) eq 'M' &&
			   substr($lines[$y+2],$x-2,1) eq 'A' &&
			   substr($lines[$y+3],$x-3,1) eq 'S') {
				$part1++;
			}
			# up-right
			if (substr($lines[$y-1],$x+1,1) eq 'M' &&
			   substr($lines[$y-2],$x+2,1) eq 'A' &&
			   substr($lines[$y-3],$x+3,1) eq 'S') {
				$part1++;
			}
			# dn-right
			if (substr($lines[$y+1],$x+1,1) eq 'M' &&
			   substr($lines[$y+2],$x+2,1) eq 'A' &&
			   substr($lines[$y+3],$x+3,1) eq 'S') {
				$part1++;
			}
		}
	}
}

#part2
for (my $y = 1; $y < scalar(@lines)-1; $y++) {
	for (my $x = 1; $x < length($lines[1])-1; $x++) {
		if (substr($lines[$y],$x,1) eq 'A') {
			#my $dn = (substr($lines[$y-1],$x,1) eq 'M' && substr($lines[$y+1],$x,1) eq 'S');
			#my $up = (substr($lines[$y+1],$x,1) eq 'M' && substr($lines[$y-1],$x,1) eq 'S');
			#my $lt = (substr($lines[$y],$x-1,3) eq 'MAS');
			#my $rt = (substr($lines[$y],$x-1,3) eq 'SAM');
			my $dnrt = (substr($lines[$y-1],$x-1,1) eq 'M' && substr($lines[$y+1],$x+1,1) eq 'S');
			my $dnlt = (substr($lines[$y-1],$x+1,1) eq 'M' && substr($lines[$y+1],$x-1,1) eq 'S');
			my $uprt = (substr($lines[$y+1],$x-1,1) eq 'M' && substr($lines[$y-1],$x+1,1) eq 'S');
			my $uplt = (substr($lines[$y+1],$x+1,1) eq 'M' && substr($lines[$y-1],$x-1,1) eq 'S');
#			if (($dn | $up) && ($lt | $rt)) {
#				$part2++;
#				printf("+ %2d,%2d\n", $x, $y);
#			}
			if (($dnrt | $uplt) && ($uprt | $dnlt)) {
				$part2++;
				#printf("x %2d,%2d\n", $x, $y);
			}
		}
	}
}

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
