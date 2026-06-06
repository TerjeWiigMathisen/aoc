#!perl -w
use strict;

my @inp = (<>); chomp(@inp);

@inp = split(/,/, $inp[0]);

sub step_groups
{
	my (@per_age) = @_; # Always 9 elements
	my @a = splice(@per_age,1,8); # Last 8
	$a[6] += $per_age[0]; # Just after spawning
	$a[8] = $per_age[0];  # Newly spawned
	return @a; # Return the new age counts
}

sub simulate
{
	my ($days, @inp) = @_;
	my @ages = (0,0,0,0,0,0,0,0,0);
	foreach (@inp) {
		$ages[$_]++;
	}
	for (my $d = 0; $d < $days; $d++) {
		@ages = step_groups(@ages);
	}
	my $tot = 0;
	foreach (@ages) {
		$tot += $_;
	}
	return $tot;
}

my $tot = simulate(80, @inp);

printf("6a: %d\n", $tot);

$tot = simulate(256, @inp);
printf("6b: %1.0f\n", $tot);

