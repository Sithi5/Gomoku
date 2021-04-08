use crate::bitboards::Bitboards;
use crate::bitboards::check_is_on_axe;
use crate::global_var::AXE_MOUVEMENT_VALUE;
// patern need to sort by order of check
static PATTERN: [(u8, usize, &str); 8] = [
    (0xF8, 6, "five"),                  // five XXXXX...
    (0x74, 7, "split four 3"),          // split four 3 .XXX.X..
    (0x6C, 7, "split four 2"),          // split four 2 .XX.XX..
    (0x5C, 7, "split four 1"),          // split four 1 .X.XXX..
	// (0xE8, 7, "close split four 3"),    // close split four 3 XXX.X...
    // (0xD8, 7, "close split four 2"),    // close split four 2 XX.XX...
    // (0xB8, 7, "close split four 1"),    // close split four 1 X.XXX...
    // (0xF0, 5, "close four"),            // close four XXXX....
    // (0xB0, 5, "close split three"),     // close split three X.XX....
    // (0xD0, 5, "close split three rev"), // close split three rev XX.X....
    // (0xE0, 4, "close three"),           // close three XXX.....
    (0x78, 6, "open four"),             // open four .XXXX...
    (0x68, 6, "open split three"),      // open split three .X.XX...
    (0x58, 6, "open split three rev"),  // open split three rev .XX.X...
    (0x70, 5, "open three"),			// open three .XXX....
];

static BLOCKER: [(u8, usize); 5] = [
	(0x82, 7), // X.....X.
	(0x84, 6), // X....X..
	(0x88, 5), // X...X...
	(0x90, 4), // X..X....
	(0xA0, 3), // X.X.....
];

pub fn pattern_axes_dispatcher(axes: &[[u16; 4]; 2], pos: usize, player: i8) {
    if player == 1 {
        println!("white player pattern in row:");
        pattern_axes_finder(&axes[0], &axes[1], pos);
    } else if player == -1 {
        println!("black player pattern in row:");
        pattern_axes_finder(&axes[1], &axes[0], pos);
    }
}

fn print_axe_value(axe:usize){
	if axe == 0 { println!("DIAGONALE UPLEFT:")}
	else if axe == 1 { println!("COLONE:")}
	else if axe == 2 { println!("DIAGONALE UPRIGHT:")}
	else { println!("LIGNE:")}
}

pub fn check_in_map(
	axe_mouvement_value: i16,
	pattern_pos: i16,
	offset: i16,
	direction_sign: i16,
) -> bool {
	let calcul = (pattern_pos + axe_mouvement_value * offset * direction_sign);
	if calcul < 0 { return false;}
	let line_checked = calcul / 19;
	let line = pattern_pos / 19;
	//ligne
	if axe_mouvement_value == 1 {
		if line_checked != line
		{
			return false;
		}
	} else {
		println!("checkpos {} pos {}",calcul,pattern_pos);
		 println!("line check {} diff {}",line_checked,line + offset * direction_sign);
		if line_checked != line + offset * direction_sign 
		{
			return false;
		}
	}
	return true;
}

fn check_border(pos: usize, l: usize, axe: usize, pattern_length: usize) -> bool {
	let axe_mouvement_value: i16 = AXE_MOUVEMENT_VALUE[axe] as i16;
	let pattern_pos: i16 = pos as i16 - l as i16 * axe_mouvement_value;
	if check_in_map(axe_mouvement_value, pattern_pos, 1,-1) == false {
		return false;
	}
	if check_in_map(axe_mouvement_value, pattern_pos, pattern_length as i16, 1) == false {
		return false;
	}
	return true;
}

fn pattern_axes_finder(axes: &[u16; 4], blocker_axes: &[u16; 4], pos: usize) {
    let y = pos % 19;
	println!("y= {}",y);
	for axe in 0..axes.len() {
		print_axe_value(axe);
		let mut player_axe = axes[axe];
		let mut blocker_axe = blocker_axes[axe];
		let mut found_pattern: (&str, usize) = ("",0);
		player_axe >>= 1;
		blocker_axe >>= 1;
		println!("player axe: {:016b}", player_axe);
		for l in 0..4 {
			let mut player_shifted = player_axe >> l;
			println!("player shifted: {:016b} l= {}", player_shifted, l);
			let mut blocker_shifted = blocker_axe >> l;
			let player_casted = player_shifted as u8;
			let blocker_casted = blocker_shifted as u8;
			let mut is_bloked: usize = 0;
			for p in 0..PATTERN.len() {
				if (player_casted & PATTERN[p].0) == PATTERN[p].0 {
					// println!("player casted: {:08b}", player_casted);
					for b in 0..BLOCKER.len() {
						if BLOCKER[b].1 == PATTERN[p].1 {
							// println!("blocker  cast: {:08b}", blocker_casted);
							// println!("blocked checked: {:08b}", blocker_casted);
							// println!("pattern checked: {:08b}", BLOCKER[b].0);
							let mut blocker_checker: u8 = (blocker_casted & BLOCKER[b].0);
							// println!("BLOCKER CHECKER: {:08b}", blocker_checker);
							// println!("l {} length {}", l , PATTERN[p].1);
							if blocker_checker == BLOCKER[b].0 {
								is_bloked = 2;
							}
							else if blocker_checker != 0 {
								is_bloked = 1;
								if check_border(pos, l, axe, PATTERN[p].1) == false {is_bloked += 1}
							}
							else if check_border(pos, l, axe, PATTERN[p].1) == false {
								is_bloked = 1;
								if blocker_checker != 0 {is_bloked += 1}
							}
							else {
								is_bloked = 0;
							}
						}
					}
					if is_bloked < 2 {
						found_pattern = (PATTERN[p].2.clone(), is_bloked);
						println!("{} found {} blocker", PATTERN[p].2, is_bloked);
						break ;
					}
				}
				if found_pattern.0 != "" { break ;}
			}
		}
		println!("PATTERN FOUND {} BLOCKER FOUND {}", found_pattern.0, found_pattern.1);
	}
}

// pub fn pattern_dispatcher(bitboards: &Bitboards, pos: usize, player: i8) {
//     if player == 1 {
//         println!("white player pattern in row:");
//         pattern_finder(&bitboards.white_board, &bitboards.black_board, pos);
//     } else if player == -1 {
//         println!("black player pattern in row:");
//         pattern_finder(&bitboards.black_board, &bitboards.white_board, pos);
//     }
// }

// fn pattern_finder(bitboard: &[u64; 6], blocker: &[u64; 6], pos: usize) {
//     let y = pos % 19;
//     let mut player_row = create_row(bitboard, pos);
// 	let mut blocker_row = create_row(blocker, pos);
//     println!("player checked: {:064b}", player_row);
// 	println!("blocker  check: {:064b}", blocker_row);
//     player_row <<= 7;
// 	blocker_row <<= 7;
// 	let mut found_pattern: (&str, usize) = ("",0);
//     for l in (0..19).rev() {
//         let mut player_shifted = player_row >> l;
// 		let mut blocker_shifted = blocker_row >> l;
//         let player_casted = player_shifted as u8;
// 		let blocker_casted = blocker_shifted as u8;
// 		let mut is_bloked: usize = 0;
//         for p in 0..PATTERN.len() {
//             if (player_casted & PATTERN[p].0) == PATTERN[p].0 {
// 				// println!("player casted: {:08b}", player_casted);
// 				for b in 0..BLOCKER.len() {
// 					if BLOCKER[b].1 == PATTERN[p].1 {
// 						// println!("blocker  cast: {:08b}", blocker_casted);
// 						// println!("blocked checked: {:08b}", blocker_casted);
//            				// println!("pattern checked: {:08b}", BLOCKER[b].0);
// 						let mut blocker_checker: u8 = (blocker_casted & BLOCKER[b].0);
// 						// println!("BLOCKER CHECKER: {:08b}", blocker_checker);
// 						// println!("l {} length {}", l , PATTERN[p].1);
// 						if blocker_checker == BLOCKER[b].0 {
// 							is_bloked = 2;
// 						}
// 						else if blocker_checker != 0 {
// 							is_bloked = 1;
// 							if l <= PATTERN[p].1 || l == 18 {is_bloked += 1;}
// 						}
// 						else if l <= PATTERN[p].1 || l == 18 {
// 							is_bloked = 1;
// 							if blocker_checker != 0 {is_bloked += 1;}
// 						}
// 						else {
// 							is_bloked = 0;
// 						}
// 					}
// 				}
// 				if is_bloked < 2 {
// 					found_pattern = (PATTERN[p].2.clone(), is_bloked);
//                 	println!("{} found {} blocker", PATTERN[p].2, is_bloked);
// 					break ;
// 				}
//             }
// 			if found_pattern.0 != "" { break ;}
//         }
//     }
// 	println!("PATTERN FOUND {} BLOCKER FOUND {}", found_pattern.0, found_pattern.1);
// }

// fn create_row(bitboard: &[u64; 6], pos: usize) -> u64 {
//     let mask: [u64; 24] = [
//         0xFFFFE00000000000,
//         0x1FFFFC000000,
//         0x3FFFF80,
//         0x7F,
//         0xFFFFE00000000,
//         0x1FFFFC000,
//         0x3FFF,
//         0x7FFFF0000000000,
//         0xFFFFE00000,
//         0x1FFFFC,
//         0x3,
//         0x7FFFF0000000,
//         0xFFFFE00,
//         0x1FF,
//         0x3FFFF800000000,
//         0x7FFFF0000,
//         0xFFFF,
//         0x1FFFFC0000000000,
//         0x3FFFF800000,
//         0xFFF0000000000000,
//         0xF800000000000000,
//         0xFFFF800000000000,
//         0xFFC0000000000000,
//         0xE000000000000000,
//     ];
//     let row_idx = pos / 19;
//     let int_idx = (row_idx * 19) / 64;
//     // println!("rowidx {}, intidx {}", row_idx, int_idx);
//     let shift = (row_idx * 19) % 64;
//     let mut row: u64;
//     if (row_idx == 3 || row_idx == 6 || row_idx == 10 || row_idx == 13 || row_idx == 16) {
//         row = (((bitboard[int_idx] & mask[row_idx]) << shift)
//             | ((bitboard[int_idx + 1] & mask[19 + int_idx]) >> (64 - shift)));
//         // println!("generated mixed row: {:064b}", row);
//     } else {
//         row = (bitboard[int_idx] & mask[row_idx]) << shift;
//         // println!("generated row: {:064b}", row);
//     }
//     return row >> 45;
// }