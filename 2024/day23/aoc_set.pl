#!perl -w
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

my %unidir = ();
my %omnidir = ();
my %comps = ();
my %all =();
while (<>) {
	chomp;
	push(@lines, $_);
	my ($a,$b) = split(/\-/);
	$all{$_}++;
	$all{"$b-$a"}++;
	$comps{$a}++;
	$comps{$b}++;
}

my @comps = sort keys %comps;
foreach (@lines) {
	my ($a,$b) = split(/\-/);
	if ($a lt $b) {
		$unidir{$a}->{$b}++;
	}
	else {
		$unidir{$b}->{$a}++;
	}
	$omnidir{$a}->{$b}++;
	$omnidir{$b}->{$a}++;
}
printf("Found %d computers\n",scalar (@comps));

#part1

my @tcomp = ();

foreach (keys %unidir) {
	if (substr($_,0,1) eq 't') {
		push(@tcomp, $_);
	}
}
printf("Found %d computers starting with 't'\n",scalar (@tcomp));

@tcomp = sort @tcomp;

my %set3 = ();
foreach (@tcomp) {
	my $t = $_;
	my @nbor = keys %{$omnidir{$t}};
	for (my $j = 1; $j < scalar(@nbor); $j++) {
		my $b = $nbor[$j];
		
		for (my $i = 0; $i < $j; $i++) {
			my $a = $nbor[$i];
			if (defined($all{"$a-$b"})) {
				$set3{join(',',sort ($t,$a,$b))}++;
			}
		}
	}
}

$part1 = scalar keys %set3;
printf("%d\n", scalar keys %set3);

# part2
  
my @maxlist = ();
my @cluster = ();

#my %seen = ();
sub grow
{
	my (@all) = @_;
	#
	return if (scalar(@cluster) + scalar(@all) <= scalar(@maxlist));
	#printf("Try %s %d\n", join(',',@cluster), scalar(@all));
	if (scalar(@cluster) > scalar(@maxlist)) {
		@maxlist = @cluster;
		#printf("Growing %s\n", join(',',@maxlist));
	}
	while (@all) {
		my $t = pop(@all);
		my @rem = ();
		foreach (@all) {
			if (defined($unidir{$_}->{$t})) {
				push(@rem, $_);
			}
		}
		push(@cluster,$t);
		grow(@rem);
		pop(@cluster);
	}
}

my @rem = @comps;
while (scalar(@rem) > scalar(@maxlist)) {
	my $first = shift @rem;
	@cluster = ($first);
	my @tail = reverse sort keys %{$unidir{$first}};
	grow(@tail);
}
$part2 = join(',',@maxlist);

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
