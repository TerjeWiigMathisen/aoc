#!perl -w
use strict;
use Time::HiRes qw(time);

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @inp = (<>); chomp(@inp);

my %seen = ();
my $hits = 0;

sub try2
{
	my ($pre, $wild, @counts) = @_;
	
	# Normalize the input to increase cache hit rates!
	$wild =~ s/^\.+//g;	# No leading dots
	$wild =~ s/\.+$//g;	# No trailing dots
	$wild =~ s/\.+/\./g;# Squash multip0le dots into one
	$wild .= ".";		# Add a guard at the end since this simplifies the core logic
	
	my $key = join(";",$wild,@counts);
	if (defined($seen{$key})) { # Have we seen this pattern before?
		$hits++;
		return $seen{$key};
	}

	my $hash = () = $wild =~ /#/g;
	if (scalar(@counts) == 0) {
		my $t = $hash == 0; # 0 or 1
		$seen{$key} = $t;
		return $t;
	}
	my $total_count = 0;
	foreach (@counts) {
		$total_count += $_;
	}
	$total_count -= $hash; # Total count remaining minus '#'s still not covered

	my $quest = () = $wild =~ /\?/g;
	if ($total_count < 0 || $quest < $total_count) { # Too many '#' remain in the pattern!
		$seen{$key} = 0;
		return 0;
	}

	my $cnt = shift @counts;
	my $w = substr($wild,0,$cnt+1);

	# Is this a possible match point, starting with a '#' and ending with '.' or '?'?
	if ($w =~ /^#[#\?]*[\.\?]$/) {
		my $t = try2($pre.'a',substr($wild,$cnt+1),@counts);
		$seen{$key} = $t;
		return $t;
	}
	if ($w =~ /^\?[#\?]*[\.\?]$/) { # Ditto, but starting with a '?'
		my $t = try2($pre.'b',substr($wild,$cnt+1),@counts);
		# Try again after skipping the first '?':
		my $t2 = try2($pre.'c',substr($wild,1),$cnt,@counts);
		$t += $t2;
		$seen{$key} = $t;
		return $t;
	}
	if (substr($w,0,1) eq '?') { # Try to skip leading '?'
		# Try next starting point, restoring the first count
		my $t = try2($pre.'d',substr($wild,1),$cnt,@counts);
		$seen{$key} = $t;
		return $t;
	}
	$seen{$key} = 0;
	return 0;
}

sub try3
{
	my ($wild, @counts) = @_;
	
	# Normalize the input to increase cache hit rates!
	$wild =~ s/^\.+//g;	# No leading dots
	$wild =~ s/\.+$//g;	# No trailing dots
	$wild =~ s/\.+/\./g;# Squash multip0le dots into one
	$wild .= ".";		# Add a guard at the end since this simplifies the core logic
	
	my $key = join(";",$wild,@counts);
	if (defined($seen{$key})) { # Have we seen this pattern before?
		$hits++;
		return $seen{$key};
	}

	my $hash = () = $wild =~ /#/g;
	if (scalar(@counts) == 0) {
		my $t = $hash == 0; # 0 or 1
		$seen{$key} = $t;
		return $t;
	}
	my $total_count = 0;
	foreach (@counts) {
		$total_count += $_;
	}
	$total_count -= $hash; # Total count remaining minus '#'s still not covered

	my $quest = () = $wild =~ /\?/g;
	if ($total_count < 0 || $quest < $total_count) { # Too many '#' remain in the pattern!
		$seen{$key} = 0;
		return 0;
	}

	my $cnt = shift @counts;
	my $w = substr($wild,0,$cnt+1);

	# Is this a possible match point, starting with a '#' and ending with '.' or '?'?
	if ($w =~ /^#[#\?]*[\.\?]$/) {
		my $t = try3(substr($wild,$cnt+1),@counts);
		$seen{$key} = $t;
		return $t;
	}
	my $t = 0;
	if ($w =~ /^\?[#\?]*[\.\?]$/) { # Ditto, but starting with a '?'
		$t = try3(substr($wild,$cnt+1),@counts);

		# Try again after skipping the first '?':
		my $t2 = try3(substr($wild,1),$cnt,@counts);
		$t += $t2;
		$seen{$key} = $t;
		return $t;
	}
	if (substr($w,0,1) eq '?') { # Try to skip leading '?'
		# Try next starting point, restoring the first count
		my $t = try3(substr($wild,1),$cnt,@counts);
		$seen{$key} = $t;
		return $t;
	}
	$seen{$key} = 0;
	return 0;
}

sub try4
{
	my ($wild, @counts) = @_;
	
	# Normalize the input to increase cache hit rates!
	$wild =~ s/^\.+//g;	# No leading dots
	$wild =~ s/\.+$//g;	# No trailing dots
	$wild =~ s/\.+/\./g;# Squash multip0le dots into one
	$wild .= ".";		# Add a guard at the end since this simplifies the core logic
	
	my $key = join(";",$wild,@counts);
	if (defined($seen{$key})) { # Have we seen this pattern before?
		$hits++;
		return $seen{$key};
	}

	my $hash = () = $wild =~ /#/g;
	if (scalar(@counts) == 0) {
		my $t = $hash == 0; # 0 or 1
		$seen{$key} = $t;
		return $t;
	}
	my $total_count = 0;
	foreach (@counts) {
		$total_count += $_;
	}
	$total_count -= $hash; # Total count remaining minus '#'s still not covered

	my $quest = () = $wild =~ /\?/g;
	if ($total_count < 0 || $quest < $total_count) { # Too many '#' remain in the pattern!
		$seen{$key} = 0;
		return 0;
	}

	# Is this a possible match point?
	my ($curr, $next) = (0, 0);
	if (substr($wild,0,1) eq '?') { # Try to skip leading '?'
		$next = try4(substr($wild,1),@counts);
	}
	my $cnt = shift @counts;
	my $w = substr($wild,0,$cnt+1);

	if ($w =~ /^[#\?]*[\.\?]$/) { # Only '#' and '?', followed by '.' or '?'
		$curr = try4(substr($wild,$cnt+1),@counts);
	}
	$curr += $next;
	$seen{$key} = $curr;
	return $curr;
}

my $line = 0;
foreach (@inp) {
	my $l = $_;
	$line++;
	my ($wild,$counts) = split(/\s+/);
	my @counts = split(/,/,$counts);
	my $p1 = try4($wild,@counts);
	$part1 += $p1;

	my @c = (@counts,@counts,@counts,@counts,@counts);
	my $w = join("?",$wild,$wild,$wild,$wild,$wild);
	%seen = ();
	my $p2 = try4($w, @counts, @counts, @counts, @counts, @counts);
	$part2 += $p2;
}

my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.3fs\n", $used);
printf("%d cache hits\n", $hits);
