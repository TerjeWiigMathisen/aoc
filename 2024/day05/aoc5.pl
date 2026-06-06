#!perl -w
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my $inp = '';

while (<>) {
	$inp = $inp.$_;
}

my ($rules, $orders) = split(/\n\n/, $inp);
my %before = (); # Create a hash of all the rules
foreach (split(/\n/, $rules)) {
	$before{$_}++;
}
my @orders = split(/\n/, $orders);

#part1
sub is_in_order
{
	my (@pages) = @_;
	
	for (my $s = 1; $s < scalar(@pages); $s++) {
		my $sp = $pages[$s];
		for (my $f = 0; $f < $s; $f++) {
			my $fp = $pages[$f];
			if (defined($before{"$sp|$fp"})) {
				return 0; 
			}
		}
	}
	return $pages[scalar(@pages)>>1];
}

sub reorder
{
	my (@pages) = @_;
	my @ord = ();
	my $target = scalar(@pages)>>1;
	while (@pages) {
		# Try all pairs of pages to find a legal first page:
		for (my $f = 0; $f < scalar(@pages); $f++) {
			my $fp = $pages[$f];
			my $ok = 1;
			#for (my $s = 0; $s < scalar(@pages); $s++) {
			for (my $s = $f+1; $s < scalar(@pages); $s++) {
				#next if $s == $f;
				my $sp = $pages[$s];
				if (defined($before{"$sp|$fp"})) {
					$ok = 0;
					last;
				}
			}
			if ($ok) {
				if (scalar(@ord) == $target) {
					#printf("early: %s\n",join(" ",@ord));
					return $fp;
				}
				push(@ord, $fp);
				splice(@pages,$f,1);
				last;
			}
		}
	}
	printf("final: %s\n",join(" ",@ord));
	return $ord[scalar(@ord)>>1];
}	

my ($ordered, $unordered, $or, $up) = (0,0,0,0);
foreach (@orders) {
	my @pages = split(/,/);
	my $p1 = is_in_order(@pages);
	if ($p1) {
		$part1 += $p1;
		$or += scalar(@pages);
		$ordered++;
	}
	else {
		$part2 += reorder(@pages);
		$up += scalar(@pages);
		$unordered++;
	}
}

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
printf("There was %d ordered (%d pages) and %d unordered (%d pages) page sets\n",$ordered, $or, $unordered, $up);
