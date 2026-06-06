#!perl -w
use strict;
use warnings;
no warnings 'recursion';
use Time::HiRes qw(time);
use List::PriorityQueue;

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @inp = (<>); chomp(@inp);
#printf("Input:\n%s\n\n", join("\n",@inp));

my ($min,$max) = (7,27);
if (scalar(@inp) > 10) { #real data!
	($min,$max) = (200000000000000,400000000000000);
}

my @dx = (1,0,-1,0);
my @dy = (0,1,0,-1);

my $EPS = 10000;

sub intersectxy
{
	my ($x1,$y1,$z1,$vx1,$vy1,$vz1,$x0,$y0,$z0,$vx0,$vy0,$vz0) = @_;
	if ($vx1*$vy0 == $vx0*$vy1) {
#		printf("%d and %d moving parallel\n",$j/6,$i/6);
		return(undef,());
	}
	my $dx = $x1-$x0;
	my $dy = $y1-$y0;
	my $t1 = ($dy*$vx0-$dx*$vy0)/($vx1*$vy0-$vx0*$vy1);
	my $t0 = $vx0 ? ($dx+$t1*$vx1)/$vx0 : 1e38;
	my ($mx,$my,$mz) = ($x1+$vx1*$t1, $y1+$vy1*$t1, $z1+$vz1*$t1);	
	
	my ($mx0,$my0,$mz0) = ($x0+$vx0*$t0, $y0+$vy0*$t0, $z0+$vz0*$t0); # Must be (nearly) identical to (mx1,my1)
#	die("Too large offsets ($mx0,$my0) vs ($mx,$my)\n") if (abs($mx-$mx0) > $EPS || abs($my-$my0) > $EPS);
	my $dz = $mz0-($z1+$vz1*$t1);
	return (abs($dz),$mx,$my,$mz);
}

sub p1
{
	my (@inp) = @_;
	my @hail = ();
	my $hails = 0;
	my @max = ();
	my @min = ();
	my @dmax = ();
	my @dmin = ();
	foreach (@inp) {
		my @p = split(/\s*[,\@]\s/);
		die unless (scalar(@p) == 6);
		for (my $i = 0;$i<3;$i++) {
			$max[$i] = $p[$i] unless (defined($max[$i]) && $max[$i] >= $p[$i]);
			$min[$i] = $p[$i] unless (defined($min[$i]) && $min[$i] <= $p[$i]);
			$dmax[$i] = $p[$i+3] unless (defined($dmax[$i]) && $dmax[$i] >= $p[$i+3]);
			$dmin[$i] = $p[$i+3] unless (defined($dmin[$i]) && $dmin[$i] <= $p[$i+3]);
		}
		push(@hail,@p);
		$hails++;
	}
	printf("Loaded $hails hail stones\n");
	printf("min: (%d,%d,%d)\nmax:(%d,%d,%d)\ndmin: (%d,%d,%d) dmax:(%d,%d,%d)\n",@min,@max,@dmin,@dmax);
#	die;
	my $hlim = $hails*6;
	my $par = 0;
	my $ok = 0;
	for (my $i = 6; $i < $hlim; $i+=6) {
		# Line i starts at $hails[0],$hails[1] and moves $hails[3],$hails[4] per ns
		my ($x1,$y1,$z1,$vx1,$vy1,$vz1) = @hail[$i..$i+5];
		for (my $j=0; $j < $i; $j += 6) {
			my ($x0,$y0,$z0,$vx0,$vy0,$vz0) = @hail[$j..$j+5];
			# Line intersection where?
#			printf("(%d,%d,%d,%d) vs (%d,%d,%d,%d)\n",$x0,$y0,$vx0,$vy0, $x1,$y1,$vx1,$vy1);
			if ($vx1*$vy0 == $vx0*$vy1) {
#				printf("%d and %d moving parallel\n",$j/6,$i/6);
				$par++;
				next;
			}
			my $dx = $x1-$x0;
			my $dy = $y1-$y0;
#			my $t1 = ($dy*$vx0 - $dx)/($vx1-$vy1*$vx0);
#			$t1 = ($dx*$vy0-$dy*$vx0)/($vy1-$vx1*$vy0);
			my $t1 = ($dy*$vx0-$dx*$vy0)/($vx1*$vy0-$vx0*$vy1);
			my $t0 = ($dx+$t1*$vx1)/$vx0;
			my ($mx,$my) = ($x1+$vx1*$t1,$y1+$vy1*$t1);	
			
			my ($mx0,$my0) = ($x0+$vx0*$t0,$y0+$vy0*$t0);	
#			die("$mx,$my != $mx0,$my0 @ $t0,$t1") if ($mx != $mx0 || $my != $my0);
			if ($t1 < 0 || $t0 < 0) {
#				printf("%d and %d: t0 = $t0, t1=$t1, mx = $mx, my=$my, minus time\n",$j/6,$i/6);
			}
			else { # Meet at coords
				if ($mx >= $min && $mx <= $max && $my >= $min && $my <= $max) {
					my $dz = ($z0+$vz0*$t1)-($z1+$vz1*$t1);
#					printf("%d and %d: t0=$t0, t1=$t1, mx = $mx, my=$my, dz=$dz\n",$j/6,$i/6) 
#						if (abs($t1-$t0)<1000000000);
					$ok++;
				}
				else {
#					printf("%d and %d: mx = $mx, my=$my, outside the test area\n",$j/6,$i/6);
				}
			}
		}
	}
	return $ok;
}

sub dist3
{
	my (@c) = @_;
	my $d2 = 0;
	for (my $i = 0; $i<3; $i++) {
		my $d = $c[$i+3]-$c[$i];
		$d2 += $d*$d;
	}
	return sqrt($d2);
}

sub p2
{
	my (@inp) = @_;
	my @hail = ();
	my $hails = 0;
	my @max = ();
	my @min = ();
	my @dmax = (-1e38,-1e38,-1e38);
	my @dmin = (1e38,1e38,1e38);
	my @sum = (0,0,0);
	my @sum395 = (0,0,0);
	foreach (@inp) {
		my @p = split(/\s*[,\@]\s/);
		die unless (scalar(@p) == 6);
		for (my $i = 0;$i<3;$i++) {
			$max[$i] = $p[$i] unless (defined($max[$i]) && $max[$i] >= $p[$i]);
			$min[$i] = $p[$i] unless (defined($min[$i]) && $min[$i] <= $p[$i]);
			$dmax[$i] = $p[$i+3] unless (defined($dmax[$i]) && $dmax[$i] >= $p[$i+3]);
			$dmin[$i] = $p[$i+3] unless (defined($dmin[$i]) && $dmin[$i] <= $p[$i+3]);
			$sum[$i]+=$p[$i];
			$sum395[$i] += $p[$i]+$p[$i+3]*3.95e11;
		}
		push(@hail,@p);
		$hails++;
	}
	printf("Loaded $hails hail stones\n");
	printf("min: (%d,%d,%d)\nmax:(%d,%d,%d)\ndmin: (%d,%d,%d) dmax:(%d,%d,%d)\n",@min,@max,@dmin,@dmax);
	
	my @centroid = ();
	my @cent395 = ();
	foreach (@sum) {
		push(@centroid,$_/$hails);
	}
	foreach (@sum395) {
		push(@cent395,$_/$hails);
	}
	printf("Centroid: (%e,%e,%e), cent 3.95e11: (%e,%e,%e)\n", @centroid, @cent395);
	my $d2max = 0;
	my $outmax;
	my @p;
	my @out;
	my @centdist =();
	my @diverge = ();
	my @offset3 = ();
	my @offset31 = ();
	my %slow = ();
	for (my $i = 0; $i < $hails; $i++) {
		@p = (@hail)[$i*6..$i*6+5];
		my $d2 = dist3(@centroid, @p[0..2]);
		push(@offset3,$d2);
		push(@offset31,dist3($p[0]+$p[3],$p[1]+$p[4],$p[2]+$p[5],@centroid));
		$slow{$i} = dist3(0,0,0,@p[3..5]);
		my $d0 = dist3((@p)[0..2],@centroid);
		my $d1 = dist3($p[0]+$p[3],$p[1]+$p[4],$p[2]+$p[5],@centroid);
#		my $d0 = dist3($p[0],$centroid[1],$centroid[2],@centroid);
#		my $d1 = dist3($p[0]+$p[3],$centroid[1],$centroid[2],@centroid);
		push(@diverge,sprintf("%10.3f %d", $d1-$d0, $i));
		push(@centdist,sprintf("%20.3f %d", $d2, $i));
		
	}
	foreach (0,3.2e11,3.5e11,3.7e11,3.8e11,3.9e11,3.95e11,4e11,4.1e11,4.2e11,4.5e11) {
		my $ns = $_;
		my $rms = 0;
		for (my $i = 0; $i < $hails; $i++) {
			@p = (@hail)[$i*6..$i*6+5];
			my $d2 = dist3($p[0]+$p[3]*$ns,$p[1]+$p[4]*$ns,$p[2]+$p[5]*$ns,@centroid);
			$rms += $d2*$d2;
		}
		$rms = sqrt($rms/$hails);
#		printf("RMS(%10.8e) = %10.8e\n",$ns,$rms);
	}

	@centdist = reverse sort(@centdist);
#	printf("Outlier hails\n");
	foreach(@centdist[0..19]) {
		my $i = substr($_,20);
#		printf("%s (%s)\n",$_,join(',',@hail[$i*6..$i*6+5]));
	}
	@diverge = reverse sort(@diverge);
#	printf("Top diverging hails\n");
	foreach(@diverge[0..19]) {
		my $i = substr($_,10);
#		printf("%s (%s) %f\n",$_,join(',',@hail[$i*6..$i*6+5]), $offset3[$i]);
	}
	my $distmax = 0;
	my @other;
	for (my $o = 0; $o < 10; $o++) {
		my $ind = substr($diverge[$o],10);
		@out = (@hail)[$ind*6..$ind*6+5];
		for (my $i = 0; $i < $hails; $i++) {
			my @p = (@hail)[$i*6..$i*6+5];
			my $d2 = dist3(@out[0..2],@p[0..2]);
			if ($d2 > $distmax) {
#				printf("New max dist: $ind at (%s)\n$i at (%s) dist3=%f\n",
#					join(",",@p),join(",",@out),$d2);
				$distmax = $d2;
				@other = (@hail)[$i*6..$i*6+5];
			}
		}
	}
	# Slowest hail:
	my @slow = sort {$slow{$a} <=> $slow{$b}} (keys %slow);
#	printf("Slowest hail:\n");
	foreach (@slow[0..9]) {
		my $i = $slow{$_};
#		printf("%3d: %6.2f (%s)\n",$_,$i,join(",",@hail[$_*6..$_*6+5]));
	}
	
	# How much has the centroid moved during 3.95e11 ns?
	
	my $time_to_centroid = 3.95e11;
	for (my $far = 0; $far < 10; $far++) {
		my $start = substr($centdist[$far],20);
		my @start = @hail[$start*6..$start*6+5];
		my @dir = (($cent395[0]-$start[0])/$time_to_centroid,
				($cent395[1]-$start[1])/$time_to_centroid,
				($cent395[2]-$start[2])/$time_to_centroid);
		my $miss = 0;
		for (my $h = 0; $h<$hails*6; $h+=6) {
			my @p = @hail[$h..$h+5];
#			my $closest = closest(@start,@p);
#			my $miss += $closest*$closest;
		}
		$miss = sqrt($miss)/$hails;
#		printf("%d (%3d): %10.8e\n",$far,$start,$miss);
	}
	# Centroid speed:
#	printf("Centroid speed = %f,%f,%f\n",($cent395[0]-$centroid[0])/$time_to_centroid,
#	  ($cent395[1]-$centroid[1])/$time_to_centroid,($cent395[2]-$centroid[2])/$time_to_centroid);
	# Initial direction to @other
	my @dir;
	for (my $i = 0; $i < 3; $i++) {
		push(@dir,$other[$i]-$out[$i]);
	}

	# Solution search starts here:
	my $d2best = 1e38;
	for (my $vx = -1000; $vx <= 1000; $vx++) {
		for (my $vy = -400; $vy <= 1000; $vy++) {
			my @h1 = @hail[0..5];
			my @h2 = @hail[6..11];
			my @h3 = @hail[12..17];
			$h1[3] += $vx;
			$h2[3] += $vx;
			$h3[3] += $vx;
			$h1[4] += $vy;
			$h2[4] += $vy;
			$h3[4] += $vy;
			my ($dz,$mx,$my) = intersectxy(@h1,@h2);
			if (!defined($my)) {
				($dz,$mx,$my) = intersectxy(@h1,@h2); # Debug error?
				next;
			}
			my ($dz3,$mx3,$my3) = intersectxy(@h1,@h3);
			if (!defined($my3)) {
				($dz3,$mx3,$my3) = intersectxy(@h1,@h3); # Debug error?
				next;
			}
			my $d2 = abs($mx3-$mx)+abs($my3-$my);
			if ($d2 < $d2best) {
#				printf("vx=$vx,vy=$vy -> d2=%10.5e\n",$d2);
				$d2best = $d2;
				if ($d2 < 1) {
					printf("vx=$vx,vy=$vy -> d2=%10.5e\n",$d2);
					$h1[5]-=1000; $h2[5]-=1000;$h3[5]-=1000;
					my $diffbest=$dz;
					for (my $vz = -1000; $vz <= 1000; $vz++) {
						my ($dz5,$mx5,$my5,$mz5) = intersectxy(@h1,@h2);
						if (!defined($my5)) {
							($dz5,$mx5,$my5) = intersectxy(@h1,@h2);
							next;
						}
#						printf("vz=%d, dz5=$dz5\n",$vz);
						if ($dz5 <= $diffbest) {
#							printf("vx=$vx,vy=$vy,vz=$vz -> dz5=%10.5e\n",$dz5);
							$diffbest = $dz5;
							if ($dz5 < 1) {
								printf("Start at ($mx5,$my5,$mz5) with speed = (%d,%d,%d)\n",-$vx,-$vy,-$vz);
								return $mx5+$my5+$mz5;
							}
						}
						$h1[5]++; $h2[5]++;$h3[5]++;
					}
				}
			}
		}
	}
			
	return $hails;
}


$part1 = p1(@inp);
$part2 = p2(@inp);

my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.3fs\n", $used);

