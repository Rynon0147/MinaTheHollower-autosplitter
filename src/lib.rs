#![allow(non_snake_case)]
#![allow(unused_imports)] //remove

use asr::{
    future::{next_tick, retry}, 
    settings::Gui, 
    Process, 
    time::Duration,
    watcher::Watcher,
    PointerSize::Bit64,
    Address,
    signature::Signature,
    print_message,
    timer::{
        reset, 
        set_game_time, 
        set_variable, 
        set_variable_float, 
        set_variable_int, 
        split, 
        start, 
        state, 
        TimerState,
        pause_game_time,
    },
};
mod splitter_settings;
mod offsets;
use crate::offsets::get_offsets;

asr::async_main!(stable);


const PROCESS_NAMES: &[&str] = &["MinaTheHollower.exe", "MinaTheHollower"];

async fn main() {

    let mut settings = splitter_settings::Settings::register();

    print_message("Setup done. Waiting for Process.");

    loop {
        let found: (&str, Process) = retry(|| PROCESS_NAMES.iter().find_map(|&name| Process::attach(name).map(|proc| (name, proc)))).await;
        let process_name = found.0;
        let process = found.1;//Process::wait_attach(process_name).await;  

        let platform : &str;
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
        watch_fPlayTime.update_infallible(0f64);

        // state watch
        let mut watch_sCheckpointGamestate: Watcher<u32> = Watcher::new();
        watch_sCheckpointGamestate.update_infallible(0u32);

        process
            .until_closes(async {
                print_message("Process found.");
                
                // Game Timer (seconds)
                if let Some(offset_arrays) = get_offsets(&process, process_name){

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
                        
                        match state(){
                            TimerState::NotRunning => {
                            //start timer
                                if let Some(fPlayTime) = &watch_fPlayTime.pair {
                                    /*
                                    if fPlayTime.changed() && fPlayTime.old == 0f64 {
                                        reset_all(/*&mut split_states*/);
                                    }
                                    */
                                }
                            }
                            TimerState::Paused | TimerState::Running => {

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


fn reset_all(/*split_states: &mut [bool;32]*/){
    //split_states.fill(false);
 
    reset();
    start();
    pause_game_time();
    
}