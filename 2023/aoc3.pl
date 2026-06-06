#!perl -w
use strict;

my $part1 = 0;
my $part2 = 0;

#Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
my @b = ();

while (<>) {
	chomp;
	push(@b,'.'.$_.'.');
}
my $l = '.' x length($b[0]);
@b = ($l, @b,$l);

print join("\n",@b); print "\n";

my %p = ();

for (my $y = 1; $y < scalar(@b)-1; $y++) {
	for (my $x = 1; $x < length($l)-1; $x++) {
		if (substr($b[$y],$x,1) =~ /[^\d\.]/) {
			my $star = substr($b[$y],$x,1) eq '*';
			my %seen = ();
			my $gears = 0;
			my $prod = 1;
			for (my $n = $y-1; $n <= $y+1; $n++) {
				for (my $e = $x-1; $e <= $x+1; $e++) {
					$p{"$e;$n"} += 1;
					if ($star && substr($b[$n],$e,1) =~ /\d/ && !defined($seen{"$e;$n"})) {
						$seen{"$e;$n"}++;
						$gears++;
						my $e0 = 0;
						my $e1 = 0;
						for ($e0 = $e-1; substr($b[$n],$e0,1) =~ /\d/; $e0--) {
							$seen{"$e0;$n"}++;
						}
						$e0++;
						for ($e1 = $e+1; substr($b[$n],$e1,1) =~ /\d/; $e1++) {
							$seen{"$e1;$n"}++;
						}
						my $g = substr($b[$n],$e0,$e1-$e0);
						printf("%d,%d,%d,%d\n",$e,$n,$g,$gears);
						$prod *= $g;
					}
				}
			}
			if ($gears == 2) { 
				$part2 += $prod;
				printf("%d\n", $prod);
			}
		}
	}
}

for (my $y = 1; $y < scalar(@b)-1; $y++) {
	for (my $x = 1; $x < length($l)-1; $x++) {
		if (substr($b[$y],$x,1) =~ /\d/) {
			my $num = '';
			my $found = 0;
			my $maxgear = 0;
			for (my $i = $x; substr($b[$y],$i,1) =~ /\d/; $i++) {
				$num .= substr($b[$y],$i,1);
				if (defined($p{"$i;$y"})) {
					$found = 1;
				}
				$x++;
			}
			$part1 += $num*$found;
#			printf("%d,%d %d %d\n", $x,$y,$num,$found);
		}
	}
}

printf("%s\n%s\n", $part1, $part2);
