#![allow(non_snake_case)]

use asr::{
    future::{next_tick, retry},
    print_message,
    settings::Gui,
    time::Duration,
    timer::{
        pause_game_time, reset, set_game_time, set_variable, set_variable_float, set_variable_int,
        split, start, state, TimerState,
    },
    watcher::Watcher,
    PointerSize::Bit64,
    Process,
};
mod offsets;
mod splitter_settings;
use crate::offsets::get_offsets;

// track which splits have already been completed so that they only trigger once
struct SplitsCompleted {
    generators: u8,
    bosses_defeated: u32,
    game_cleared: bool,
    map_seen: [bool; 17],
    trinkets_seen: [bool; 60],
}

impl SplitsCompleted {
    fn new() -> Self {
        SplitsCompleted {
            generators: 0x00,
            bosses_defeated: 0x00,
            game_cleared: false,
            map_seen: [false; 17],
            trinkets_seen: [false; 60],
        }
    }

    fn reset(&mut self) {
        self.generators = 0x00;
        self.bosses_defeated = 0x00;
        self.game_cleared = false;
        self.map_seen = [false; 17];
        self.trinkets_seen = [false; 60];
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
                return;
            }
        }
        set_variable("Platform", platform);

        // Game Timer
        let mut watch_fPlayTime: Watcher<f64> = Watcher::new();
        watch_fPlayTime.update_infallible(-1.0f64);

        let mut watch_roomLastPosx: Watcher<f32> = Watcher::new();
        watch_roomLastPosx.update_infallible(0.0f32);

        let mut watch_roomLastPosy: Watcher<f32> = Watcher::new();
        watch_roomLastPosy.update_infallible(0.0f32);

        // state watch
        /*
        let mut watch_sCheckpointGamestate: Watcher<u32> = Watcher::new();
        watch_sCheckpointGamestate.update_infallible(0u32);
        */

        process
            .until_closes(async {
                print_message("Process found.");

                let mut splits_completed = SplitsCompleted::new();
                let mut room_timer_initialized = false;
                let mut last_room_time = 0.0f64;
                let mut prev_timer_state = TimerState::NotRunning;
                let offset_arrays = retry(|| get_offsets(&process, process_name)).await;

                print_message("Starting Loop.");
                loop {
                    settings.update();
                    let current_timer_state = state();

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
                    }

                    if let Ok(time) = process.read_pointer_path::<f64>(
                        offset_arrays.savemanager,
                        Bit64,
                        &offset_arrays.fPlayTimeTotal,
                    ) {
                        set_variable_float("fPlayTimeTotal", time);
                        //set_game_time(Duration::seconds_f64(time));
                    }

                    // last room positions
                    if let Ok(roomLastPosx) = process.read_pointer_path::<f32>(
                        offset_arrays.savemanager,
                        Bit64,
                        &offset_arrays.roomLastPosx,
                    ) {
                        watch_roomLastPosx.update_infallible(roomLastPosx);
                        set_variable_float("roomLastPosx", roomLastPosx);
                    }
                    if let Ok(roomLastPosy) = process.read_pointer_path::<f32>(
                        offset_arrays.savemanager,
                        Bit64,
                        &offset_arrays.roomLastPosy,
                    ) {
                        watch_roomLastPosy.update_infallible(roomLastPosy);
                        set_variable_float("roomLastPosy", roomLastPosy);
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

                    let mut bossDefeated: u32 = 0;
                    if let Ok(b) = process.read_pointer_path::<u32>(
                        offset_arrays.savemanager,
                        Bit64,
                        &offset_arrays.bossDefeated,
                    ) {
                        bossDefeated = b;
                        set_variable_int("bossDefeated", bossDefeated);
                        set_variable("bossDefeated", format!("{:034b}", bossDefeated).as_str());
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

                    let mut trinkets_bytes = [0u8; 60];
                    for i in 0..60u64 {
                        if let Ok(byte) = process.read_pointer_path::<u8>(
                            offset_arrays.savemanager,
                            Bit64,
                            &[offset_arrays.trinkets[0], offset_arrays.trinkets[1] + i * 4],
                        ) {
                            trinkets_bytes[i as usize] = byte;
                        }
                    }

                    // other custom variables
                    if let Ok(deaths) = process.read_pointer_path::<u8>(
                        offset_arrays.savemanager,
                        Bit64,
                        &offset_arrays.sDeathCount,
                    ) {
                        set_variable_int("Death Count", deaths);
                    }

                    // room timer calculation
                    if let Some(roomLastPosx) = &watch_roomLastPosx.pair {
                        if let Some(roomLastPosy) = &watch_roomLastPosy.pair {
                            if let Some(fPlayTime) = &watch_fPlayTime.pair {
                                if fPlayTime.current == 0.0 {
                                    room_timer_initialized = false;
                                    set_variable("Room Time", format!("{:.2}", 0.00).as_str());
                                } else if fPlayTime.current > 0.0 && !room_timer_initialized {
                                    last_room_time = fPlayTime.current;
                                    room_timer_initialized = true;
                                    set_variable("Room Time", format!("{:.2}", 0.00).as_str());
                                } else if fPlayTime.current > 0.0
                                    && room_timer_initialized
                                    && roomLastPosx.current != roomLastPosx.old
                                    && roomLastPosy.current != roomLastPosy.old
                                {
                                    let time_diff = fPlayTime.current - last_room_time;
                                    set_variable("Room Time", format!("{:.2}", time_diff).as_str());
                                    last_room_time = fPlayTime.current;
                                }
                            }
                        }
                    }

                    /*
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
                    */

                    // detect if the timer was started, to handle manual timer start as well
                    if prev_timer_state == TimerState::NotRunning
                        && matches!(
                            current_timer_state,
                            TimerState::Running | TimerState::Paused
                        )
                    {
                        splits_completed.reset();
                        pause_game_time();
                    }
                    prev_timer_state = current_timer_state;

                    match state() {
                        TimerState::NotRunning => {
                            // start timer
                            if let Some(fPlayTime) = &watch_fPlayTime.pair {
                                if settings.auto_reset_start
                                    && fPlayTime.old == 0f64
                                    && fPlayTime.current > 0f64
                                    && fPlayTime.current < 1f64
                                {
                                    fresh_start(&mut splits_completed);
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
                                }
                                if fPlayTime.current > 0f64 {
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

                            // bosses
                            if settings.thorne_1_defeated
                                && (bossDefeated & 0x01) != 0
                                && (splits_completed.bosses_defeated & 0x01) == 0
                            {
                                splits_completed.bosses_defeated |= 0x01;
                                split();
                            } else if settings.the_duchess_defeated
                                && (bossDefeated & 0x02) != 0
                                && (splits_completed.bosses_defeated & 0x02) == 0
                            {
                                splits_completed.bosses_defeated |= 0x02;
                                split();
                            } else if settings.noxs_beast_defeated
                                && (bossDefeated & 0x04) != 0
                                && (splits_completed.bosses_defeated & 0x04) == 0
                            {
                                splits_completed.bosses_defeated |= 0x04;
                                split();
                            } else if settings.the_carving_man_defeated
                                && (bossDefeated & 0x08) != 0
                                && (splits_completed.bosses_defeated & 0x08) == 0
                            {
                                splits_completed.bosses_defeated |= 0x08;
                                split();
                            } else if settings.mined_mind_defeated
                                && (bossDefeated & 0x10) != 0
                                && (splits_completed.bosses_defeated & 0x10) == 0
                            {
                                splits_completed.bosses_defeated |= 0x10;
                                split();
                            } else if settings.locomotress_agnes_defeated
                                && (bossDefeated & 0x20) != 0
                                && (splits_completed.bosses_defeated & 0x20) == 0
                            {
                                splits_completed.bosses_defeated |= 0x20;
                                split();
                            } else if settings.the_congealed_defeated
                                && (bossDefeated & 0x40) != 0
                                && (splits_completed.bosses_defeated & 0x40) == 0
                            {
                                splits_completed.bosses_defeated |= 0x40;
                                split();
                            } else if settings.baron_lionel_defeated
                                && (bossDefeated & 0x80) != 0
                                && (splits_completed.bosses_defeated & 0x80) == 0
                            {
                                splits_completed.bosses_defeated |= 0x80;
                                split();
                            } else if settings.radiant_lionel_defeated
                                && (bossDefeated & 0x0100) != 0
                                && (splits_completed.bosses_defeated & 0x0100) == 0
                            {
                                splits_completed.bosses_defeated |= 0x0100;
                                split();
                            } else if settings.thorne_2_defeated
                                && (bossDefeated & 0x0200) != 0
                                && (splits_completed.bosses_defeated & 0x0200) == 0
                            {
                                splits_completed.bosses_defeated |= 0x0200;
                                split();
                            } else if settings.nether_kraken_defeated
                                && (bossDefeated & 0x0400) != 0
                                && (splits_completed.bosses_defeated & 0x0400) == 0
                            {
                                splits_completed.bosses_defeated |= 0x0400;
                                split();
                            } else if settings.madd_house_defeated
                                && (bossDefeated & 0x1000) != 0
                                && (splits_completed.bosses_defeated & 0x1000) == 0
                            {
                                splits_completed.bosses_defeated |= 0x1000;
                                split();
                            } else if settings.major_miner_defeated
                                && (bossDefeated & 0x2000) != 0
                                && (splits_completed.bosses_defeated & 0x2000) == 0
                            {
                                splits_completed.bosses_defeated |= 0x2000;
                                split();
                            } else if settings.lumenarks_defeated
                                && (bossDefeated & 0x4000) != 0
                                && (splits_completed.bosses_defeated & 0x4000) == 0
                            {
                                splits_completed.bosses_defeated |= 0x4000;
                                split();
                            } else if settings.frozen_horror_defeated
                                && (bossDefeated & 0x8000) != 0
                                && (splits_completed.bosses_defeated & 0x8000) == 0
                            {
                                splits_completed.bosses_defeated |= 0x8000;
                                split();
                            } else if settings.dugin_defeated
                                && (bossDefeated & 0x010000) != 0
                                && (splits_completed.bosses_defeated & 0x010000) == 0
                            {
                                splits_completed.bosses_defeated |= 0x010000;
                                split();
                            } else if settings.mock_moon_defeated
                                && (bossDefeated & 0x020000) != 0
                                && (splits_completed.bosses_defeated & 0x020000) == 0
                            {
                                splits_completed.bosses_defeated |= 0x020000;
                                split();
                            } else if settings.maxi_defeated
                                && (bossDefeated & 0x040000) != 0
                                && (splits_completed.bosses_defeated & 0x040000) == 0
                            {
                                splits_completed.bosses_defeated |= 0x040000;
                                split();
                            } else if settings.furgus_the_faithful_defeated
                                && (bossDefeated & 0x080000) != 0
                                && (splits_completed.bosses_defeated & 0x080000) == 0
                            {
                                splits_completed.bosses_defeated |= 0x080000;
                                split();
                            } else if settings.wonder_willis_defeated
                                && (bossDefeated & 0x100000) != 0
                                && (splits_completed.bosses_defeated & 0x100000) == 0
                            {
                                splits_completed.bosses_defeated |= 0x100000;
                                split();
                            } else if settings.armand_defeated
                                && (bossDefeated & 0x200000) != 0
                                && (splits_completed.bosses_defeated & 0x200000) == 0
                            {
                                splits_completed.bosses_defeated |= 0x200000;
                                split();
                            } else if settings.evra_defeated
                                && (bossDefeated & 0x400000) != 0
                                && (splits_completed.bosses_defeated & 0x400000) == 0
                            {
                                splits_completed.bosses_defeated |= 0x400000;
                                split();
                            } else if settings.thorne_3_defeated
                                && (bossDefeated & 0x800000) != 0
                                && (splits_completed.bosses_defeated & 0x800000) == 0
                            {
                                splits_completed.bosses_defeated |= 0x800000;
                                split();
                            } else if settings.dark_deluxy_defeated
                                && (bossDefeated & 0x01000000) != 0
                                && (splits_completed.bosses_defeated & 0x01000000) == 0
                            {
                                splits_completed.bosses_defeated |= 0x01000000;
                                split();
                            } else if settings.mirren_defeated
                                && (bossDefeated & 0x02000000) != 0
                                && (splits_completed.bosses_defeated & 0x02000000) == 0
                            {
                                splits_completed.bosses_defeated |= 0x02000000;
                                split();
                            } else if settings.thalassion_defeated
                                && (bossDefeated & 0x04000000) != 0
                                && (splits_completed.bosses_defeated & 0x04000000) == 0
                            {
                                splits_completed.bosses_defeated |= 0x04000000;
                                split();
                            } else if settings.hulk_trooper_defeated
                                && (bossDefeated & 0x08000000) != 0
                                && (splits_completed.bosses_defeated & 0x08000000) == 0
                            {
                                splits_completed.bosses_defeated |= 0x08000000;
                                split();
                            } else if settings.midden_defeated
                                && (bossDefeated & 0x10000000) != 0
                                && (splits_completed.bosses_defeated & 0x10000000) == 0
                            {
                                splits_completed.bosses_defeated |= 0x10000000;
                                split();
                            }

                            // trinkets
                            if settings.lace_glove
                                && trinkets_bytes[0] != 0
                                && !splits_completed.trinkets_seen[0]
                            {
                                splits_completed.trinkets_seen[0] = true;
                                split();
                            } else if settings.twill_weave
                                && trinkets_bytes[1] != 0
                                && !splits_completed.trinkets_seen[1]
                            {
                                splits_completed.trinkets_seen[1] = true;
                                split();
                            } else if settings.smelling_salts
                                && trinkets_bytes[2] != 0
                                && !splits_completed.trinkets_seen[2]
                            {
                                splits_completed.trinkets_seen[2] = true;
                                split();
                            } else if settings.brisk_brew
                                && trinkets_bytes[3] != 0
                                && !splits_completed.trinkets_seen[3]
                            {
                                splits_completed.trinkets_seen[3] = true;
                                split();
                            } else if settings.seismic_belt
                                && trinkets_bytes[4] != 0
                                && !splits_completed.trinkets_seen[4]
                            {
                                splits_completed.trinkets_seen[4] = true;
                                split();
                            } else if settings.plasma_funnel
                                && trinkets_bytes[5] != 0
                                && !splits_completed.trinkets_seen[5]
                            {
                                splits_completed.trinkets_seen[5] = true;
                                split();
                            } else if settings.deboning_wand
                                && trinkets_bytes[6] != 0
                                && !splits_completed.trinkets_seen[6]
                            {
                                splits_completed.trinkets_seen[6] = true;
                                split();
                            } else if settings.steady_soles
                                && trinkets_bytes[7] != 0
                                && !splits_completed.trinkets_seen[7]
                            {
                                splits_completed.trinkets_seen[7] = true;
                                split();
                            } else if settings.valor_medallion
                                && trinkets_bytes[8] != 0
                                && !splits_completed.trinkets_seen[8]
                            {
                                splits_completed.trinkets_seen[8] = true;
                                split();
                            } else if settings.bell_of_grace
                                && trinkets_bytes[9] != 0
                                && !splits_completed.trinkets_seen[9]
                            {
                                splits_completed.trinkets_seen[9] = true;
                                split();
                            } else if settings.willow_the_wisp
                                && trinkets_bytes[10] != 0
                                && !splits_completed.trinkets_seen[10]
                            {
                                splits_completed.trinkets_seen[10] = true;
                                split();
                            } else if settings.helio_the_wisp
                                && trinkets_bytes[11] != 0
                                && !splits_completed.trinkets_seen[11]
                            {
                                splits_completed.trinkets_seen[11] = true;
                                split();
                            } else if settings.keri_the_wisp
                                && trinkets_bytes[12] != 0
                                && !splits_completed.trinkets_seen[12]
                            {
                                splits_completed.trinkets_seen[12] = true;
                                split();
                            } else if settings.windfall_charm
                                && trinkets_bytes[13] != 0
                                && !splits_completed.trinkets_seen[13]
                            {
                                splits_completed.trinkets_seen[13] = true;
                                split();
                            } else if settings.chain_capacitor
                                && trinkets_bytes[14] != 0
                                && !splits_completed.trinkets_seen[14]
                            {
                                splits_completed.trinkets_seen[14] = true;
                                split();
                            } else if settings.spike_spurs
                                && trinkets_bytes[15] != 0
                                && !splits_completed.trinkets_seen[15]
                            {
                                splits_completed.trinkets_seen[15] = true;
                                split();
                            } else if settings.desperation_bonnet
                                && trinkets_bytes[16] != 0
                                && !splits_completed.trinkets_seen[16]
                            {
                                splits_completed.trinkets_seen[16] = true;
                                split();
                            } else if settings.stolenoid
                                && trinkets_bytes[17] != 0
                                && !splits_completed.trinkets_seen[17]
                            {
                                splits_completed.trinkets_seen[17] = true;
                                split();
                            } else if settings.fly_bait
                                && trinkets_bytes[18] != 0
                                && !splits_completed.trinkets_seen[18]
                            {
                                splits_completed.trinkets_seen[18] = true;
                                split();
                            } else if settings.proto_spark
                                && trinkets_bytes[19] != 0
                                && !splits_completed.trinkets_seen[19]
                            {
                                splits_completed.trinkets_seen[19] = true;
                                split();
                            } else if settings.primed_vial_pouch
                                && trinkets_bytes[20] != 0
                                && !splits_completed.trinkets_seen[20]
                            {
                                splits_completed.trinkets_seen[20] = true;
                                split();
                            } else if settings.flame_guard
                                && trinkets_bytes[21] != 0
                                && !splits_completed.trinkets_seen[21]
                            {
                                splits_completed.trinkets_seen[21] = true;
                                split();
                            } else if settings.spark_catcher
                                && trinkets_bytes[22] != 0
                                && !splits_completed.trinkets_seen[22]
                            {
                                splits_completed.trinkets_seen[22] = true;
                                split();
                            } else if settings.evasion_powder
                                && trinkets_bytes[23] != 0
                                && !splits_completed.trinkets_seen[23]
                            {
                                splits_completed.trinkets_seen[23] = true;
                                split();
                            } else if settings.vascular_syrup
                                && trinkets_bytes[24] != 0
                                && !splits_completed.trinkets_seen[24]
                            {
                                splits_completed.trinkets_seen[24] = true;
                                split();
                            } else if settings.pit_preserver
                                && trinkets_bytes[25] != 0
                                && !splits_completed.trinkets_seen[25]
                            {
                                splits_completed.trinkets_seen[25] = true;
                                split();
                            } else if settings.iron_lung
                                && trinkets_bytes[26] != 0
                                && !splits_completed.trinkets_seen[26]
                            {
                                splits_completed.trinkets_seen[26] = true;
                                split();
                            } else if settings.tumbling_tutu
                                && trinkets_bytes[27] != 0
                                && !splits_completed.trinkets_seen[27]
                            {
                                splits_completed.trinkets_seen[27] = true;
                                split();
                            } else if settings.plasma_jug
                                && trinkets_bytes[28] != 0
                                && !splits_completed.trinkets_seen[28]
                            {
                                splits_completed.trinkets_seen[28] = true;
                                split();
                            } else if settings.uranium_bracelet
                                && trinkets_bytes[29] != 0
                                && !splits_completed.trinkets_seen[29]
                            {
                                splits_completed.trinkets_seen[29] = true;
                                split();
                            } else if settings.bubble_ring
                                && trinkets_bytes[30] != 0
                                && !splits_completed.trinkets_seen[30]
                            {
                                splits_completed.trinkets_seen[30] = true;
                                split();
                            } else if settings.shock_flint
                                && trinkets_bytes[31] != 0
                                && !splits_completed.trinkets_seen[31]
                            {
                                splits_completed.trinkets_seen[31] = true;
                                split();
                            } else if settings.intravenous_vial
                                && trinkets_bytes[32] != 0
                                && !splits_completed.trinkets_seen[32]
                            {
                                splits_completed.trinkets_seen[32] = true;
                                split();
                            } else if settings.pneumatic_armlet
                                && trinkets_bytes[33] != 0
                                && !splits_completed.trinkets_seen[33]
                            {
                                splits_completed.trinkets_seen[33] = true;
                                split();
                            } else if settings.starving_beastium
                                && trinkets_bytes[34] != 0
                                && !splits_completed.trinkets_seen[34]
                            {
                                splits_completed.trinkets_seen[34] = true;
                                split();
                            } else if settings.draining_beastium
                                && trinkets_bytes[35] != 0
                                && !splits_completed.trinkets_seen[35]
                            {
                                splits_completed.trinkets_seen[35] = true;
                                split();
                            } else if settings.reckless_beastium
                                && trinkets_bytes[36] != 0
                                && !splits_completed.trinkets_seen[36]
                            {
                                splits_completed.trinkets_seen[36] = true;
                                split();
                            } else if settings.volatile_beastium
                                && trinkets_bytes[37] != 0
                                && !splits_completed.trinkets_seen[37]
                            {
                                splits_completed.trinkets_seen[37] = true;
                                split();
                            } else if settings.burning_beastium
                                && trinkets_bytes[38] != 0
                                && !splits_completed.trinkets_seen[38]
                            {
                                splits_completed.trinkets_seen[38] = true;
                                split();
                            } else if settings.warding_beastium
                                && trinkets_bytes[39] != 0
                                && !splits_completed.trinkets_seen[39]
                            {
                                splits_completed.trinkets_seen[39] = true;
                                split();
                            } else if settings.dummy_cache
                                && trinkets_bytes[40] != 0
                                && !splits_completed.trinkets_seen[40]
                            {
                                splits_completed.trinkets_seen[40] = true;
                                split();
                            } else if settings.blinking_glass
                                && trinkets_bytes[41] != 0
                                && !splits_completed.trinkets_seen[41]
                            {
                                splits_completed.trinkets_seen[41] = true;
                                split();
                            } else if settings.watchful_eye
                                && trinkets_bytes[42] != 0
                                && !splits_completed.trinkets_seen[42]
                            {
                                splits_completed.trinkets_seen[42] = true;
                                split();
                            } else if settings.bridge_weaver
                                && trinkets_bytes[43] != 0
                                && !splits_completed.trinkets_seen[43]
                            {
                                splits_completed.trinkets_seen[43] = true;
                                split();
                            } else if settings.vial_salvo
                                && trinkets_bytes[44] != 0
                                && !splits_completed.trinkets_seen[44]
                            {
                                splits_completed.trinkets_seen[44] = true;
                                split();
                            } else if settings.dodging_pendulum
                                && trinkets_bytes[45] != 0
                                && !splits_completed.trinkets_seen[45]
                            {
                                splits_completed.trinkets_seen[45] = true;
                                split();
                            } else if settings.spring_heels
                                && trinkets_bytes[46] != 0
                                && !splits_completed.trinkets_seen[46]
                            {
                                splits_completed.trinkets_seen[46] = true;
                                split();
                            } else if settings.wallowers_gauntlets
                                && trinkets_bytes[47] != 0
                                && !splits_completed.trinkets_seen[47]
                            {
                                splits_completed.trinkets_seen[47] = true;
                                split();
                            } else if settings.oozing_organ
                                && trinkets_bytes[48] != 0
                                && !splits_completed.trinkets_seen[48]
                            {
                                splits_completed.trinkets_seen[48] = true;
                                split();
                            } else if settings.voltaic_guard
                                && trinkets_bytes[49] != 0
                                && !splits_completed.trinkets_seen[49]
                            {
                                splits_completed.trinkets_seen[49] = true;
                                split();
                            } else if settings.repulsing_root
                                && trinkets_bytes[50] != 0
                                && !splits_completed.trinkets_seen[50]
                            {
                                splits_completed.trinkets_seen[50] = true;
                                split();
                            } else if settings.lightning_grip
                                && trinkets_bytes[51] != 0
                                && !splits_completed.trinkets_seen[51]
                            {
                                splits_completed.trinkets_seen[51] = true;
                                split();
                            } else if settings.dead_leaf
                                && trinkets_bytes[52] != 0
                                && !splits_completed.trinkets_seen[52]
                            {
                                splits_completed.trinkets_seen[52] = true;
                                split();
                            } else if settings.niter_belt
                                && trinkets_bytes[53] != 0
                                && !splits_completed.trinkets_seen[53]
                            {
                                splits_completed.trinkets_seen[53] = true;
                                split();
                            } else if settings.bellows_bustle
                                && trinkets_bytes[54] != 0
                                && !splits_completed.trinkets_seen[54]
                            {
                                splits_completed.trinkets_seen[54] = true;
                                split();
                            } else if settings.tunneling_codex
                                && trinkets_bytes[55] != 0
                                && !splits_completed.trinkets_seen[55]
                            {
                                splits_completed.trinkets_seen[55] = true;
                                split();
                            } else if settings.joule_syringe
                                && trinkets_bytes[56] != 0
                                && !splits_completed.trinkets_seen[56]
                            {
                                splits_completed.trinkets_seen[56] = true;
                                split();
                            } else if settings.polyp_lamp
                                && trinkets_bytes[57] != 0
                                && !splits_completed.trinkets_seen[57]
                            {
                                splits_completed.trinkets_seen[57] = true;
                                split();
                            } else if settings.thermal_pack
                                && trinkets_bytes[58] != 0
                                && !splits_completed.trinkets_seen[58]
                            {
                                splits_completed.trinkets_seen[58] = true;
                                split();
                            } else if settings.counter_vial
                                && trinkets_bytes[59] != 0
                                && !splits_completed.trinkets_seen[59]
                            {
                                splits_completed.trinkets_seen[59] = true;
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
            })
            .await;
    }
}

fn reset_all(splits_completed: &mut SplitsCompleted) {
    splits_completed.reset();
    reset();
    pause_game_time();
    start();
}

fn fresh_start(splits_completed: &mut SplitsCompleted) {
    splits_completed.reset();
    pause_game_time();
    start();
}
