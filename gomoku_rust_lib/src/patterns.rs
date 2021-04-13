// patern need to sort by order of check
pub static PATTERN: [(u8, usize, usize, i32, &str); 8] = [
    (0xF8, 6, 0, 2000, "five"),               // five XXXXX...
    (0x74, 7, 4, 80, "split four 3"),         // split four 3 .XXX.X..
    (0x6C, 7, 3, 60, "split four 2"),         // split four 2 .XX.XX..
    (0x5C, 7, 2, 80, "split four 1"),         // split four 1 .X.XXX..
    (0x78, 6, 0, 90, "open four"),            // open four .XXXX...
    (0x58, 6, 2, 30, "open split three"),     // open split three .X.XX...
    (0x68, 6, 3, 30, "open split three rev"), // open split three rev .XX.X...
    (0x70, 5, 0, 50, "open three"),           // open three  .XXX....
];

pub static CAPTURE_PATTERN: [(u8, usize, &str); 2] = [
    (0x90, 5, "capturing pair"), // capturing pair	X..X....
    (0x60, 4, "open double"),    // open double 	.XX.....
];

pub static BLOCKER: [(u8, usize); 5] = [
    (0x82, 7), // X.....X.
    (0x84, 6), // X....X..
    (0x88, 5), // X...X...
    (0x90, 4), // X..X....
    (0xA0, 3), // X.X.....
];
