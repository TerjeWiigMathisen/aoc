#!perl -w
use strict;
use Time::HiRes qw(time);
#use List::PriorityQueue;
use Math::Polygon::Calc;

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @inp = (<>); my $inp = join("",@inp); my ($rules,$parts) = split(/\n\n/,$inp);
my @rules = split(/\n/,$rules); chomp(@inp);
my @parts = split(/\n/,$parts); chomp(@parts);

#printf("Input:\n%s\n\n", join("\n",@inp));

sub p1
{
	my ($r,$p) = @_;
	my %wf = ();
	foreach (@{$r}) {
		die("Bad rule list: ".$_) unless (/^(\S+)\{(\S+)\}/);
		my ($n, $r) = ($1,$2);
		$wf{$n} = $r;
	}
	my $sum = 0;
	foreach (@{$p}) {
		my $w = 'in';
		die("Bad part: ".$_) unless (/^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$/);
		my @xmas = ($1,$2,$3,$4);
		while ($w ne 'A' && $w ne 'R') {
			my @r = split(/,/,$wf{$w});
			foreach (@r) {
				my ($r,$d) = split(/:/);
				unless (defined($d)) {
					$w = $_;
					last;
				}
				die("Bad test: ".$_) unless ($r =~ /^([xmas])([\<\>])(\d+)$/);
				my ($i,$c,$lim) = (index("xmas",$1),$2,$3);
				if ($c eq '<' && $xmas[$i] < $lim) {
					$w = $d;
					last;
				}
				if ($c eq '>' && $xmas[$i] > $lim) {
					$w = $d;
					last;
				}
			}
		}
		if ($w eq 'A') {
			foreach (@xmas) {
				$sum += $_;
			}
		}
	}
	return $sum;
}

my @volumes = ();

sub reach
{
	my ($path, $rule, $wf, @limits) = @_;
	printf("  %s -> %s",$path,$rule);
	my $w = $wf->{$rule};
	if (!defined($w)) {
		printf("\n");
		if ($rule eq 'A') {
#			printf("$path: %s\n",join(",",@limits));
			printf("%s\n",join(",",@limits));
			push(@volumes,\@limits);
		}
		return;
	}
	printf(" %s\n",$w);
	
	my @r = split(/,/,$w);
	foreach (@r) {
		my ($r,$d) = split(/:/);
		if (!defined($d)) {
			reach($path,$r,$wf,@limits);
			next;
		}
		die("Bad test: ".$_) unless ($r =~ /^([xmas])([\<\>])(\d+)$/);
		my ($i,$c,$lim) = (index("xmas",$1),$2,$3);
		my $p1;
		my $p2;
		my @l = @limits;
		if ($c eq '<') {
			if ($l[$i*2+1] > $lim) {
				$l[$i*2+1] = $lim;
			}
			reach("$path;$r",$d,$wf,@l);
			if ($limits[$i*2] < $lim) {
				$limits[$i*2] = $lim;
			}
			$path.= ";".substr($r,0,1).">=".substr($r,2);
		}
		else {
			if ($l[$i*2] <= $lim) {
				$l[$i*2] = $lim+1;
			}
			reach("$path;$r",$d,$wf,@l);
			if ($limits[$i*2+1] > $lim+1) {
				$limits[$i*2+1] = $lim+1;
			}
			$path.= ";".substr($r,0,1)."<=".substr($r,2);
		}
	}
}

sub min
{
	my ($a, $b) = @_;
	return $a < $b? $a : $b;
}

sub max
{
	my ($a, $b) = @_;
	return $a > $b? $a : $b;
}

sub volume
{
	my @o = @_;
	return ($o[1]-$o[0])*($o[3]-$o[2])*($o[5]-$o[4])*($o[7]-$o[6]);
}
	
sub overlap
{
	my (@c) = @_;
	my @o = ();
	for (my $i = 0; $i < 8; $i += 2) {
		push(@o, max($c[$i],$c[$i+8]), min($c[$i+1],$c[$i+9]));
	}
	for (my $i = 0; $i < 8; $i += 2) {
		return 0 if ($o[$i] >= $o[$i+1]);
	}
	return volume(@o);
}

sub intersect
{
	my (@c) = @_;
	my @o = ();
	for (my $i = 0; $i < 8; $i += 2) {
		push(@o, max($c[$i],$c[$i+8]), min($c[$i+1],$c[$i+9]));
	}
	for (my $i = 0; $i < 8; $i += 2) {
		return () if ($o[$i] >= $o[$i+1]);
	}
	return @o;
}
sub subdivide
{
	my ($x,$m,$a,$s,$v) = @_;
	my @x = @{$x};
	my @m = @{$m};
	my @a = @{$a};
	my @s = @{$s};
	my @v = @{$v};
	my $xs = scalar(@x);
	my $ms = scalar(@m);
	my $as = scalar(@a);
	my $ss = scalar(@s);
	my $sum = 0;
	if ($xs > 2 && $ms>2 && $as>2 && $ss>2) {
		$xs>>=1; $ms>>=1; $as>>=1; $ss>>=1;
		my @xp = ($x[0],$x[$xs],$x[-1]);
		for (my $i = 1; $i <= 2; $i++) {
			my @sub = ();
			foreach(@v) {
				my @inter = intersect($xp[$i-1],$xp[$i],,,@{$_});
				if (@inter) {
					push(@sub,\@inter);
				}
			}
			$sum += subdivide($x[$i-1..$i]);
		}
	}
}

sub p2
{
	my ($r,$p) = @_;
	my %wf = ();

	foreach (@{$r}) {
		die("Bad rule list: ".$_) unless (/^(\S+)\{(\S+)\}/);
		my ($n, $r) = ($1,$2);
		$wf{$n} = $r;
	}
	my $opt=1;
	my %opt = ();
	while ($opt) {
		$opt = 0;
		foreach (keys %wf) {
			my $key = $_;
			next if (defined($opt{$_}));
			my %dest = ();
			my @r = split(/,/,$wf{$key});
			foreach (@r) {
				my ($r,$d) = split(/:/);
				if (defined($d)) {
					$_ = $d;
				}
				while (defined($opt{$_})) {
					$_ = $opt{$_};
				}
				$dest{$_}++;
			}
			if (scalar(keys %dest) == 1) {
				my $d = (keys %dest)[0];
#				printf("%s(%s) -> %s\n",$key,$wf{$key},$d);
				$wf{$key} = $d;
				$opt{$key} = $d;
				$opt++;
			}
		}
	}
	printf("opt list\n");
	foreach (sort keys %wf) {
		printf("%s{%s}\n",$_,$wf{$_});
	}
#	my %reach = ();
	reach('','in',\%wf, 1,4001,1,4001,1,4001,1,4001);
	my @vol = ();
	my $sum = 0;
	my %x = (1=>1,4001=>1);
	my %m = (1=>1,4001=>1);
	my %a = (1=>1,4001=>1);
	my %s = (1=>1,4001=>1);
	
	foreach (@volumes) {
		my @v = @{$_};
		$x{$v[0]}++;
		$x{$v[1]}++;
		$m{$v[2]}++;
		$m{$v[3]}++;
		$a{$v[4]}++;
		$a{$v[5]}++;
		$s{$v[6]}++;
		$s{$v[7]}++;
	}
	my @x = sort {$a<=>$b} (keys %x); printf("%s\n",join(",",@x));
	my @m = sort {$a<=>$b} (keys %m); printf("%s\n",join(",",@m));
	my @a = sort {$a<=>$b} (keys %a); printf("%s\n",join(",",@a));
	my @s = sort {$a<=>$b} (keys %s); printf("%s\n",join(",",@s));
	
	@volumes = sort {volume(@{$b}) <=> volume(@{$a})} @volumes;
	
#	$sum = subdivide(\@x,\@m,\@a,\@s,\@volumes);
	
#	my $px0 = pop @x;
#	my $pm0 = pop @m;
#	my $pa0 = pop @a;
#	my $ps0 = pop @s;

	#my %seen = ();
	my @seen = ();
	my $bsize = (scalar(@x)*scalar(@m)*scalar(@a)*scalar(@s)+31)>>5;
	printf("Bitmap size = %d\n", $bsize);
	for (my $b = 0; $b < $bsize; $b++) {
		push(@seen,0);
	}
	my $vv = 0;
	foreach (@volumes) {
		my @v = @{$_};
		printf("%d: %s\n",$sum,join(",",@v));
		for (my $x = 1; $x < scalar(@x); $x++) {
			my $xx = $x[$x-1];
			next if ($xx < $v[0]);
			last if ($xx >= $v[1]);
			my $volx = $x[$x]-$xx;
			for (my $m = 1; $m < scalar(@m); $m++) {
				my $mm = $m[$m-1];
				next if ($mm < $v[2]);
				last if ($mm >= $v[3]);
				my $volm = $volx * ($m[$m]-$mm);
				for (my $a = 1; $a < scalar(@a); $a++) {
					my $aa = $a[$a-1];
					next if ($aa < $v[4]);
					last if ($aa >= $v[5]);
					my $vola = $volm * ($a[$a]-$aa);
					for (my $s = 1; $s < scalar(@s); $s++) {
						my $ss = $s[$s-1];
						next if ($ss < $v[6]);
						last if ($ss >= $v[7]);
						#my $key = join("\t",$xx,$mm,$aa,$ss);
						my $kk = (($x*scalar(@m)+$m)*scalar(@a)+$a)*scalar(@s)+$s;
						my $kw = $kk >> 5; 
						my $kb = 1 << ($kk & 31);
						if (($seen[$kw] & $kb) == 0) {
							my $vols = $vola*($s[$s]-$ss);
							$sum += $vols;
							$seen[$kw] |= $kb;
#							$seen{$key} = $vols;
#							printf("%s, %d, %d\n",$key, $vols, $sum);
						}
					}
				}
			}
		}
		$vv++;
		printf("%d of %d: (%s) %d\n",$vv,scalar(@volumes), join(",",@v), $sum);
	}
	return $sum;
}

$part1 = p1(\@rules,\@parts);
$part2 = p2(\@rules,\@parts);
my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.6fs\n", $used);
