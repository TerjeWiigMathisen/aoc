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


my %seen = ();
# try to grow a clique starting with each set3
sub grow
{
	my (@comps) = @_;
	#printf("Growing %s\n", join(',',@comps));
	my %comps = ();
	foreach (@comps) { $comps{$_}++; }
	my @max = @comps;
	foreach (@comps) {
		my $t = $_;
		foreach (keys %{$link{$t}}) {
			my $n = $_;
			next if (defined($comps{$_})); # Already in the set
			my $skip = 0;
			foreach (@comps) {
				unless (defined($link{$_}->{$n})) {
					$skip = 1;
					last;
				}
			}
			next if ($skip);
			my @try = sort (@comps,$n);
			my $try = join(',',@try);
			next if (defined ($seen{$try}));
			$seen{$try}++;
			
			my @m = grow(@try);
			if (scalar(@m) > scalar(@max)) {
				@max = @m;
			}
		}
	}
	return @max;
}

my %setn = ();
%set3 = ();
foreach (@comps) {
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

my @set3 = sort keys %set3;
my @max = split(/,/,$set3[0]);
foreach (@set3) {
	my @set = split(/,/,$_);
	my @m = grow(@set);
	if (scalar(@m) > scalar(@max)) {
		@max = @m;
	}
}

$part2 = join(',',sort @max);

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
