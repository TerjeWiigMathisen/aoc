#!perl -w
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

my %links = ();
my %linkto = ();
while (<>) {
	chomp;
	push(@lines, $_);
	my @c = sort split(/\-/);
	my $f = join("-", @c);
	$links{$f}++;
	$linkto{$c[0]} .= $c[1];
}
printf("Found %d computers\n",scalar (keys %linkto));

#part1

my @tcomp = ();

foreach (keys %linkto) {
	if (substr($_,0,1) eq 't') {
		push(@tcomp, $_);
	}
}
printf("Found %d computers starting with 't'\n",scalar (@tcomp));

@tcomp = sort @tcomp;

my %set3 = ();
foreach (@tcomp) {
	my $t = $_;
	my @nbor = split(//,$linkto{$t});
	for (my $j = 1; $j < scalar(@nbor); $j++) {
		my $b = $nbor[$j];
		
		for (my $i = 0; $i < $j; $i++) {
			my $a = $nbor[$i];
			if (defined($links{"$a-$b"})) {
				$set3{join(',',sort ($t,$a,$b))}++;
			}
		}
	}
}

$part1 = scalar keys %set3;
printf("%d\n", scalar keys %set3);

# part2
my @comps = sort {length($linkto{$b}) <=> length($linkto{$b})} keys %linkto;

my %seen = ();

my @max = ();
# try to grow a clique starting from a given computer
sub grow
{
	my ($next, @curr) = @_;
	#printf("Growing %s\n", join(',',@comps));
	for (my $n = $next; $n < scalar(@comps); $n++) {
		my $t = $comps[$n];
		my $ok = 1;
		foreach (@curr) {
			unless (defined($links{"$_-$t"})) {
				$ok = 0;
				last;
			}
		}
		if ($ok) {
			my @m = grow($next+1,@curr,$t);
			if (scalar(@m) > scalar(@max)) { @max = @m; }
		}
	}
	return @max;
}

my %setn = ();
%set3 = ();
foreach (@comps) {
	my $t = $_;
	my @nbor = keys %{$links{$t}};
	for (my $j = 1; $j < scalar(@nbor); $j++) {
		my $b = $nbor[$j];
		
		for (my $i = 0; $i < $j; $i++) {
			my $a = $nbor[$i];
			if (defined($linkto{$a}->{$b})) {
				$set3{join(',',sort ($t,$a,$b))}++;
			}
		}
	}
}

my @set3 = sort keys %set3;
@max = split(/,/,$set3[0]);
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
