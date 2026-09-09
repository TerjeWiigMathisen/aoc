#!perl -w
use strict;
use Time::HiRes qw(time);

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @hands = (<>); chomp(@hands);

sub hand_rank
{
	my ($hand, $wild) = @_;
	$hand = substr($hand,0,5); # only keep the 5 cards
	my %card_counts = ();
	foreach (split(//,$hand)) { $card_counts{$_}++; }
	my $jokers = $card_counts{$wild}; $card_counts{$wild} = 0; # Extract any jokers first
	$jokers = 0 unless (defined($jokers));

	# Order the rest of the hand by decreasing number of identical cards:
	my @cnt = sort {$card_counts{$b} <=> $card_counts{$a}} (keys %card_counts);
	my $top = $card_counts{$cnt[0]}+$jokers;
	my $scn = defined($cnt[1])? $card_counts{$cnt[1]} : 0;
	
	my $rank = $top.$scn;

	# Translate each card into a sortable hex value:
    if ($wild eq 'J') { $hand =~ tr/AKQJT/edc1a/; }
	else { $hand =~ tr/AKQJT/edcba/; }
	
	return $rank.$hand;
}

sub total_win
{
	my ($joker,@hands) = @_;
	my @ranked_hands = ();
	foreach (@hands) {
		push(@ranked_hands,hand_rank($_, $joker).' '.$_);
	}

	my @sorted_hands = sort @ranked_hands;
	my $sum = 0;
	while (my ($r, $t) = each (@sorted_hands)) {
		my ($rank, $h, $bid) = split(/ /,$t);
		my $win = $bid*($r+1);
		$sum += $win;
#		printf("%d %5d -> %6d %s\n", $r, $bid, $win, $t);
	}
	return $sum;
}

$part1 = total_win('',@hands);

$part2 = total_win('J',@hands);

my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.6fus\n", $used);
