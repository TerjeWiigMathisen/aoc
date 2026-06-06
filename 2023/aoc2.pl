#!perl -w
use strict;

my @limit = (12,13,14);

my $part1 = 0;
my $part2 = 0;

#Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

sub check
{
	my ($game, @samples) = @_;
	foreach (@samples) {
		if (/(\d+) red/) {
			if ($1 > $limit[0]) { return 0;}
		}
		if (/(\d+) green/) {
			if ($1 > $limit[1]) { return 0;}
		}
		if (/(\d+) blue/) {
			if ($1 > $limit[2]) { return 0;}
		}
	}
	return substr($game,5);
}

sub power
{
	my ($game, @samples) = @_;
	my @req = (0,0,0);
	foreach (@samples) {
		if (/(\d+) red/) {
			if ($1 > $req[0]) { $req[0] = $1;}
		}
		if (/(\d+) green/) {
			if ($1 > $req[1]) { $req[1] = $1;}
		}
		if (/(\d+) blue/) {
			if ($1 > $req[2]) { $req[2] = $1;}
		}
	}
	return $req[0]*$req[1]*$req[2];
}
		
while (<>) {
	chomp;
	my ($game, $samples) = split(/:/);
	my @samples = split(/;/,$samples);
	
	$part1 += check($game,@samples);
	$part2 += power($game,@samples);
	
}

printf("%s\n%s\n", $part1, $part2);
