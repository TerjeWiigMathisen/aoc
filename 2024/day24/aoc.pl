#!perl -w
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

my $one = '';

while (<>) {
	$one .= $_;
	chomp;
	push(@lines, $_);
}

my ($inputs, $gates) = split(/\n\n/, $one);
my @inputs = split(/\n/,$inputs);
my @gates = split(/\n/,$gates);

my %value = ();
foreach (@inputs) {
	if (/^(\S+)\: (\d+)$/) {
		$value{$1} = $2;
	}
	else { die($_); }
}

#part1

my @regates = ();
my @ogates = @gates;
my %ren = ();
while (scalar(@ogates)) {
	die("Bad input $_") unless (/^(\S+) (\S+) (\S+) \-\> (\S+)$/);
	my ($ra, $op, $rb, $res) = ($1, $2, $3, $4);
	($ra,$rb) = sort ($ra,$rb);
	if ($ra =~ /^x(\d\d)$/) {
		my $n = $1;
		die("Unmatched names: $_") unless ($rb =~ /^y(\d\d)$/);
		die("Unmatched levels: $_") unless ($n == $1);
		if ($op eq 'AND') {
			$ren{$res} = 'A'.$n;
		}
		elsif ($op eq 'XOR') {
			$ren{$res} = 'X'.$n;
		}
		else { die("Unknown operation: $_"); 
		}
	}
			

my %swap = ();

sub simulate
{
	my (@ugates) = @_;
	my %val = %value;
	#my @ugates = @gates;
	my $part1 = 0;

	my %prop = ();
	my $iteration = 1;
	while (scalar(@ugates)) {
		my @gat = ();
		foreach (@ugates) {
			if (/^(\S+) (\S+) (\S+) \-\> (\S+)$/) {
				my ($ra, $op, $rb, $res) = ($1, $2, $3, $4);
				if (defined($swap{$ra})) { $ra = $swap{$ra}; }
				if (defined($swap{$rb})) { $rb = $swap{$rb}; }
				if (defined($swap{$res})) { $res = $swap{$res}; }
				($ra,$rb) = sort ($ra,$rb);
				my ($a, $b) = ($val{$ra}, $val{$rb});
				if (defined($a) && defined($b)) {
					my $rename;
					if ($ra =~ /x(\d\d)/) {
						$rename = $1;
					}
					my $r;
					if ($op eq 'AND') {
						$r = $a & $b;
						if ($ra =~ /x(\d\d)/) {
							$swap{$res} = 'a'.$rename;
						}
						elsif ($ra =~ /c(\d\d)/) {
							$swap{$res} = 't'.$1;
						}
					}
					elsif ($op eq 'OR') {
						$r = $a | $b;
						if ($ra =~/a(\d\d)/) {
							$swap{$res} = 'c'.$1;
						}
					}
					elsif ($op eq 'XOR') {
						$r = ($a == $b)? 0 : 1;
						if (defined($rename)) {
							$swap{$res} = 'r'.$rename;
						}
					}
					else {die("Bad op ".$op); }
					die("bad result from $_ = $r") unless ($r eq '0' or $r eq '1');
					if (defined($swap{$res})) {
						#$res = $swap{$res};
					}
					if ($r) {
						$prop{$res}++;
					}
					$val{$res} = $r;
					printf("$ra $op $rb -> $res\n");
					#printf("%3d:  %s\n",$iteration,$_);
					last;
				}
				else {
					push(@gat,$_);
				}
			}
			else { die("Bad inst ".$_); }
		}
		@ugates = @gat;
		$iteration++;
	}

	foreach (keys %val) {
		if (/^z(\d+)$/) {
			#printf("%s $val{$_}\n", $_);
			$part1 |= $val{$_} << $1;
		}
	}
	foreach (split(/,/,'x12,x13,y12,y13,nqs,qts,jmr,fsf,whc,wfh,gdr,z12,z13')) {
		#printf("%s:%d ",$_,$val{$_});
	}
	#printf("\n");
	return ($part1, join(" ",sort keys %prop));
}

sub simulate_adder
{
	my ($bit, $cin, @ugates) = @_;
	my %val = ();
	$val{sprintf("x%02d",$bit)} = 1;
	$val{sprintf("x%02d",$bit+1)} = 1;
	$val{sprintf("x%02d",$bit+2)} = 1;
	$val{sprintf("x%02d",$bit+3)} = 0;
	
	$val{sprintf("y%02d",$bit)} = 1;
	$val{sprintf("y%02d",$bit+1)} = 0;
	$val{sprintf("y%02d",$bit+2)} = 0;
	$val{sprintf("y%02d",$bit+3)} = 0;
	foreach (split(/,/,$cin)) {
		$val{$_} = 0;
	}
	my %seen = ();
	foreach (keys %val) {
		$seen{$_} = 0;
	}
	
	#my @ugates = @gates;
	my $part1 = 0;

	my %prop = ();
	for (my $iter = 1; $iter <= 15; $iter++) {
		foreach (@ugates) {
			if (/^(\S+) (\S+) (\S+) \-\> (\S+)$/) {
				my ($a, $op, $b, $res) = ($1, $2, $3, $4);
				($a, $b) = ($val{$a}, $val{$b});
				if (defined($a) && defined($b)) {
					my $r;
					if ($op eq 'AND') {
						$r = $a & $b;
					}
					elsif ($op eq 'OR') {
						$r = $a | $b;
					}
					elsif ($op eq 'XOR') {
						$r = ($a == $b)? 0 : 1;
					}
					else {die("Bad op ".$op); }
					die("bad result from $_ = $r") unless ($r eq '0' or $r eq '1');
					if (defined($swap{$res})) {
						$res = $swap{$res};
					}
					if ($r) {
						$prop{$res}++;
					}
					$val{$res} = $r;
					unless (defined($seen{$res})) {
						$seen{$res} = $iter;
					}
				}
			}
			else { die("Bad inst ".$_); }
		}
	}

	foreach (sort keys %val) {
		$seen{$_} += 0;
		printf("%s $val{$_} %d\n", $_, $seen{$_});
	}
}

swap('fgc','z12');

$part1 = simulate(@gates);
die;

my %ok = ();
sub simulate2
{
	my ($ix,$iy) = @_;
	my ($x,$y) = ($ix,$iy);
	for (my $b = 0; $b < 45; $b++) {
		$value{sprintf('x%02d',$b)} = $x & 1; $x >>= 1;
		$value{sprintf('y%02d',$b)} = $y & 1; $y >>= 1;
	}
	
	($x,$y) = ($ix,$iy);
	my ($res, $prop) = simulate(@gates);
	if ($res != $x + $y) {
		printf("Error: %x + %x -> %x (should be %x)\n", $x, $y, $res, $x+$y);
		#printf("OK gates = %s\n",join(" ",sort keys %ok));
		foreach (split(/ /, $prop)) {
			if (!defined($ok{$_})) {
				printf("$_ ");
			}
		}
		printf("\n");
		#die;
	}
	else {
		foreach (split(/ /,$prop)) {
			$ok{$_}++;
		}
			
		#printf("OK: $prop\n");
	}
}

sub swap
{
	my ($a, $b) = @_;
	$swap{$a} = $b;
	$swap{$b} = $a;
}
swap('mtj','z29');
swap('dgr','vvm');
swap('dtv','z37');

printf("%s\n",join(',', sort ('fgc','z12','mtj','z29','dgr','vvm','dtv','z37')));


for (my $bit = 32; $bit < 44; $bit++) {
	my $v = 1 << $bit;
	my $u = $v*3;
	printf("$bit %x + %x\n", $v,$u);
	simulate2($u,$u);
	simulate2($v,$u);
}

#simulate_adder(32,'dtp',@gates);

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
