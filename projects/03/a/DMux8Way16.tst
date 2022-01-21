load DMux8Way16.hdl,
output-file DMux8Way16.out,
compare-to DMux8Way16.cmp,
output-list in%D2.2.2 sel%B2.3.2 a%D2.2.2 b%D2.2.2 c%D2.2.2 d%D2.2.2 e%D2.2.2 f%D2.2.2 g%D2.2.2 h%D2.2.2;

set in 0,
set sel %B000,
eval,
output;

set sel %B001,
eval,
output;

set sel %B010,
eval,
output;

set sel %B011,
eval,
output;

set sel %B100,
eval,
output;

set sel %B101,
eval,
output;

set sel %B110,
eval,
output;

set sel %B111,
eval,
output;

set in 12,
set sel %B000,
eval,
output;

set in 34,
set sel %B001,
eval,
output;

set in 56,
set sel %B010,
eval,
output;

set in 78,
set sel %B011,
eval,
output;

set in 90,
set sel %B100,
eval,
output;

set in 98,
set sel %B101,
eval,
output;

set in 76,
set sel %B110,
eval,
output;

set in 54,
set sel %B111,
eval,
output;
