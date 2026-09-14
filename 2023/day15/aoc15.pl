#!perl -w
use strict;
use Time::HiRes qw(time);

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @inp = (<>); chomp(@inp);
#printf("Input:\n%s\n\n", join("\n",@inp));

sub hash
{
	my ($s) = @_;
	my $h = 0;
	foreach (split(//,$s)) {
		my $c = ord($_);
		$h = (($h+$c)*17) & 255;
	}
	return $h;
}

sub hash_sum
{
	my ($init) = @_;
	my $sum = 0;
	foreach (split(/,/,$init)) {
		$sum += hash($_);
	}
	return $sum;
}

sub hashmap
{
	my ($init) = @_;
	my @box;
	foreach (split(/,/,$init)) {
		my ($op, $focal) = split(/[\-\=]/);
		my $b = hash($op);
		$box[$b] = '' unless (defined($box[$b]));
		if ($focal eq '') { # remove lens if found
			my @list = split(/\t/,$box[$b]);
			my @new = ();
			foreach (@list) {
				my ($o,$f) = split(/ /);
				push(@new, $_) unless ($o eq $op);
			}
			$box[$b] = join("\t",@new);
		}
		else { # add or replace?
			my @list = split(/\t/,$box[$b]);
			my @new = ();
			my $replaced = 0;
			foreach (@list) {
				my ($o,$f) = split(/ /);
				if ($o eq $op) { # Replace lens
					push(@new,$op.' '.$focal);
					$replaced = 1;
				}
				else {
					push(@new, $_);
				}
			}
			push(@new,$op.' '.$focal) unless ($replaced);
			$box[$b] = join("\t",@new);
		}
		
	}
#	for (my $b = 0; $b < 256; $b++) {
#		if (defined($box[$b]) && $box[$b] ne '') {
#			printf("Box %d: %s\n", $b, $box[$b]);
#		}
#	}
#	printf("\n");

	my $focusing_power = 0;
	for (my $b = 1; $b <= 256; $b++) {
		my $box = $box[$b-1];
		if (defined($box) && $box ne '') {
			my @lenses = split(/\t/,$box);
			for (my $slot = 1; $slot <= scalar(@lenses); $slot++) {
				my ($label,$f) = split(/ /,$lenses[$slot-1]);
				my $focus = $b*$slot*$f;
				$focusing_power += $focus;
			}
		}
	}
	return $focusing_power;
}

$part1 = hash_sum($inp[0]);
$part2 = hashmap($inp[0]);
	
my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.3fs\n", $used);
