#![allow(non_snake_case)]
#![allow(unused_imports)] //remove

use asr::{
    future::{next_tick, retry},
    print_message,
    settings::Gui,
    signature::Signature,
    time::Duration,
    timer::{
        pause_game_time, reset, set_game_time, set_variable, set_variable_float, set_variable_int,
        split, start, state, TimerState,
    },
    watcher::Watcher,
    Address,
    PointerSize::Bit64,
    Process,
};
mod offsets;
mod splitter_settings;
use crate::offsets::get_offsets;

asr::async_main!(stable);

const PROCESS_NAMES: &[&str] = &["MinaTheHollower.exe", "MinaTheHollower"];

async fn main() {
    let mut settings = splitter_settings::Settings::register();

    print_message("Setup done. Waiting for Process.");

    loop {
        let found: (&str, Process) = retry(|| {
            PROCESS_NAMES
                .iter()
                .find_map(|&name| Process::attach(name).map(|proc| (name, proc)))
        })
        .await;
        let process_name = found.0;
        let process = found.1; //Process::wait_attach(process_name).await;

        let platform: &str;
        match process_name {
            "MinaTheHollower" => {
                platform = "Linux";
            }
            "MinaTheHollower.exe" => {
                platform = "Windows";
            }
            _ => {
                print_message("unknown platform");
                platform = "";
            }
        }
        set_variable("Platform", platform);

        // Game Timer
        let mut watch_fPlayTime: Watcher<f64> = Watcher::new();
        watch_fPlayTime.update_infallible(-1.0f64);

        // state watch
        let mut watch_sCheckpointGamestate: Watcher<u32> = Watcher::new();
        watch_sCheckpointGamestate.update_infallible(0u32);

        process
            .until_closes(async {
                print_message("Process found.");

                // track which splits have already been completed so that they only trigger once
                struct SplitsCompleted {
                    generators: u8,
                    game_cleared: bool,
                }

                impl SplitsCompleted {
                    fn new() -> Self {
                        SplitsCompleted {
                            generators: 0x00,
                            game_cleared: false,
                        }
                    }
                }

                let mut splits_completed = SplitsCompleted::new();

                if let Some(offset_arrays) = get_offsets(&process, process_name) {
                    print_message("Starting Loop.");
                    loop {
                        settings.update();

                        // Game Timer
                        if let Ok(time) = process.read_pointer_path::<f64>(
                            offset_arrays.savemanager,
                            Bit64,
                            &offset_arrays.fPlayTime,
                        ) {
                            watch_fPlayTime.update_infallible(time);
                            set_variable_float("fPlayTime", time);
                            //set_game_time(Duration::seconds_f64(time));
                        }

                        if let Ok(time) = process.read_pointer_path::<f64>(
                            offset_arrays.savemanager,
                            Bit64,
                            &offset_arrays.fPlayTimeCleared,
                        ) {
                            set_variable_float("fPlayTimeCleared", time);
                            set_game_time(Duration::seconds_f64(time));
                        }

                        if let Ok(time) = process.read_pointer_path::<f64>(
                            offset_arrays.savemanager,
                            Bit64,
                            &offset_arrays.fPlayTimeTotal,
                        ) {
                            set_variable_float("fPlayTimeTotal", time);
                            //set_game_time(Duration::seconds_f64(time));
                        }

                        // split logic variables
                        let mut generator: u32 = 0;
                        if let Ok(g) = process.read_pointer_path::<u32>(
                            offset_arrays.savemanager,
                            Bit64,
                            &offset_arrays.generatorActivated,
                        ) {
                            generator = g;
                            set_variable_int("generatorActivated", generator);
                        }

                        let mut bGameCleared: u8 = 0;
                        if let Ok(bc) = process.read_pointer_path::<u8>(
                            offset_arrays.savemanager,
                            Bit64,
                            &offset_arrays.bGameCleared,
                        ) {
                            bGameCleared = bc;
                            set_variable_int("bGameCleared", bGameCleared as i32);
                        }

                        if let Ok(state) = process.read_pointer_path::<u32>(
                            offset_arrays.savemanager,
                            Bit64,
                            &offset_arrays.sCheckpointGamestate,
                        ) {
                            set_variable_int("sCheckpointGamestate", state);
                            watch_sCheckpointGamestate.update_infallible(state);
                            if let Some(sCheckpointGamestate) = &watch_sCheckpointGamestate.pair {
                                if sCheckpointGamestate.changed() && state == 1270270836u32 {
                                    print_message("i believe this was a reset")
                                }
                            }
                        }

                        match state() {
                            TimerState::NotRunning => {
                                // start timer
                                if let Some(fPlayTime) = &watch_fPlayTime.pair {
                                    if fPlayTime.old != fPlayTime.current
                                        && fPlayTime.old == 0f64
                                        && fPlayTime.current > 0f64
                                    {
                                        start();
                                        //reset_all();
                                    }
                                }
                            }
                            TimerState::Paused => {}
                            TimerState::Running => {
                                // reset and start timer, made to handle quitting to title and reloading the profile
                                if let Some(fPlayTime) = &watch_fPlayTime.pair {
                                    if fPlayTime.old == 0f64
                                        && fPlayTime.current > 0f64
                                        && fPlayTime.current < 1f64
                                    {
                                        reset_all();
                                    }
                                }

                                // split logic
                                if settings.queensbury_crypt
                                    && (generator & 0x02) != 0
                                    && (splits_completed.generators & 0x02) == 0
                                {
                                    splits_completed.generators |= 0x02;
                                    split();
                                } else if settings.nox_bayou
                                    && (generator & 0x04) != 0
                                    && (splits_completed.generators & 0x04) == 0
                                {
                                    splits_completed.generators |= 0x04;
                                    split();
                                } else if settings.septemburg
                                    && (generator & 0x08) != 0
                                    && (splits_completed.generators & 0x08) == 0
                                {
                                    splits_completed.generators |= 0x08;
                                    split();
                                } else if settings.bone_beach
                                    && (generator & 0x10) != 0
                                    && (splits_completed.generators & 0x10) == 0
                                {
                                    splits_completed.generators |= 0x10;
                                    split();
                                } else if settings.coltrane_peak
                                    && (generator & 0x20) != 0
                                    && (splits_completed.generators & 0x20) == 0
                                {
                                    splits_completed.generators |= 0x20;
                                    split();
                                } else if settings.astral_orrery
                                    && (generator & 0x40) != 0
                                    && (splits_completed.generators & 0x40) == 0
                                {
                                    splits_completed.generators |= 0x40;
                                    split();
                                } else if settings.game_cleared
                                    && bGameCleared != 0
                                    && !splits_completed.game_cleared
                                {
                                    splits_completed.game_cleared = true;
                                    split();
                                }
                            }
                            _ => {}
                        }

                        next_tick().await;
                    }
                }
            })
            .await;
    }
}

fn reset_all(/*split_states: &mut [bool;32]*/) {
    //split_states.fill(false);

    reset();
    start();
    pause_game_time();
}
