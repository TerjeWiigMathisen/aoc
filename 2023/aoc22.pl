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

my @supports = ('');
my @supported_by = ('');
sub do_not_support
{
	my (@inp) = @_;
	# Sort by z values
	@inp = sort {(split(/[,~]/,$a))[2] <=> (split(/[,~]/,$b))[2]} @inp;
	my %at = ();
	my $brick = 0;
	my ($xmax,$ymax) = (0,0);
	foreach (@inp) {
		$brick++;
		push(@supports,'');
		my ($f,$t) = split(/~/);
		my ($x0,$y0,$z0) = split(/,/,$f);
		my ($x1,$y1,$z1) = split(/,/,$t);
		die("$_") if ($x1 < $x0 || $y1 < $y0 || $z1 < $z0);
		$xmax = $x1 if ($x1 > $xmax);
		$ymax = $y1 if ($y1 > $ymax);
		
		#Try to let it fall:
		my %sup_by = ();
		while ($z0 > 1) {
			my $z = $z0-1;
			for (my $y = $y0; $y <= $y1; $y++) {
				for (my $x = $x0; $x <= $x1; $x++) {
					my $k = join(",",$x,$y,$z);
					if (defined($at{$k})) {
						$sup_by{$at{$k}}++;
					}
				}
			}
			last if (scalar(keys %sup_by));
			$z0--; $z1--;
		}
		# Final place for this brick
		my @sup_by = keys %sup_by;
		my $sup_by = join(',',@sup_by);
#		printf("Brick $brick is supported at z=$z0 by (%s)\n",$sup_by);
		for (my $z = $z0; $z <= $z1; $z++) {
			for (my $y = $y0; $y <= $y1; $y++) {
				for (my $x = $x0; $x <= $x1; $x++) {
					my $k = join(",",$x,$y,$z);
					$at{$k} = $brick;
				}
			}
		}
		
		if ($z0 == 0) {
			$sup_by = "0";
			@sup_by = ();
		}
		$supported_by[$brick] = $sup_by;
		foreach (@sup_by) {
			my @s = split(/,/,$supports[$_]);
			$supports[$_] = join(",",@s,$brick);
		}
	}
	# All fallen as deep as possible
	my @only_support = ();
	my $ok = 0;
	for (my $b = 1; $b <= $brick; $b++) {
		if ($supports[$b] eq '') {
#			printf("OK: Brick %d does not support any\n",$b);
			$ok++;
			$only_support[$b] = '';
			next;
		}
		my @s = split(/,/,$supports[$b]);
		my $multi = 1;
		my @only = ();
		foreach (@s) {
			my @b = split(/,/,$supported_by[$_]);
			if (scalar(@b)==1) {
#				printf("   Brick %d is the only support for %d\n",$b,$_);
				$multi = 0;
				push(@only,$_);
			}
		}
		unless ($multi) {
			$only_support[$b] = join(',',@only);
			next;
		}
#		printf("OK: All supported by %d has multiple supports\n",$b);
		$ok++;
	}
#	printf("part1 = %d\n",$ok);
	printf("max = ($xmax,$ymax)\n");
	return $ok;
}

sub nosupport
{
	my ($b,$dis) = @_;
	$dis->{$b}++;

	my $fall = 0;
	foreach(split(/,/,$supports[$b])) {
		my $o = $_;
		my $s = 0;
		foreach(split(/,/,$supported_by[$o])) {
			if (!defined($dis->{$_})) {
#				printf("Brick $b is still supported by $_\n");
				$s++;
			}
		}
		next if ($s);
#		printf("Brick $o will fall\n");
		$fall++;
		$fall += nosupport($o,$dis);
	}
	return $fall;
}

sub p2
{
	my $brick = scalar(@supports)-1;
	my $p2 = 0;
	for (my $b = 1; $b <= $brick; $b++) {
		my %dis = ();
		my $fall = nosupport($b,\%dis);
		if ($fall) {
#			printf("Removing $b causes $fall to fall\n");
			$p2 += $fall;
		}
		foreach (keys %dis) {
			if ($dis{$_} > 1) {
				printf("Brick $_ fell %d times\n",is{$_});
			}
		}
	}
	return $p2;
}

$part1 = do_not_support(@inp);
$part2 = p2; # 13841

my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.3fs\n", $used);

#$t0 = time;
#$part2 = solve_star(4,10);

#$used = time-$t0;

#printf("star %s\n%s\n", $part1, $part2);
#printf("Used %1.3fs\n", $used);
