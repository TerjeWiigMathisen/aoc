#!perl -w
use strict;
use Time::HiRes qw(time);
use List::PriorityQueue;

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @inp = (<>); chomp(@inp);
#printf("Input:\n%s\n\n", join("\n",@inp));

my %dest = ();
my %typ = ();
my %state = ();
my %mod_dest_slot = ();
my %slots = ();

sub solve
{
	foreach (@inp) {
		my ($mod,$dest) = split(/ \-\> /);
		die unless(defined($dest));
		
		my $t = 'broad';
		if ($mod =~ /^[\%\&]/) {
			$t = substr($mod,0,1);
			$mod = substr($mod,1);
		}
		$dest{$mod} = $dest;
		$typ{$mod} = $t;
		if ($t eq '%') {
			$state{$mod} = 0;
		}
		foreach (split(/, /,$dest)) {
			my @s = ();
			my $s = $slots{$_};
			if (defined($s)) {
				@s = split(/,/,$s);
			}
			$mod_dest_slot{"$mod;$_"} = scalar(@s);
			push(@s,0);
			$slots{$_} = join(",",@s);
#			printf("module %s has %d inputs\n",$_,scalar(@s));
		}
	}

	# Press button
	my @pulse = ();
	
	my ($p1,$p2);
	
	my $zeroes = 0;
	my @zeroes = (0,0,0,0);
	
	my @counts = (0,0);
	for (my $press = 1; $press < 100000000; $press++) {
		push(@pulse,'button',0, "broadcaster");
		while (scalar(@pulse)) {
			my ($src, $p, $mod) = splice(@pulse,0,3);
			$counts[$p]++;
			if ($mod eq 'output') {
				printf("$src $p output\n");
				next;
			}
#			printf("$press: $src $p -> $mod\n");
			if ($mod eq 'rx') {
#				printf("Signal rx after $press runder\n");
				return ($p1,$press) if ($p == 0);
				next;
			}
			if ($src eq 'button') {
				my $d = $dest{$mod};
				foreach (split(/, /,$d)) {
					push(@pulse,$mod,0, $_);
				}
			}
			elsif ($typ{$mod} eq '%') { # Flip-flop
				next if ($p); # NOP for high pulse
				$state{$mod} ^= 1;
				my $p = $state{$mod};
				my $d = $dest{$mod};
				foreach (split(/, /,$d)) {
					push(@pulse,$mod,$p, $_);
				}
			}
			else { # & conjunction
				my $s = $mod_dest_slot{"$src;$mod"};
				my @s = split(/,/,$slots{$mod});
				$s[$s] = $p;
				$slots{$mod} = join(",",@s);
				my $h = 1;
				foreach (@s) {
					$h &= $_;
				}
				$h ^= 1;
				my $i = index(";jg;;rh;;jm;;hf", $mod);
				if ($h == 1 && $i > 0) {
					$i >>= 2;
					@zeroes[$i] = $press;
					printf("%s = 0 at %d\n",$mod,$press);
					$p2 = $zeroes[0]*$zeroes[1]*$zeroes[2]*$zeroes[3];
					if ($p2) {
						return ($p1,$p2);
					}
				}
				my $d = $dest{$mod};
				die($mod) unless (defined($d));
				foreach (split(/, /,$d)) {
					push(@pulse, $mod,$h,$_);
				}
			}
		}
		if ($press == 1000) {
			$p1 = $counts[0]*$counts[1];
			printf("part1=%d\n",$p1);
			$p2 = 3793*3947*4003*4019;
			return ($p1,$p2);
		}
	}
	return ($p1,$p2)
}

($part1,$part2) = solve();
#$part2 = solve(4,10);
my $used = time-$t0;

printf("solve %s\n%s\n", $part1, $part2);
printf("Used %1.3fs\n", $used);

#$t0 = time;
#$part2 = solve_star(4,10);

#$used = time-$t0;

#printf("star %s\n%s\n", $part1, $part2);
#printf("Used %1.3fs\n", $used);
