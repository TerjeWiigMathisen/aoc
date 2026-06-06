#!perl -w

my $inc = 0;

my $p2 = <>;
chomp($p2);
my $p1 = <>;
chomp($p1);
my $p0 = <>;
my $prev = $p2 + $p1 + $p0;

while (<>) {
	chomp;
	my $curr = $_ + $p0 + $p1;
	$inc++ if ($curr > $prev);
	$p1 = $p0; 
	$p0 = $_; 
	$prev = $curr;
}

printf("%d\n",$inc);
