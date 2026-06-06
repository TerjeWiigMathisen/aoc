#!perl -w
use strict;
use Time::HiRes qw(time);

my $part1 = 0;
my $part2 = 0;

my %seeds = ();

my @inp = (<>);
chomp(@inp);
push(@inp,"map:");

my $line = 0;

my $t0 = time;
# part1
while (1) { # Gather the seeds
	$_ = $inp[$line++];
	chomp;
	if (/seeds:\s+(\S.*)$/) {
		foreach (split(/\s+/, $1)) {
			$seeds{$_} = 1;
		}
	}
	elsif (/map:/) {
		last;
	}
}
printf("%s\n", join(", ", sort {$a<=> $b} keys %seeds));

sub mapit
{
	my ($txt, $inp, $dst) = @_;
	
	# seed-to-soil map:
	%{$dst} =();
	my %seen = ();
	while (1) {
		nxt:
#		last if ($line >= scalar(@inp));
		$_ = $inp[$line++];
		chomp;
		if (/\d[\d\s]*$/) {
			my ($dest, $src,$len) = split(/\s+/);
			foreach (keys %{$inp}) {
				next if defined($seen{$_});
				if ($_ >= $src && $_ < $src+$len) {
					my $d = $_ - $src + $dest;
					$dst->{$d} = 1;
					$seen{$_} = 1;
#					printf("%s->%s ",$_,$d);
#					goto nxt;
				}
			}
		}
		elsif (/map:/) {
			foreach (keys %{$inp}) {
				if (!defined($seen{$_})) {
					$dst->{$_} = 1;
					$seen{$_} = 1;
				}
			}
			last;
		}
	}
#	printf("$txt: %s\n", join(", ", sort {$a<=> $b} keys %{$dst}));
}

# part2
$line = 0;
my @seeds = ();
while (1) { # Gather the seed ranges
	$_ = $inp[$line++];
	chomp;
	if (/seeds:\s+(\S.*)$/) {
		@seeds = split(/\s+/,$1);
	}
	elsif (/map:/) {
		last;
	}
}
#printf("%s\n", join(", ", @seeds));

sub mapitrange
{
	my ($txt, @ranges) = @_;
	
	# pairs of numbers define ranges:
	my @out =();
	my @save = ();
	while (1) {
		$_ = $inp[$line++];
		chomp;
		if (/\d[\d\s]*$/) {
			push(@ranges, @save); @save = ();
			my ($dest, $src,$len) = split(/\s+/);
			while (@ranges) {
				my ($s,$l) = splice(@ranges,0,2);
				# Does this range overlap with the current map?
				if ($s < $src && $s+$l > $src) {
					# Save the range before the start of the map:
					my ($s0, $l0, $s1, $l1) = ($s, $src-$s, $src, $s+$l-$src);
					push(@save, $s0, $l0);
#					printf("Cut before: %d->%d => %d->%d and %d->%d\n", $s,$l, $s0, $l0, $s1, $l1);
					$s = $s1;
					$l = $l1;
				}
				if ($s < $src+$len && $s+$l > $src+$len) {
					my ($s0, $l0, $s1, $l1) = ($s, $src+$len-$s, $src+$len, $s+$l-$src-$len);
					push(@save, $s1, $l1);
#					printf("Cut after: %d->%d => %d->%d and %d->%d\n", $s,$l, $s0,$l0, $s1, $l1);
					$s = $s0;
					$l = $l0;
				}
				if ($s >= $src && $s+$l <= $src+$len) {
					push(@out, $s+$dest-$src, $l);
#					printf("Remapped: %d->%d => %d->%d\n", $s, $l, $s+$dest-$src, $l);
				}
				else { # outside the range
					push(@save, $s,$l);
				}
			}
		}
		elsif (/map:/) {
			push(@out, @save);
			last;
		}
	}
	my %sort = ();
	for (my $i = 0; $i < scalar(@out); $i += 2) {
		$sort{$out[$i]} = $out[$i+1];
	}
	@out = ();
	my @skeys = sort {$a <=> $b} keys %sort;
	my @p = ();
	foreach (@skeys) {
		push(@out, $_, $sort{$_});
		push(@p, sprintf("%d->%d",$_, $sort{$_}));
	}
#	printf("$txt: %s\n",join(", ",@p));
	return @out;
}

my @soils = mapitrange("soils", @seeds);

my @fert = mapitrange("fert",@soils);

my @water = mapitrange("water",@fert);

my @light = mapitrange("light",@water);

my @temp = mapitrange("temp",@light);

my @humi = mapitrange("humi",@temp);

my @loc = mapitrange("loc", @humi);

$part2 = $loc[0];
my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.6fus\n", $used);