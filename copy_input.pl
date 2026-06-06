#!perl -w 

use strict;

my $year = shift;

opendir(D, $year);
my @files = readdir(D);
close(D);

my $tdir = "c:\\github\\advent-of-code-rust\\input\\year$year";
mkdir($tdir) unless (-d $tdir);

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
foreach (@files) {
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
	my $target = sprintf("c:\\github\\advent-of-code-rust\\input\\year%d\\day%02d.txt", $year, $day);
	next if ($seen{$target}++ > 0);
	open(T, '>', $target);
	syswrite(T, $buffer);
	close(T);
	printf(STDERR "Created $target\n");
	$cnt++;
}
printf("$cnt input files copied\n");
