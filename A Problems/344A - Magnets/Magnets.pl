#!/usr/bin/perl

$n = <> + 0;
$groups = 1;
$previous = <>;

for ($i = 0; $i < ($n-1); $i++) {
    $magnet = <>;
    if ($magnet ne $previous) {
        $groups++;
    }

    $previous = $magnet;
}

print "$groups\n";