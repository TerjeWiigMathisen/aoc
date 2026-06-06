#!perl -w 

use strict;

my $year = shift;

opendir(D, $year);
my @files = readdir(D);
close(D);

#my $tdir = "c:\\github\\aoc\\input\\year$year";
#mkdir($tdir) unless (-d $tdir);

sub datecmp{
	my ($a, $b) = @_;
	$a = '0'.$a;
	$b = '0'.$b;
	$a =~ s/\D//g;
	$b =~ s/\D//g;
	return $a <=> $b;
}

@files = sort {datecmp($a,$b)} @files;

my %seen = ();
my $cnt = 0;
foreach (sort @files) {
    next if (/^\.+$/);
	my $buffer = "";
	my $day;
	if (/input(\d+)\.txt/) {
		$day = $1;
		open(I,$year.'/'.$_);
		sysread(I, $buffer, 1e8);
		close(I);
	}
	elsif (/day(\d+)$/) {
		$day = $1;
		my $fn = $year.'/'.$_.'/input.txt';
		if (-f $fn) {
			open(I,$fn);
			sysread(I, $buffer, 1e8);
			close(I);
		}
	}
	next unless($buffer && $day > 0);
	my $dir = sprintf("c:\\github\\aoc\\%d\\day%02d", $year, $day);
	if (!(-d $dir)) {
		my $sdir = sprintf("c:\\github\\aoc\\%d\\day%d", $year, $day);
		if (-d $sdir) {
			rename($sdir,$dir);
		}
		else {
			mkdir($dir);
		}
	}

	my $target = $dir."\\input.txt";
	next if ($seen{$target}++ > 0);
	open(T, '>', $target);
	syswrite(T, $buffer);
	close(T);
	printf(STDERR "Created $target\n");
	$cnt++;
}
printf("$cnt input files copied\n");
