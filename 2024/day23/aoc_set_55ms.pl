#!perl -w
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

my %link;
while (<>) {
	chomp;
	push(@lines, $_);
	my ($a,$b) = split(/\-/);
	$link{$a}->{$b}++;
	$link{$b}->{$a}++;
}
printf("Found %d computers\n",scalar (keys %link));

#part1

my @tcomp = ();

foreach (keys %link) {
	if (substr($_,0,1) eq 't') {
		push(@tcomp, $_);
	}
}
printf("Found %d computers starting with 't'\n",scalar (@tcomp));

@tcomp = sort @tcomp;

my %set3 = ();
foreach (@tcomp) {
	my $t = $_;
	my @nbor = keys %{$link{$t}};
	for (my $j = 1; $j < scalar(@nbor); $j++) {
		my $b = $nbor[$j];
		
		for (my $i = 0; $i < $j; $i++) {
			my $a = $nbor[$i];
			if (defined($link{$a}->{$b})) {
				$set3{join(',',sort ($t,$a,$b))}++;
			}
		}
	}
}

$part1 = scalar keys %set3;
printf("%d\n", scalar keys %set3);

# part2
my @comps = sort keys %link;
				   
my @maxlist = ();

#my %seen = ();
sub grow
{
	my ($curr, @links) = @_;
	#
	my @cluster = @{$curr};
	#printf("Try %s\n", join(',',@maxlist));
	return if (scalar(@cluster) + scalar(@links) <= scalar(@maxlist));
	if (scalar(@cluster) > scalar(@maxlist)) {
		@maxlist = @cluster;
		#printf("Growing %s\n", join(',',@maxlist));
	}
	while (@links) {
		my $t = shift @links;
		my @rem = ();
		foreach (@links) {
			if (defined($link{$_}->{$t})) {
				push(@rem, $_);
			}
		}
		push(@cluster,$t);
		grow(\@cluster,@rem);
		pop(@cluster);
	}
}

my @rem = @comps;
while (scalar(@rem) > scalar(@maxlist)) {
	my $first = shift @rem;
	my @c = ($first);
	my @tail = sort keys %{$link{$first}};
	grow(\@c, @tail);
}
$part2 = join(',',@maxlist);

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
