#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);

#use bigint;

#use JSON::Parse;
#no warnings 'recursion';

my $start = time;

my $DEBUG = 0;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

my $part1 = 0;
my $part2 = 0;

my @bot = ();
my @rule = ();
my @output = ();

my @inputvalues = ();

foreach (@inp) {
	if (/value (\d+) goes to bot (\d+)/) {
		push(@inputvalues, $_);
	}
	elsif (/bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)/) {
		if (defined($rule[$1] )) {
			printf("defined bot %d twice\n", $1);
			die;
		}
		$rule[$1] = [$2 eq "bot"? $3 : -1-$3, $4 eq "bot" ? $5 : -1-$5];
		@{$bot[$1]} = () unless (defined($bot[$1]));
	}
}

my %used = ();

sub pushval
{
	my ($lvl, $val, $bot) = @_;
	if ($bot < 0) {
		my $o = -$bot -1;
		push(@{$output[$o]}, $val);
		return;
	}
	push(@{$bot[$bot]}, $val);
	if (scalar(@{$bot[$bot]}) >= 2) {
			
		my @rule = @{$rule[$bot]};
		my ($lo, $hi) = sort {$a <=> $b} @{$bot[$bot]};
		@{$bot[$bot]} = ();
		printf("%sbot %d gives %d to %d and %d to %d\n", " " x $lvl, $bot, $lo, $rule[0], $hi, $rule[1])
			if ($lo == 17 || $lo == 61 || $hi == 17 || $hi == 61);
		
		if ($lo == 17 || $lo == 61 || $hi == 17 || $hi == 61) {
			if ($used{$bot}++) {
				printf("Bot # %d have seen 17/61 %d times\n", $bot, $used{$bot}++);
			}
		}
			
		if ($lo == 17 && $hi == 61) {
			$part1 = $b;
		}
		pushval($lvl+1,$lo, $rule[0]);
		pushval($lvl+1,$hi, $rule[1]);
	}
}
		

foreach (@inputvalues) {
	if (/value (\d+) goes to bot (\d+)/) {
		my ($val, $bot) = ($1, $2);
		printf("Input %d to bot %d\n", $val,$bot);
		pushval(0,$val, $bot);
	}
}

for (my $o = 0; $o < scalar(@output); $o++) {
	printf("output %d: %s\n", $o, join(',',@{$output[$o]}));
}

printf(STDERR "Total time = %f\n", time - $start);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
