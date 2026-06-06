#!perl -w
use strict;

my @inp = (<>); chomp(@inp);

sub paint
{
	my ($diag, @inp) = (@_);
	my %m = (); # Hash table to count vertex visits
	foreach (@inp) {
		my ($x0,$y0,$x1,$y1) = ($1,$2,$3,$4) if (/^(\d+),(\d+) -> (\d+),(\d+)\s*$/);
		
		my $dy = $y1 > $y0? 1 : $y1 == $y0 ? 0 : -1; # Step size/direction
		my $dx = $x1 > $x0? 1 : $x1 == $x0 ? 0 : -1;
		next if (($dx & $dy) && ($diag == 0)); # Include a diagonal line?
		
		for (my ($x, $y) = ($x0, $y0); 1; $x += $dx, $y += $dy) {
				$m{"$x;$y"}++;
				last unless (($x-$x1) | ($y-$y1));
		}
	}
	my $overlap = 0;
	foreach (keys %m) {
		$overlap++ if ($m{$_} > 1);
	}
	return $overlap;
}

printf("5a: %d\n", paint(0, @inp));
printf("5b: %d\n", paint(1, @inp));
