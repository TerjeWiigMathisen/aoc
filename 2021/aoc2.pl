#!perl -w

my $pos = 0;
my $depth = 0;

while (<>) {
	chomp;
	if (/^forward\s+(\d+)$/) {
		$pos += $1;
	}
	elsif (/^down\s+(\d+)$/) {
		$depth += $1;
	} elsif (/^up\s+(\d+)$/) {
		$depth -= $1;
	}
}

printf("%d, %d, %d\n",$pos, $depth, $pos*$depth);
