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

#part1
my %src = ();
foreach (@lines) {
	my ($src, @dest) = split;
	chop($src);
	$src{$src} = \@dest;
}

my %cache = ();
sub paths
{
	my ($p) = @_;
	if (defined($cache{$p})) { return $cache{$p}; }
	my $paths = 0;
	foreach (@{$src{$p}}) {
		if ($_ eq 'out') {$paths++;}
		elsif (defined($src{$_})) {
			$paths += paths($_);
		}
	}
	$cache{$p} = $paths;
	return $paths;
}

$part1 = paths('you');

#my %cache = ();
sub p2
{
	my ($p,$dst,$not) = @_;
	my $key = join(",",@_);
	if (defined($cache{$key})) { return $cache{$key}; }
	my $paths = 0;
	foreach (@{$src{$p}}) {
		if ($_ eq $dst) {$paths++;}
		elsif ($_ eq $not) {} #skip
		elsif (defined($src{$_})) {
			$paths += paths2($_,$dst,$not);
		}
	}
	$cache{$key} = $paths;
	return $paths;
}

sub paths2
{
#	%cache = ();
	my $p = p2(@_);
	#printf("$p - %s\n",join(",",@_));
	return $p;
}

my $youdac = paths2('svr','dac','fft');
my $youfft = paths2('svr','fft','dac');
my $dacfft = paths2('dac','fft','');
my $fftdac = paths2('fft','dac','');
my $dacout = paths2('dac','out','fft');
my $fftout = paths2('fft','out','dac');

$part2 = $youdac * $dacfft * $fftout + $youfft * $fftdac * $dacout;
my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
printf("Too high: 4581426212159811816\n");