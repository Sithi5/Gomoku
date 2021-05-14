use crate::algorithms;
use crate::bitboard_operations::apply_bit;
use crate::data_struct;
use crate::data_struct::Bitboards;
use crate::global_var;
use crate::heuristic_ratios;
use crate::state;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use stoppable_thread;

pub fn test_negamax_benchmark() {
    let time_before_kill_process = Duration::from_secs(1);

    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 180;
    apply_bit(&mut bitboards, 0, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 1, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, 19, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 120, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, 181, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 25, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);

    let state: data_struct::State = state::create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        pos,
        0,
        0,
        (0, 0),
        0,
    );

    // Try with depth 1
    let mut copy_state = state.clone();
    let handle = stoppable_thread::spawn(move |stopped| {
        let depth = 1;
        println!("DEPTH {} :", depth);
        let start_time = Instant::now();
        algorithms::negamax(
            &mut copy_state,
            depth,
            heuristic_ratios::HEURISTIC_MIN_VALUE,
            heuristic_ratios::HEURISTIC_MAX_VALUE,
            1,
        );
        algorithms::return_move(&mut copy_state);
        let end_time = Instant::now();
        println!("time to process {:?}", end_time.duration_since(start_time));
        println!();
    });
    thread::sleep(time_before_kill_process);
    println!("Stopping thread...");
    handle.stop();

    // Try with depth 2
    let mut copy_state = state.clone();
    let handle = stoppable_thread::spawn(move |stopped| {
        let depth = 2;
        println!("DEPTH {} :", depth);
        let start_time = Instant::now();
        algorithms::negamax(
            &mut copy_state,
            depth,
            heuristic_ratios::HEURISTIC_MIN_VALUE,
            heuristic_ratios::HEURISTIC_MAX_VALUE,
            1,
        );
        algorithms::return_move(&mut copy_state);
        let end_time = Instant::now();
        println!("time to process {:?}", end_time.duration_since(start_time));
        println!();
    });

    thread::sleep(time_before_kill_process);
    println!("Stopping thread...");
    handle.stop();

    // Try with depth 3
    let mut copy_state = state.clone();
    let handle = stoppable_thread::spawn(move |stopped| {
        let depth = 3;
        println!("DEPTH {} :", depth);
        let start_time = Instant::now();
        algorithms::negamax(
            &mut copy_state,
            depth,
            heuristic_ratios::HEURISTIC_MIN_VALUE,
            heuristic_ratios::HEURISTIC_MAX_VALUE,
            1,
        );
        algorithms::return_move(&mut copy_state);
        let end_time = Instant::now();
        println!("time to process {:?}", end_time.duration_since(start_time));
        println!();
    });

    thread::sleep(time_before_kill_process);
    println!("Stopping thread...");
    handle.stop();

    // Try with depth 4
    let mut copy_state = state.clone();
    let handle = stoppable_thread::spawn(move |stopped| {
        let depth = 4;
        println!("DEPTH {} :", depth);
        let start_time = Instant::now();
        algorithms::negamax(
            &mut copy_state,
            depth,
            heuristic_ratios::HEURISTIC_MIN_VALUE,
            heuristic_ratios::HEURISTIC_MAX_VALUE,
            1,
        );
        algorithms::return_move(&mut copy_state);
        let end_time = Instant::now();
        println!("time to process {:?}", end_time.duration_since(start_time));
        println!();
    });

    thread::sleep(time_before_kill_process);
    println!("Stopping thread...");
    handle.stop();

    // Try with depth 5
    let mut copy_state = state.clone();
    let handle = stoppable_thread::spawn(move |stopped| {
        let depth = 5;
        println!("DEPTH {} :", depth);
        let start_time = Instant::now();
        algorithms::negamax(
            &mut copy_state,
            depth,
            heuristic_ratios::HEURISTIC_MIN_VALUE,
            heuristic_ratios::HEURISTIC_MAX_VALUE,
            1,
        );
        algorithms::return_move(&mut copy_state);
        let end_time = Instant::now();
        println!("time to process {:?}", end_time.duration_since(start_time));
        println!();
    });

    thread::sleep(time_before_kill_process);
    println!("Stopping thread...");
    handle.stop();

    // Try with depth 6
    let mut copy_state = state.clone();
    let handle = stoppable_thread::spawn(move |stopped| {
        let depth = 6;
        println!("DEPTH {} :", depth);
        let start_time = Instant::now();
        algorithms::negamax(
            &mut copy_state,
            depth,
            heuristic_ratios::HEURISTIC_MIN_VALUE,
            heuristic_ratios::HEURISTIC_MAX_VALUE,
            1,
        );
        algorithms::return_move(&mut copy_state);
        let end_time = Instant::now();
        println!("time to process {:?}", end_time.duration_since(start_time));
        println!();
    });

    thread::sleep(time_before_kill_process);
    println!("Stopping thread...");
    handle.stop();

    // Try with depth 7
    let mut copy_state = state.clone();
    let handle = stoppable_thread::spawn(move |stopped| {
        let depth = 7;
        println!("DEPTH {} :", depth);
        let start_time = Instant::now();
        algorithms::negamax(
            &mut copy_state,
            depth,
            heuristic_ratios::HEURISTIC_MIN_VALUE,
            heuristic_ratios::HEURISTIC_MAX_VALUE,
            1,
        );
        algorithms::return_move(&mut copy_state);
        let end_time = Instant::now();
        println!("time to process {:?}", end_time.duration_since(start_time));
        println!();
    });

    thread::sleep(time_before_kill_process);
    println!("Stopping thread...");
    handle.stop();

    // Try with depth 8
    let mut copy_state = state.clone();
    let handle = stoppable_thread::spawn(move |stopped| {
        let depth = 8;
        println!("DEPTH {} :", depth);
        let start_time = Instant::now();
        algorithms::negamax(
            &mut copy_state,
            depth,
            heuristic_ratios::HEURISTIC_MIN_VALUE,
            heuristic_ratios::HEURISTIC_MAX_VALUE,
            1,
        );
        algorithms::return_move(&mut copy_state);
        let end_time = Instant::now();
        println!("time to process {:?}", end_time.duration_since(start_time));
        println!();
    });

    thread::sleep(time_before_kill_process);
    println!("Stopping thread...");
    handle.stop();

    // Try with depth 9
    let mut copy_state = state.clone();
    let handle = stoppable_thread::spawn(move |stopped| {
        let depth = 9;
        println!("DEPTH {} :", depth);
        let start_time = Instant::now();
        algorithms::negamax(
            &mut copy_state,
            depth,
            heuristic_ratios::HEURISTIC_MIN_VALUE,
            heuristic_ratios::HEURISTIC_MAX_VALUE,
            1,
        );
        algorithms::return_move(&mut copy_state);
        let end_time = Instant::now();
        println!("time to process {:?}", end_time.duration_since(start_time));
        println!();
    });

    thread::sleep(time_before_kill_process);
    println!("Stopping thread...");
    handle.stop();

    // Try with depth 10
    let mut copy_state = state.clone();
    let handle = stoppable_thread::spawn(move |stopped| {
        let depth = 10;
        println!("DEPTH {} :", depth);
        let start_time = Instant::now();
        algorithms::negamax(
            &mut copy_state,
            depth,
            heuristic_ratios::HEURISTIC_MIN_VALUE,
            heuristic_ratios::HEURISTIC_MAX_VALUE,
            1,
        );
        algorithms::return_move(&mut copy_state);
        let end_time = Instant::now();
        println!("time to process {:?}", end_time.duration_since(start_time));
        println!();
    });

    thread::sleep(time_before_kill_process);
    println!("Stopping thread...");
    handle.stop();
}
