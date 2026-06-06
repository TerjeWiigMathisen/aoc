sub A {
    my ($k, $x1, $x2, $x3, $x4, $x5) = @_;
    my($B);
    $B = sub { A(--$k, $B, $x1, $x2, $x3, $x4) };
    $k <= 0 ? &$x4 + &$x5 : &$B;
}

for (my $k = 1; $k < 30; $k++) {
	print $k, A($k, sub{1}, sub {-1}, sub{-1}, sub{1}, sub{0} ), "\n";
}