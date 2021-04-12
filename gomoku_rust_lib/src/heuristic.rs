use crate::check_bits::check_is_wrong_move;
use crate::global_var;
use crate::state::State;
use crate::HashMap;
use crate::checking_and_apply_bits_move;
use crate::check_pos_still_win;

pub struct Board_state_info {
	pub is_wrong_move : i8,
	pub stone_captured: i8,
	pub flank : i8,
	pub pattern_value: i32,
	pub is_winning: isize
}

pub fn heuristic(state: &mut State) -> i32 {
    let mut value: i32 = 0;
	let board_check: Board_state_info = checking_and_apply_bits_move(state);
	if state.win_state != 362 {
		let winning_player = if state.win_state < 0 {-1} else {1};
		if check_pos_still_win(state.bitboards,state.win_state) == true {
			if state.current_player == winning_player {value = 5000;}
			else {value = -5000;}
			return value;
		}
		else {state.win_state = 362;}
	} 
	if board_check.is_winning != 362 {
		state.win_state = board_check.is_winning;
	}
    if board_check.is_wrong_move != global_var::VALID_MOVE {
        value = -1000;
    } else {
        value += board_check.pattern_value as i32; 
    }
    return value;
}
