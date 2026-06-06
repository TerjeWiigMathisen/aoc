#!perl
while (<>) {
	my ($c,$n)=split;
	if ($c eq "forward") {
		$p+=$n;
		$d+=$n*$a;
	} elsif ($c eq "down") {
		$a+=$n;
	} elsif ($c eq "up") {
		$a-=$n;
	}
}
print($p*$d);
