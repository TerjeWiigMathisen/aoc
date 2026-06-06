#!perl -w

use strict;

my @inp = (<>);
chomp(@inp);

sub count  # Count "1" bits per column
{
	my (@inp) = @_;
	my @ones = ();

	foreach (@inp) {
		my @bits = split(//);
		for (my $b = 0; $b < length($_); $b++) {
			$ones[$b] += $bits[$b];
		}
	}
	return @ones;
}

my @ones = count(@inp);

my $gamma = '';
for (my $b = 0; $b < length($inp[0]); $b++) {
	$gamma .= $ones[$b]*2 > scalar(@inp) ? "1" : "0";
}

my $eps = $gamma;
$eps =~ tr/10/01/; # Flip all the bits!

sub bin2dec { return unpack("N", pack("B32", substr("0" x 32 . shift, -32))); } 

printf("%d\n", bin2dec($gamma)*bin2dec($eps)); # Part a

sub filter
{
	my ($bpos, $xor, @inp) = @_;
	my @ones = count(@inp);
	
	my @out = ();
	# Check the one bit count in this column, XOR the result if looking for lowest count
	my $lookfor = sprintf("%d", ($ones[$bpos]*2 >= scalar(@inp)) ^ $xor);
	foreach (@inp) {
		push(@out, $_) if (substr($_, $bpos, 1) eq $lookfor);
	}
	return @out;
}

sub filterloop
{
	my ($xor, @o) = @_;
	for (my $b = 0; scalar(@o) > 1; $b++) {
		@o = filter($b, $xor, @o);
	}
	return bin2dec($o[0]);
}

my $ox = filterloop(0, @inp);
my $co2 = filterloop(1, @inp);

printf("%d\n", $ox*$co2); # Part b