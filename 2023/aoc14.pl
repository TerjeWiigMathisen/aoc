#!perl -w
use strict;
use Time::HiRes qw(time);

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @inp = (<>); chomp(@inp);
#printf("Input:\n%s\n\n", join("\n",@inp));

sub north
{
	my (@inp) = @_;
	my $xmax = length($inp[0]);
	my $ymax = scalar(@inp);
	my $swap;
	do {
		$swap = 0;
		my @c = split(//,$inp[0]);
		for (my $y = 1; $y < $ymax; $y++) {
			my @p = @c;
			@c = split(//,$inp[$y]);
			for (my $x = 0; $x < $xmax; $x++) {
				if ($p[$x] eq '.' && $c[$x] eq 'O') {
					$p[$x] = 'O';
					$c[$x] = '.';
					$swap++;
				}
			}
			$inp[$y-1] = join("",@p);
			$inp[$y] = join("",@c);
		}
	} while ($swap);
	return @inp;
}
	
sub north1
{
	my (@inp) = @_;
	my $xmax = length($inp[0]);
	my $ymax = scalar(@inp);
	for (my $x = 0; $x < $xmax; $x++) {
		my $top = 0;
		my $bot = 0;
		while (1) {
			for (; $top < $ymax && substr($inp[$top],$x,1) ne '.'; $top++) {}
			$bot = $top+1 if ($bot <= $top);
			for (; $bot < $ymax && substr($inp[$bot],$x,1) eq '.'; $bot++) {}
			last if ($bot >= $ymax);

			if (substr($inp[$bot],$x,1) eq 'O') {
				substr($inp[$top],$x,1) = 'O';
				substr($inp[$bot],$x,1) = '.';
				$top++;
			}
			elsif (substr($inp[$bot],$x,1) eq '#') { # Boundary
				$top = $bot;
			}
		}
	}
	return @inp;
}
	
sub west
{
	my (@inp) = @_;
	my $xmax = length($inp[0]);
	my $ymax = scalar(@inp);
	my $swap;
	do {
		$swap = 0;
		for (my $y = 0; $y < $ymax; $y++) {
			my @c = split(//,$inp[$y]);
			for (my $x = 1; $x < $xmax; $x++) {
				if ($c[$x-1] eq '.' && $c[$x] eq 'O') {
					$c[$x-1] = 'O';
					$c[$x] = '.';
					$swap++;
				}
			}
			$inp[$y] = join("",@c);
		}
	} while ($swap);
	return @inp;
}
	
sub west1
{
	my (@inp) = @_;
	my $xmax = length($inp[0]);
	my $ymax = scalar(@inp);
	for (my $y = 0; $y < $ymax; $y++) {
		my $top = 0;
		my $bot = 0;
		while (1) {
			for (; $top < $xmax && substr($inp[$y],$top,1) ne '.'; $top++) {}
			$bot = $top+1 if ($bot <= $top);
			for (; $bot < $xmax && substr($inp[$y],$bot,1) eq '.'; $bot++) {}
			last if ($bot >= $xmax);

			if (substr($inp[$y],$bot,1) eq 'O') {
				substr($inp[$y],$top,1) = 'O';
				substr($inp[$y],$bot,1) = '.';
				$top++;
			}
			elsif (substr($inp[$y],$bot,1) eq '#') { # Boundary
				$top = $bot;
			}
		}
	}
	return @inp;
}
	
sub south
{
	my (@inp) = @_;
	my $xmax = length($inp[0]);
	my $ymax = scalar(@inp);
	my $swap;
	do {
		$swap = 0;
		my @c = split(//,$inp[$ymax-1]);
		for (my $y = $ymax-2; $y >= 0; $y--) {
			my @p = @c;
			@c = split(//,$inp[$y]);
			for (my $x = 0; $x < $xmax; $x++) {
				if ($p[$x] eq '.' && $c[$x] eq 'O') {
					$p[$x] = 'O';
					$c[$x] = '.';
					$swap++;
				}
			}
			$inp[$y+1] = join("",@p);
			$inp[$y] = join("",@c);
		}
	} while ($swap);
	return @inp;
}

sub south1
{
	my (@inp) = @_;
	my $xmax = length($inp[0]);
	my $ymax = scalar(@inp);
	for (my $x = 0; $x < $xmax; $x++) {
		my $top = $ymax-1;
		my $bot = $top-1;
		while (1) {
			for (; $top >= 0 && substr($inp[$top],$x,1) ne '.'; $top--) {}
			$bot = $top-1 if ($bot>= $top);
			for (; $bot >= 0 && substr($inp[$bot],$x,1) eq '.'; $bot--) {}
			last if ($bot < 0);

			if (substr($inp[$bot],$x,1) eq 'O') {
				substr($inp[$top],$x,1) = 'O';
				substr($inp[$bot],$x,1) = '.';
				$top--;
			}
			elsif (substr($inp[$bot],$x,1) eq '#') { # Boundary
				$top = $bot;
			}
		}
	}
	return @inp;
}

sub east
{
	my (@inp) = @_;
	my $xmax = length($inp[0]);
	my $ymax = scalar(@inp);
	my $swap;
	do {
		$swap = 0;
		for (my $y = 0; $y < $ymax; $y++) {
			my @c = split(//,$inp[$y]);
			for (my $x = $xmax-2; $x >= 0; $x--) {
				if ($c[$x+1] eq '.' && $c[$x] eq 'O') {
					$c[$x+1] = 'O';
					$c[$x] = '.';
					$swap++;
				}
			}
			$inp[$y] = join("",@c);
		}
	} while ($swap);
	return @inp;
}

sub east1
{
	my (@inp) = @_;
	my $xmax = length($inp[0]);
	my $ymax = scalar(@inp);
	for (my $y = 0; $y < $ymax; $y++) {
		my $top = $xmax-1;
		my $bot = $top;
		while (1) {
			for (; $top >= 0 && substr($inp[$y],$top,1) ne '.'; $top--) {}
			$bot = $top-1 if ($bot >= $top);
			for (; $bot >= 0 && substr($inp[$y],$bot,1) eq '.'; $bot--) {}
			last if ($bot < 0);

			if (substr($inp[$y],$bot,1) eq 'O') {
				substr($inp[$y],$top,1) = 'O';
				substr($inp[$y],$bot,1) = '.';
				$top--;
			}
			elsif (substr($inp[$y],$bot,1) eq '#') { # Boundary
				$top = $bot;
			}
		}
	}
	return @inp;
}


sub weigh
{
	my (@inp) = @_;
	# Weigh the 'O' cells:
	my $xmax = length($inp[0]);
	my $ymax = scalar(@inp);
	my $w = 0;
	for (my $y = 0; $y < $ymax; $y++) {
		my $o = $inp[$y];
		$o =~ s/[^O]//g;
		$w += length($o) * ($ymax-$y);
	}
	return $w;
}	

sub part1
{
	my (@inp) = @_;
	@inp = north1(@inp);
	return weigh(@inp);
}

sub part2
{
	my (@inp) = @_;
	my %seen = ();
	$seen{join("\n",@inp)} = 0;
	my $look = 1;
	for (my $cycles = 1; $cycles <= 1000000000; $cycles++) {
		@inp = north1(@inp);
		@inp = west1(@inp);
		@inp = south1(@inp);
		@inp = east1(@inp);
		
		my $key = join("\n",@inp);
#		printf("%s\n\n", $key);
		if ($look) {
			if (defined($seen{$key})) {
	#			printf("%s\n",$key);
				printf("Found a repeating pattern at cycle $cycles from %d\n", $seen{$key});
				my $cyc = $cycles - $seen{$key};
				my $full_cycles = int((1000000000 - $cycles) / $cyc);
				$cycles += $full_cycles*$cyc;
				$look = 0;
			}
			$seen{$key} = $cycles;
		}
	}
	return weigh(@inp);
}

$part1 = part1(@inp);
$part2 = part2(@inp);
	
my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.3fs\n", $used);
