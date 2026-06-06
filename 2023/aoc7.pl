#!perl -w
use strict;
use Time::HiRes qw(time);

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @hands = (<>); chomp(@hands);

sub hand_type
{
	my ($hand) = @_;
	my @cards = sort((split(//,$hand))[0..4]);
	my %cards = ();
	foreach (@cards) {
		$cards{$_}++;
	}
	my @cnt = sort {$cards{$b} <=> $cards{$a}} (keys %cards);
	if ($cards{$cnt[0]} == 5) { return 6000; }
	if ($cards{$cnt[0]} == 4) { return 5000; }
	if (($cards{$cnt[0]} == 3) && ($cards{$cnt[1]} == 2)) { return 4000; }
	if (($cards{$cnt[0]} == 3) && ($cards{$cnt[1]} == 1)) { return 3000; }
	if (($cards{$cnt[0]} == 2) && ($cards{$cnt[1]} == 2)) { return 2000; }
	if (($cards{$cnt[0]} == 2) && ($cards{$cnt[1]} == 1)) { return 1000; }
	return 0;
}

sub compare_hands
{
	my ($x, $y) = @_;
	my ($xt, $yt) = (hand_type($x,0),hand_type($y,0));
	return ($xt-$yt) if ($xt != $yt);
	$x =~ tr/AKQJT/edcba/;
	$y =~ tr/AKQJT/edcba/;
	return ($x cmp $y);
}

foreach(@hands) {
#	printf("%4d %s\n",hand_type($_), $_);
}

my @sorted_hands = sort {compare_hands($a,$b)} @hands;
for (my $r = 1; $r <= scalar(@sorted_hands); $r++) {
	my $h = $sorted_hands[$r-1];
	my $bid = substr($h,6);
	my $win = $bid*$r;
	$part1 += $win;
#	printf("%d %5d -> %6d %s\n", $r, $bid, $win, $h);
}

sub hand_type_joker
{
	my ($hand) = @_;
	my @cards = sort((split(//,$hand))[0..4]);
	my %cards = ();
	foreach (@cards) {
		$cards{$_}++;
	}
	my $jokers = $cards{'J'}; $cards{'J'} = 0; $jokers = 0 unless (defined($jokers));
	
	my @cnt = sort {$cards{$b} <=> $cards{$a}} (keys %cards);
	
	if ($cards{$cnt[0]}+$jokers == 5) { return 6000; }
	if ($cards{$cnt[0]}+$jokers == 4) { return 5000; }
	if (($cards{$cnt[0]}+$jokers == 3) && ($cards{$cnt[1]} == 2)) { return 4000; }
	if (($cards{$cnt[0]}+$jokers == 3) && ($cards{$cnt[1]} == 1)) { return 3000; }
	if (($cards{$cnt[0]}+$jokers == 2) && ($cards{$cnt[1]} == 2)) { return 2000; }
	if (($cards{$cnt[0]}+$jokers == 2) && ($cards{$cnt[1]} == 1)) { return 1000; }
	return 0;
}

sub compare_hands_joker
{
	my ($x, $y) = @_;
	my ($xt, $yt) = (hand_type_joker($x,0),hand_type_joker($y,0));
	return ($xt-$yt) if ($xt != $yt);

	$x =~ tr/AKQJT/edc1a/;
	$y =~ tr/AKQJT/edc1a/;
	return ($x cmp $y);
}

foreach(@hands) {
#	printf("%4d %s\n",hand_type_joker($_), $_);
}

@sorted_hands = sort {compare_hands_joker($a,$b)} @hands;
#print;
foreach(@sorted_hands) {
#	printf("%4d %s\n",hand_type_joker($_), $_);
}

for (my $r = 1; $r <= scalar(@sorted_hands); $r++) {
	my $h = $sorted_hands[$r-1];
	my $bid = substr($h,6);
	my $win = $bid*$r;
	$part2 += $win;
#	printf("%d %5d -> %6d %s\n", $r, $bid, $win, $h);
}


my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.6fus\n", $used);

$t0 = time;
$part1 = 0;
$part2 = 0;

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

$used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.6fus\n", $used);
