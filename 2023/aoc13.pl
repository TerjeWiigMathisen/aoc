#!perl -w
use strict;
use Time::HiRes qw(time);

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @inp = (<>); chomp(@inp);
my $inp = join("\n",@inp);

my @inputs = split(/\n\n/, $inp);
#printf("Input:\n%s\n\n", join("\n",@inp));

sub vert_mirror
{
	my ($inp, $skip) = @_;
	my @inp = split(/\n/,$inp);
	
	for (my $v = 1; $v < scalar(@inp); $v++) {
		if ($inp[$v] eq $inp[$v-1]) {
			my $up = $v-2;
			my $dn = $v+1;
			my $mirror = 1;
			while ($up >= 0 && $dn < scalar(@inp)) {
				if ($inp[$up] ne $inp[$dn]) {
					$mirror = 0;
					last;
				}
				$up--;
				$dn++;
			}
			return $v*100 if ($mirror && $v*100 != $skip);
		}
	}
	return 0;
}

sub hori_mirror
{
	my ($inp, $skip) = @_;
	my @inp = split(/\n/,$inp);
	
	for (my $h = 1; $h < length($inp[0]); $h++) {
		my $mirror = 1;
		for (my $y = 0; $y < scalar(@inp); $y++) {
			my $l = $inp[$y];
			my $lt = substr($l,0,$h);
			my $rt = substr($l,$h);
			if (length($lt) < length($rt)) {
				$rt = substr($rt,0,length($lt));
			}
			elsif (length($lt) > length($rt)) {
				$lt = substr($lt,-length($rt));
			}
			if ($lt ne join("",reverse (split(//,$rt)))) {
				$mirror = 0;
				last;
			}
		}
		return $h if ($mirror && $h != $skip);
	}
	return 0;
}

my $line = 0;
foreach (@inputs) {
	my $t = vert_mirror($_,0);
	if (!$t) {
		$t = hori_mirror($_,0);
	}
	$part1 += $t;

	my $inp = $_;
	my $found = 0;
	for (my $flip = 0; $flip < length($inp); $flip++) {
		my $i = $inp;
		my $c = substr($i,$flip,1);
		next if ($c eq "\n");

		substr($i,$flip,1) = $c eq '#' ? '.' : '#';
		my $t2 = vert_mirror($i,$t);
		if (!$t2) {
			$t2 += hori_mirror($i,$t);
		}
		if ($t2 && $t != $t2) {
#			printf("%d\n%s\n\n%s\n\n",$t2,$inp,$i);
			$part2 += $t2;
			$found = 1;
			last;
		}
	}
	die("No reflection found for \n$inp!") unless ($found);
}

my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.3fs\n", $used);
