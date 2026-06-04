use asr::{
    signature::Signature,
    Address,
    Process,
};
use asr::{
    print_message,
    timer::{
        set_variable, 
        set_variable_float, 
        set_variable_int, 
    },
};


pub fn get_offsets(process: &Process, process_name: &str) -> Option<Offsets> {
    let mut module_range = process.get_module_range(process_name).ok()?;
    set_variable("module_address", format!("{:X}", module_range.0.value()).as_str());
    set_variable("module_size", format!("{:X}", module_range.1).as_str());
    // hacked together garbage... gotta fix every patch - TODO find a real solution later
    if process_name == "MinaTheHollower" {
        module_range.1 = module_range.1 + 0x420F000; // linux is a bitch, do it right
    }
    //SAVEMANAGERHERE | Thanks Shane <3
    const SAVEMANAGER_SIG: Signature<15> = Signature::new("53 41 56 45 4D 41 4E 41 47 45 52 48 45 52 45"); 
    let savemanger_address = SAVEMANAGER_SIG.scan_process_range(process, module_range)?;
    Some(Offsets{
        savemanager: savemanger_address,
        fPlayTime: [0x18, 0x8],
        fPlayTimeCleared: [0x18, 0x10],
        fPlayTimeTotal: [0x18, 0x18],
    })
}

pub(crate) struct Offsets {
    pub savemanager: Address,
    pub fPlayTime: [u64; 2],
    pub fPlayTimeCleared: [u64; 2],
    pub fPlayTimeTotal: [u64; 2],
}