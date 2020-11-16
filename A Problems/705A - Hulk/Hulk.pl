#!/usr/bin/perl
use strict;
use warnings;

my $n = <> + 0;
my @feelings = ();

for (my $i = 0; $i < $n; $i++) {
    if($i % 2 eq 0) {
        push(@feelings, "I hate");
    } else {
        push(@feelings, "I love");
    }
}

my $result = join(" that ", @feelings);
print $result . " it\n";