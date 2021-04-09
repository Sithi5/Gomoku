use std::collections::HashMap;

use crate::bitboards::remove_bit;
use crate::bitboards::Bitboards;
use crate::global_var::AXE_MOUVEMENT_VALUE;
use crate::print_bitboards;

// patern need to sort by order of check
static PATTERN: [(u8, usize, usize, &str); 8] = [
    (0xF8, 6, 0, "five"),         // five XXXXX...
    (0x74, 7, 4, "split four 3"), // split four 3 .XXX.X..
    (0x6C, 7, 3, "split four 2"), // split four 2 .XX.XX..
    (0x5C, 7, 2, "split four 1"), // split four 1 .X.XXX..
    // (0xE8, 7, "close split four 3"),    // close split four 3 XXX.X...
    // (0xD8, 7, "close split four 2"),    // close split four 2 XX.XX...
    // (0xB8, 7, "close split four 1"),    // close split four 1 X.XXX...
    // (0xF0, 5, "close four"),            // close four XXXX....
    // (0xB0, 5, "close split three"),     // close split three X.XX....
    // (0xD0, 5, "close split three rev"), // close split three rev XX.X....
    // (0xE0, 4, "close three"),           // close three XXX.....
    (0x78, 6, 0, "open four"),            // open four .XXXX...
    (0x58, 6, 2, "open split three"),     // open split three .X.XX...
    (0x68, 6, 3, "open split three rev"), // open split three rev .XX.X...
    (0x70, 5, 0, "open three"),           // open three  .XXX....
];

static CAPTURE_PATTERN: [(u8, usize, &str); 2] = [
    (0x90, 5, "capturing pair"), // capturing pair	X..X....
    (0x60, 4, "open double"),    // open double 	.XX.....
];

static BLOCKER: [(u8, usize); 5] = [
    (0x82, 7), // X.....X.
    (0x84, 6), // X....X..
    (0x88, 5), // X...X...
    (0x90, 4), // X..X....
    (0xA0, 3), // X.X.....
];

pub fn pattern_axes_dispatcher(
    bitboards: &mut Bitboards,
    axes: &[[u16; 4]; 2],
    pos: usize,
    player: i8,
) -> HashMap<String, i8> {
    let mut pattern_return_infos: HashMap<String, i8> = HashMap::new();
    let mut axe_pattern: [(usize, usize); 4] = [(0, 0), (0, 0), (0, 0), (0, 0)];
    if player == 1 {
        println!("white player pattern in row:");
        // check and apply capture
        pattern_return_infos.insert(
            String::from("stone_captured"),
            check_and_apply_capture(bitboards, &axes[0], &axes[1], pos, player),
        );
        check_flank(&axes[0], &axes[1]);
        axe_pattern = pattern_axes_finder(&axes[0], &axes[1], pos);
    } else if player == -1 {
        println!("black player pattern in row:");
        pattern_return_infos.insert(
            String::from("stone_captured"),
            check_and_apply_capture(bitboards, &axes[1], &axes[0], pos, player),
        );
        check_flank(&axes[1], &axes[0]);
        axe_pattern = pattern_axes_finder(&axes[1], &axes[0], pos);
    }
    println!("double triple {}", check_double_triple(axe_pattern));
    println!("pattern on axe {:?}", axe_pattern);
    return pattern_return_infos;
}

fn check_double_triple(axe_pattern: [(usize, usize); 4]) -> bool {
    let mut count = 0;
    for axe in 0..axe_pattern.len() {
        if axe_pattern[axe].1 == 0 {
            if axe_pattern[axe].0 == 7 || axe_pattern[axe].0 == 6 {
                count += 1;
            }
        }
    }
    return if count >= 2 { false } else { true };
}

fn check_and_apply_capture(
    bitboards: &mut Bitboards,
    axes: &[u16; 4],
    blocker_axes: &[u16; 4],
    pos: usize,
    player: i8,
) -> i8 {
    let mut stone_captured: i8 = 0;
    for axe in 0..axes.len() {
        let mut player_axe = axes[axe];
        let mut blocker_axe = blocker_axes[axe];
        player_axe >>= 1;
        blocker_axe >>= 1;
        println!("player axe: {:016b}", player_axe);
        println!("block  axe: {:016b}", blocker_axe);
        let shift: [usize; 2] = [0, 3];
        for s in shift.iter() {
            let player_shifted = player_axe >> s;
            println!("player shifted: {:016b} l= {}", player_shifted, s);
            let blocker_shifted = blocker_axe >> s;
            let player_casted = player_shifted as u8;
            let blocker_casted = blocker_shifted as u8;
            if (player_casted & CAPTURE_PATTERN[0].0) == CAPTURE_PATTERN[0].0 {
                if (blocker_casted & CAPTURE_PATTERN[1].0) == CAPTURE_PATTERN[1].0 {
                    println!("captured");
                    println!(
                        "axe: {}, direction {}, pos {}",
                        AXE_MOUVEMENT_VALUE[axe], s, pos
                    );
                    if *s == 3 {
                        stone_captured += 2;
                        apply_capture(bitboards, AXE_MOUVEMENT_VALUE[axe], -1, pos, player);
                    } else {
                        stone_captured += 2;
                        apply_capture(bitboards, AXE_MOUVEMENT_VALUE[axe], 1, pos, player);
                    }
                }
                print_bitboards(bitboards);
            }
        }
    }
    return stone_captured;
}

fn apply_capture(bitboards: &mut Bitboards, axe: usize, s: isize, pos: usize, player: i8) {
    let opponent = -player;
    for n in 1..3 {
        if s == -1 {
            remove_bit(bitboards, pos - (n * axe), opponent);
        } else {
            remove_bit(bitboards, pos + (n * axe), opponent);
        }
    }
}

fn check_flank(axes: &[u16; 4], blocker_axes: &[u16; 4]) {
    for axe in 0..axes.len() {
        let mut player_axe = axes[axe];
        let mut blocker_axe = blocker_axes[axe];
        player_axe >>= 1;
        blocker_axe >>= 1;
        println!("player axe: {:016b}", player_axe);
        println!("block  axe: {:016b}", blocker_axe);
        let shift: [usize; 2] = [1, 2];
        for s in shift.iter() {
            let player_shifted = player_axe >> s;
            let blocker_shifted = blocker_axe >> s;
            let player_casted = player_shifted as u8;
            let blocker_casted = blocker_shifted as u8;
            if (player_casted & CAPTURE_PATTERN[1].0) == CAPTURE_PATTERN[1].0 {
                if (blocker_casted & CAPTURE_PATTERN[0].0) == CAPTURE_PATTERN[0].0 {
                    println!("blocked");
                } else if (blocker_casted & CAPTURE_PATTERN[0].0) != 0 {
                    println!("flank");
                } else {
                    println!("free");
                }
            }
        }
    }
}

fn print_axe_value(axe: usize) {
    if axe == 0 {
        println!("DIAGONALE UPLEFT:")
    } else if axe == 1 {
        println!("COLONE:")
    } else if axe == 2 {
        println!("DIAGONALE UPRIGHT:")
    } else {
        println!("LIGNE:")
    }
}

fn check_in_map(
    axe_mouvement_value: i16,
    pattern_pos: i16,
    offset: i16,
    direction_sign: i16,
) -> bool {
    let calcul = pattern_pos + axe_mouvement_value * offset * direction_sign;
    if calcul < 0 {
        return false;
    }
    let line_checked = calcul / 19;
    let line = pattern_pos / 19;
    //ligne
    if axe_mouvement_value == 1 {
        if line_checked != line {
            return false;
        }
    } else {
        println!("checkpos {} pos {}", calcul, pattern_pos);
        println!(
            "line check {} diff {}",
            line_checked,
            line + offset * direction_sign
        );
        if line_checked != line + offset * direction_sign {
            return false;
        }
    }
    return true;
}

fn check_border(pos: usize, l: usize, axe: usize, pattern_length: usize) -> bool {
    let axe_mouvement_value: i16 = AXE_MOUVEMENT_VALUE[axe] as i16;
    let pattern_pos: i16 = pos as i16 - l as i16 * axe_mouvement_value;
    if check_in_map(axe_mouvement_value, pattern_pos, 1, -1) == false {
        return false;
    }
    if check_in_map(axe_mouvement_value, pattern_pos, pattern_length as i16, 1) == false {
        return false;
    }
    return true;
}

fn pattern_axes_finder(
    axes: &[u16; 4],
    blocker_axes: &[u16; 4],
    pos: usize,
) -> [(usize, usize); 4] {
    let mut return_pattern: [(usize, usize); 4] = [(0, 0), (0, 0), (0, 0), (0, 0)];
    let mut is_blocked: usize;
    for axe in 0..axes.len() {
        print_axe_value(axe);
        let mut player_axe = axes[axe];
        let mut blocker_axe = blocker_axes[axe];
        let mut found_pattern: (usize, usize) = (PATTERN.len(), 0);
        player_axe >>= 1;
        blocker_axe >>= 1;
        for l in 0..6 {
            let player_shifted = player_axe >> l;
            let blocker_shifted = blocker_axe >> l;
            let player_casted = player_shifted as u8;
            let blocker_casted = blocker_shifted as u8;
            is_blocked = 0;
            for p in 0..PATTERN.len() {
                if (player_casted & PATTERN[p].0) == PATTERN[p].0 {
                    for b in 0..BLOCKER.len() {
                        if BLOCKER[b].1 == PATTERN[p].1 {
                            let blocker_checker: u8 = blocker_casted & BLOCKER[b].0;
                            println!("pattern {}", PATTERN[p].3);
                            is_blocked =
                                check_blocker(blocker_checker, blocker_casted, pos, b, p, l, axe);
                        }
                    }
                    if is_blocked < 2 && p < found_pattern.0 {
                        found_pattern.0 = p;
                        found_pattern.1 = is_blocked;
                        println!("{} found {} blocker", PATTERN[p].3, is_blocked);
                        break;
                    }
                }
            }
        }
        if found_pattern.0 < PATTERN.len() {
            return_pattern[axe] = found_pattern;
            println!("PATTERN FOUND {}", PATTERN[found_pattern.0].3,);
        }
    }
    return return_pattern;
}

fn check_blocker(
    blocker_checker: u8,
    blocker_casted: u8,
    pos: usize,
    b: usize,
    p: usize,
    l: usize,
    axe: usize,
) -> usize {
    let mut is_blocked: usize;
    if PATTERN[p].2 != 0 && check_one_bit_in_pattern(&blocker_casted, PATTERN[p].2) == true {
        is_blocked = 2;
    } else if blocker_checker == BLOCKER[b].0 {
        is_blocked = 2;
    } else if blocker_checker != 0 {
        is_blocked = 1;
        if check_border(pos, l, axe, PATTERN[p].1) == false {
            is_blocked += 1
        }
    } else if check_border(pos, l, axe, PATTERN[p].1) == false {
        is_blocked = 1;
        if blocker_checker != 0 {
            is_blocked += 1
        }
    } else {
        is_blocked = 0;
    }
    return is_blocked;
}

fn check_one_bit_in_pattern(pattern: &u8, length: usize) -> bool {
    let checked_pos = 8 - length;
    // let mask : u8 = 1 << checked_pos;
    let mask: u8 = 0x80 >> length;
    println!(
        "mask {:08b} legnth {} , checkpos {}",
        mask, length, checked_pos
    );
    if pattern & mask != 0 {
        return true;
    } else {
        return false;
    }
}
// TO DO
// fn check_unblocable_five(bitboards: &mut Bitboards , pos: usize, axe: usize, player: i8) -> bool {
// 	for n in 0..5 {
// 		if bitboards.player.pos[n * direction] == flanked
// 			return false;
// 	}
// 	return true;
// }
