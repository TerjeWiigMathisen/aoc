#!perl -w

my $inc = 0;

my $prev = <>;
chomp($prev);

while (<>) {
	chomp;
	$inc++ if ($_ > $prev);
	$prev = $_;
}

printf("%d\n",$inc);
