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

// track which splits have already been completed so that they only trigger once
struct SplitsCompleted {
    generators: u8,
    game_cleared: bool,
    map_seen: [bool; 17],
}

impl SplitsCompleted {
    fn new() -> Self {
        SplitsCompleted {
            generators: 0x00,
            game_cleared: false,
            map_seen: [false; 17],
        }
    }

    fn reset(&mut self) {
        self.generators = 0x00;
        self.game_cleared = false;
        self.map_seen = [false; 17];
    }
}

asr::async_main!(stable);

const PROCESS_NAMES: &[&str] = &["MinaTheHollower", "MinaTheHollower.exe"];

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

                        let mut mapSeen_bytes = [0u8; 17];
                        for i in 0..17u64 {
                            if let Ok(byte) = process.read_pointer_path::<u8>(
                                offset_arrays.savemanager,
                                Bit64,
                                &[offset_arrays.mapSeen[0], offset_arrays.mapSeen[1] + i],
                            ) {
                                mapSeen_bytes[i as usize] = byte;
                            }
                        }
                        set_variable_int(
                            "map_seen_count",
                            mapSeen_bytes.iter().filter(|&&b| b != 0).count() as i32,
                        );
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
                                    if settings.auto_reset_start
                                        && fPlayTime.old == 0f64
                                        && fPlayTime.current > 0f64
                                        && fPlayTime.current < 1f64
                                    {
                                        reset_all(&mut splits_completed);
                                        start();
                                    }
                                }
                            }
                            TimerState::Paused | TimerState::Running => {
                                // reset and start timer, made to handle quitting to title and reloading the profile
                                if let Some(fPlayTime) = &watch_fPlayTime.pair {
                                    if settings.auto_reset_start
                                        && fPlayTime.old == 0f64
                                        && fPlayTime.current > 0f64
                                        && fPlayTime.current < 1f64
                                    {
                                        reset_all(&mut splits_completed);
                                        start();
                                    }
                                    if fPlayTime.current < 1f64 {
                                        set_game_time(Duration::seconds_f64(fPlayTime.current));
                                    }
                                }
                                // split logic

                                // enter area
                                if settings.astral_orrery_enter
                                    && mapSeen_bytes[0] != 0
                                    && !splits_completed.map_seen[0]
                                {
                                    splits_completed.map_seen[0] = true;
                                    split();
                                } else if settings.backwaters_enter
                                    && mapSeen_bytes[1] != 0
                                    && !splits_completed.map_seen[1]
                                {
                                    splits_completed.map_seen[1] = true;
                                    split();
                                } else if settings.noxs_bayou_enter
                                    && mapSeen_bytes[2] != 0
                                    && !splits_completed.map_seen[2]
                                {
                                    splits_completed.map_seen[2] = true;
                                    split();
                                } else if settings.sandfalls_enter
                                    && mapSeen_bytes[3] != 0
                                    && !splits_completed.map_seen[3]
                                {
                                    splits_completed.map_seen[3] = true;
                                    split();
                                } else if settings.bone_beach_enter
                                    && mapSeen_bytes[4] != 0
                                    && !splits_completed.map_seen[4]
                                {
                                    splits_completed.map_seen[4] = true;
                                    split();
                                } else if settings.mourners_mile_enter
                                    && mapSeen_bytes[5] != 0
                                    && !splits_completed.map_seen[5]
                                {
                                    splits_completed.map_seen[5] = true;
                                    split();
                                } else if settings.queensbury_crypt_enter
                                    && mapSeen_bytes[6] != 0
                                    && !splits_completed.map_seen[6]
                                {
                                    splits_completed.map_seen[6] = true;
                                    split();
                                } else if settings.eastern_heath_enter
                                    && mapSeen_bytes[7] != 0
                                    && !splits_completed.map_seen[7]
                                {
                                    splits_completed.map_seen[7] = true;
                                    split();
                                } else if settings.coltrane_peak_enter
                                    && mapSeen_bytes[8] != 0
                                    && !splits_completed.map_seen[8]
                                {
                                    splits_completed.map_seen[8] = true;
                                    split();
                                } else if settings.loners_landing_enter
                                    && mapSeen_bytes[9] != 0
                                    && !splits_completed.map_seen[9]
                                {
                                    splits_completed.map_seen[9] = true;
                                    split();
                                } else if settings.radiant_manor_foyer_enter
                                    && mapSeen_bytes[10] != 0
                                    && !splits_completed.map_seen[10]
                                {
                                    splits_completed.map_seen[10] = true;
                                    split();
                                } else if settings.radiant_manor_enter
                                    && mapSeen_bytes[11] != 0
                                    && !splits_completed.map_seen[11]
                                {
                                    splits_completed.map_seen[11] = true;
                                    split();
                                } else if settings.ossex_enter
                                    && mapSeen_bytes[12] != 0
                                    && !splits_completed.map_seen[12]
                                {
                                    splits_completed.map_seen[12] = true;
                                    split();
                                } else if settings.kindlewood_enter
                                    && mapSeen_bytes[13] != 0
                                    && !splits_completed.map_seen[13]
                                {
                                    splits_completed.map_seen[13] = true;
                                    split();
                                } else if settings.septemburg_enter
                                    && mapSeen_bytes[14] != 0
                                    && !splits_completed.map_seen[14]
                                {
                                    splits_completed.map_seen[14] = true;
                                    split();
                                } else if settings.southern_outskirts_enter
                                    && mapSeen_bytes[15] != 0
                                    && !splits_completed.map_seen[15]
                                {
                                    splits_completed.map_seen[15] = true;
                                    split();
                                } else if settings.western_wilds_enter
                                    && mapSeen_bytes[16] != 0
                                    && !splits_completed.map_seen[16]
                                {
                                    splits_completed.map_seen[16] = true;
                                    split();
                                }

                                // generators
                                if settings.queensbury_crypt
                                    && (generator & 0x02) != 0
                                    && (splits_completed.generators & 0x02) == 0
                                {
                                    splits_completed.generators |= 0x02;
                                    split();
                                } else if settings.noxs_bayou
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
                                // game end
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

fn reset_all(splits_completed: &mut SplitsCompleted) {
    splits_completed.reset();
    reset();
    pause_game_time();
}
