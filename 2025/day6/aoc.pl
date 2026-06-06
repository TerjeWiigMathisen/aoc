#!perl -w
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

while (<>) {
	chomp;
	push(@lines, $_);
}

my %mat = ();
my $COLS;
my $y = 0;
foreach (@lines) {
	my @cols = split();
	#printf("%d %d %s\n",$y, scalar(@cols), join(",",@cols));
	for (my $x = 0; $x < scalar(@cols); $x++) {
		$mat{"$x;$y"} = $cols[$x];
	}
	$COLS = scalar(@cols);
	$y++;
}

my $ROWS = scalar(@lines);
my $LR = $ROWS-1;
my @OPS = ();
for (my $x = 0; $x < $COLS; $x++) {
	my $op = $mat{"$x;$LR"};
	if ($op eq '+') {
		my $sum = 0;
		for (my $y = 0; $y < $LR; $y++) {
			$sum += $mat{"$x;$y"};
		}
#		printf("$x: + $sum\n");
		$part1 += $sum;
	}
	elsif ($op eq '*') {
		my $prod = 1;
		for (my $y = 0; $y < $LR; $y++) {
			$prod *= $mat{"$x;$y"};
		}
#		printf("$x: * $prod\n");
		$part1 += $prod;
	}
	else {
		die("Bad op: $op");
	}
	push(@OPS,$op);
}	

#part2
#printf("1058 + 3253600 + 625 + 8544 = 3263827\n");
my $opline = $lines[$LR];
for (my $col = 0; $col < length($opline); $col++) {
	my $op = substr($opline,$col,1);
	if ($op ne ' ') {
		my @nums = ();
		for (my $x = $col; 1; $x++) {
			my $v = '';
			for (my $y = 0; $y < $LR; $y++) {
				my $c = substr($lines[$y],$x,1);
				$v .= $c;
			}
			last unless ($v =~ /\d/);
			push(@nums,$v);
#			printf("%s\n",$v);
		}
#		printf("%s\n",join($op,@nums));
		if ($op eq '+') {
			my $sum = 0;
			for (my $y = 0; $y < $LR; $y++) {
				$sum += $nums[$y] if ($nums[$y]);
			}
#			printf("+ $sum\n");
			$part2 += $sum;
		}
		elsif ($op eq '*') {
			my $prod = 1;
			for (my $y = 0; $y < $LR; $y++) {
				$prod *= $nums[$y] if ($nums[$y]);
			}
#			printf("* $prod\n");
			$part2 += $prod;
		}
		else {
			die("Bad op: $op");
		}
	}
}

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
