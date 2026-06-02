use asr::{
    future::next_tick, 
    settings::Gui, 
    Process, 
    watcher::Watcher,
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

asr::async_main!(stable);

async fn main() {
    // TODO: Set up some general state and settings.
    let mut settings = splitter_settings::Settings::register();

    // Base Settings
    let plattform = "linux";
    let process_name: &str;
    match plattform {
        "linux" => {
            process_name = "MinaTheHollower";
        }
        "windows" => {
            process_name = "MinaTheHollower.exe";
        }
        _ => {
            asr::print_message("invalid plattform");
            process_name = "";
        }
    }

    
    asr::print_message("Setup done. Waiting for Process.");

    loop {
        let process = Process::wait_attach(process_name).await;
        process
            .until_closes(async {
                asr::print_message("Process found.");

                if let Ok(base_address) = process.get_module_address(process_name){
                    set_variable_int("debug", address.value());
                }

                // Game Timer (seconds)
                let mut watch_fPlayTimeCleared: Watcher<f64> = Watcher::new();
                watch_fPlayTimeCleared.update_infallible(0f64);

                asr::print_message("Starting Loop.");
                loop {
                    settings.update();
                    
                    // TODO: Do something on every tick.

                    /*
                    // Game Timer
                    if let Ok(time) = process.read_pointer_path::<f32>(
                        module.g_world(),
                        Bit64,
                        &offsets.fPlayTimeCleared,
                    ) {
                        if time > 0f32 {
                            watch_total_fPlayTimeCleared.update_infallible(time);
                            set_variable_float("fPlayTimeCleared", time);
                            //set_game_time(Duration::seconds_f32(time/100.0));
                        }
                    }
                    */
                    next_tick().await;
                }
            })
            .await;
    }
}
