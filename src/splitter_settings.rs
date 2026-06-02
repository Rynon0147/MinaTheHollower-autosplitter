use asr::settings::gui::{set_tooltip, Title};
use asr::settings::Gui;

pub fn set_tooltips() {
    {
        /*
        set_tooltip(
            "show_completion",
            r##"Sets the variable "Completion". Can be displayed by LiveSplit as text."##,
        );
        */
    }
}

#[derive(Gui)]
pub struct Settings {
    #[heading_level = 0]
    mina_the_hollower: Title,
    #[heading_level = 1]
    bosses: Title,
    #[default = true]
    pub lionel_two: bool,

}
