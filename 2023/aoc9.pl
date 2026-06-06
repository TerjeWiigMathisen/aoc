#!perl -w
use strict;
use Time::HiRes qw(time);

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @inp = (<>); chomp(@inp);

sub predict
{
	my ($seq) = @_;
	my @lines = ($seq);
	my @last = ();
	my @first = ();
	my @seq = split(/\s+/,$seq);
#	printf("%s\n", $seq);
	my $pred = 0;
	my $before = 0;
	my $sum;
	do {
		$sum = 0;
		my $curr = shift @seq;
		unshift(@first, $curr);
		my @diff = ();
		foreach (@seq) {
			my $d = $_-$curr;
			push(@diff,$d);
			$sum |= $d;
			$curr = $_;
		}
		push(@last, $curr);
		$pred += $curr;
		push(@lines,join(" ",@diff));
		@seq = @diff;
	} while ($sum);
#	printf("%s\n\n%s\n%s\n\n", join("\n",@lines), join(" ",@last), join(" ",reverse @first));
	
	foreach (@first) {
		$before = $_ - $before;
	}
	
	return ($pred, $before);
}

foreach(@inp) {
	my ($p1,$p2) = predict($_);
	my ($t1,$t2) = predict($p2.' '.$_);
#	printf("p2,p1,t1=%d,%d,%d\n", $p2,$p1,$t1);
	die($_) unless ($p1 == $t1);
	$part1 += $p1;
	$part2 += $p2;
}
		
my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.6fs\n", $used);
