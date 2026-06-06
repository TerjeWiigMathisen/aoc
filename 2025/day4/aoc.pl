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
	push(@lines, '.'.$_.'.');
}

my $blank = '.' x length($lines[0]);
unshift(@lines,$blank);
push(@lines,$blank);

my $ROWS = scalar(@lines)-2;
my $COLS = length($lines[0])-2;

#part1
sub removable
{
	my $part1 = 0;
	for (my $y = 1; $y <= $ROWS; $y++) {
		for (my $x = 1; $x <= $COLS; $x++) {
			if (substr($lines[$y],$x,1) eq '@') {
				my $n = 0;
				if (substr($lines[$y-1],$x-1,1) eq '@') { $n++; }
				if (substr($lines[$y-1],$x,1) eq '@') { $n++; }
				if (substr($lines[$y-1],$x+1,1) eq '@') { $n++; }
				if (substr($lines[$y],$x-1,1) eq '@') { $n++; }
				if (substr($lines[$y],$x+1,1) eq '@') { $n++; }
				if (substr($lines[$y+1],$x-1,1) eq '@') { $n++; }
				if (substr($lines[$y+1],$x,1) eq '@') { $n++; }
				if (substr($lines[$y+1],$x+1,1) eq '@') { $n++; }

				if ($n < 4) {
					$part1++;
					substr($lines[$y],$x,1) = 'x';
				}
			}
		}
	}
	return $part1;
}

$part1 = removable();
$part2 = $part1;
my $p;
do {
	$p = removable();
	$part2 += $p;
} while ($p);

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
