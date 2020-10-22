#!/usr/bin/perl
use strict;
use warnings;

my $n = <> + 0;
my $isEasy = 1;
my $line = <>;

my @numbers = split ' ', $line;

foreach my $number (@numbers) {
    if($number == 1) {
        $isEasy = 0;
    }
}

if($isEasy) {
    print "EASY\n";
} else {
    print "HARD\n";
}