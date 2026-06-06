#!perl -w

use strict;

my @inp = (<>);
chomp(@inp);

my @numbers = split(/,/,shift(@inp)); 

my $board = 0;
my @b = ();
#my @boards = ();
my %b = ();
my @hor = ();
my %numat = ();
my %hits = ();
my %board = ();

my $WINS = 0;
my $BOARDS = 0;
my %found = ();

foreach (@inp) {
	if ($_ eq '') {
		save (@b);
		@b = ();
		next;
	}
	push(@b, $_);
}
save(@b);

my ($ROWS, $COLS);

sub save
{
	my (@b) = @_;
	return unless(scalar(@b));
	
	my $board = $BOARDS++;
	$ROWS = scalar(@b);
	for (my $y = 0; $y < scalar(@b); $y++) {
		my @n = split(" ", $b[$y]);
		$COLS = scalar(@n);
		for (my $x = 0; $x < scalar(@n); $x++) {
			my $num = $n[$x];
			push(@{$numat{$num}},"$x,$y,$board");
			$board{"$x,$y,$board"} = $num;
			$hits{",$y,$board"} = scalar(@n);
			$hits{"$x,,$board"} = scalar(@b);
		}
	}
	# push(@boards, \@b);
	dumpboards($board);
}

sub dumpboards
{
	my (@b) = @_;
	foreach (@b) {
		printf(STDERR "Board %d:\n", $_);
		my $key = $_;
		for (my $y = 0; $y < $ROWS; $y++) {
			for (my $x = 0; $x < $COLS; $x++) {
				printf(STDERR "%3d",$board{"$x,$y,$key"});
			}
			printf(STDERR "\n");
		}
		printf(STDERR "\n");
	}
}

# Process @numbers
foreach (@numbers) {
	my $num = $_;
	if (defined($numat{$_})) {
		my @b = @{$numat{$_}};
		foreach (@b) {
			my $key = $_;
			my ($x,$y,$board) = split(/,/, $key);
			next if (defined($found{$board}));
			printf(STDERR "Removing $num from $key\n");
			#dumpboards($board);
			die("Wrong number $num") unless ($board{$key} == $num);
			$board{$key} = 0;
			unless (--$hits{",$y,$board"}) {
				won($board, $num);
			}
			unless (--$hits{"$x,,$board"}) {
				won($board,$num);
			}
			#dumpboards($board);
		}
	}
}

sub won
{
	my ($board, $lastnum) = @_;
	if (!defined($found{$board})) {
		dumpboards($board);
		my $sum = 0;
		for (my $y = 0; $y < $ROWS; $y++) {
			my @n = split(" ", $b[$y]);
			$COLS = scalar(@n);
			for (my $x = 0; $x < $COLS; $x++) {
				$sum += $board{"$x,$y,$board"};
			}
		}
		printf("board = %d, lastnum = %d, sum = %d, prod = %1.0f\n", $board, $lastnum, $sum, $sum * $lastnum);
		exit if (++$WINS == $BOARDS);
		$found{$board}++;
	}
}

	

