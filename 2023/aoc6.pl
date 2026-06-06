#!perl -w
use strict;
use Time::HiRes qw(time);

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

$_ = <>; chomp; my @times = split();
$_ = <>; chomp; my @dists = split();

sub winning
{
	my ($tid, $distance_to_beat) = @_;
	my $wins = 0;
	my $minwin = int($distance_to_beat/$tid);
	while ($minwin * ($tid-$minwin) <= $distance_to_beat) {
		$minwin++;
	}
	# Max distance_to_beat time, use bin search?
	my $maxwin = $tid-1;
	while ($maxwin * ($tid-$maxwin) <= $distance_to_beat) {
		$maxwin--;
	}
	$wins = $maxwin-$minwin+1;
	printf("t=%d, d=%d, wins=%d (%d-%d)\n", $tid, $distance_to_beat, $wins, $minwin, $maxwin);
	return $wins;
}

sub winning2
{
	my ($tid, $distance_to_beat) = @_;
	my $wins = 0;
	# dist = button_press * (tid-button_press) > distance_to_beat
	# -b^2 + b*t - distance_to_beat = 0
	# b^2 - b*t + distance_to_beat = 0
	# (b - t/2)^2 + distance_to_beat - (t/2)^2
	# b = t/2 +- sqrt((t/2)^2-distance_to_beat)
	my $t2 = $tid*0.5;
	my $root = sqrt($t2*$t2-$distance_to_beat);
#	printf("Roots = %1.6f,%1.6f\n", $t2-$root, $t2+$root);

	my $minwin = int($t2-$root);
	while ($minwin * ($tid-$minwin) <= $distance_to_beat) {
		$minwin++;
	}
	# Max distance_to_beat time
	my $maxwin = int($t2+$root+0.999);
	while ($maxwin * ($tid-$maxwin) <= $distance_to_beat) {
		$maxwin--;
	}
	$wins = $maxwin-$minwin+1;
#	printf("t=%d, d=%d, wins=%d (%d-%d)\n", $tid, $distance_to_beat, $wins, $minwin, $maxwin);
	return $wins;
}

$part1 = 1;
for (my $game = 1; $game < scalar(@times); $game++) {
	$part1 *= winning2($times[$game],$dists[$game]);
}

shift @times; shift @dists;
$part2 = winning2(join('',@times), join('',@dists));

my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.6fus\n", $used);