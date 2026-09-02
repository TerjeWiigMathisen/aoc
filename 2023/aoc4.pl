#!perl -w
use strict;

my $part1 = 0;
my $part2 = 0;

my @cards = ();
my @copies = ();

while (<>) { # Gather the cards and verify the regex
	chomp;
	if (/Card\s+(\d+): (.*) \| (.*)$/) {
		push(@cards,$_);
		push(@copies,1);
	}
	else {
		die("Bad input $_");
	}
}

for (my $c = 0; $c < scalar(@cards); $c++) {
	$_ = $cards[$c];
	next unless (/Card\s+(\d+): (.*) \| (.*)$/); # Will never exit since we checked this above
	my ($card, $winners, $ticket) = ($1,$2,$3);
	my %winhash = ();
	foreach(split(/\s+/,$winners)) {
		next unless (/\d/);
		$winhash{$_} = 1;
	}
	my $price = 0;
	foreach (split(/\s+/,$ticket)) {
		next unless (/\d/);
		if (defined($winhash{$_})) { 
			$price++; 
		}
	}
	if ($price) {
		$part1 += 1 << ($price-1);
		my $curr = $copies[$c]; # How many copies do we have of the current card?
		for (my $i = 1; $i <= $price; $i++) {
			$copies[$c+$i] += $curr;
		}
	}
}

foreach (@copies) { # Count all cards
	$part2 += $_;
}

printf("%s\n%s\n", $part1, $part2);
