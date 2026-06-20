use asr::settings::gui::{set_tooltip, Title};
use asr::settings::Gui;

/*
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
*/

#[derive(Gui)]
pub struct Settings {
    #[heading_level = 0]
    mina_the_hollower: Title,

    #[heading_level = 1]
    timer: Title,
    /// Auto-Reset/Start (on Profile Select)
    ///
    /// Automatically reset and start the timer. This will happen when selecting a new profile.
    #[default = true]
    pub auto_reset_start: bool,

    #[heading_level = 1]
    generators: Title,
    /// Queensbury Crypt
    ///
    /// Split on the Queensbury Crypt generator
    #[default = true]
    pub queensbury_crypt: bool,
    /// Nox's Bayou
    ///
    /// Split on the Nox's Bayou generator
    #[default = true]
    pub noxs_bayou: bool,
    /// Septemburg
    ///
    /// Split on the Septemburg generator
    #[default = true]
    pub septemburg: bool,
    /// Bone Beach
    ///
    /// Split on the Bone Beach generator
    #[default = true]
    pub bone_beach: bool,
    /// Coltrane Peak
    ///
    /// Split on the Coltrane Peak generator
    #[default = true]
    pub coltrane_peak: bool,
    /// Astral Orrery
    ///
    /// Split on the Astral Orrery generator
    #[default = true]
    pub astral_orrery: bool,
    /// Radiant Manor
    ///
    /// Split on the Radiant Manor generator
    #[default = true]
    pub game_cleared: bool,
    #[heading_level = 1]
    bosses_defeated: Title,
    #[heading_level = 2]
    required_bosses: Title,
    /// Nether Kraken
    ///
    /// Split after defeating Nether Kraken
    #[default = false]
    pub nether_kraken_defeated: bool,
    /// Hulk Trooper
    ///
    /// Split after defeating Hulk Trooper
    #[default = false]
    pub hulk_trooper_defeated: bool,
    /// Thorne 1
    ///
    /// Split after defeating Thorne 1
    #[default = false]
    pub thorne_1_defeated: bool,
    /// The Duchess
    ///
    /// Split after defeating The Duchess
    #[default = false]
    pub the_duchess_defeated: bool,
    /// Nox's Beast
    ///
    /// Split after defeating Nox's Beast
    #[default = false]
    pub noxs_beast_defeated: bool,
    /// Madd House
    ///
    /// Split after defeating Madd House
    #[default = false]
    pub madd_house_defeated: bool,
    /// The Carving Man
    ///
    /// Split after defeating The Carving Man
    #[default = false]
    pub the_carving_man_defeated: bool,
    /// Major Miner
    ///
    /// Split after defeating Major Miner
    #[default = false]
    pub major_miner_defeated: bool,
    /// Mined Mind
    ///
    /// Split after defeating Mined Mind
    #[default = false]
    pub mined_mind_defeated: bool,
    /// Thorne 2
    ///
    /// Split after defeating Thorne 2
    #[default = false]
    pub thorne_2_defeated: bool,
    /// Frozen Horror
    ///
    /// Split after defeating Frozen Horror
    #[default = false]
    pub frozen_horror_defeated: bool,
    /// Locomotress Agnes
    ///
    /// Split after defeating Locomotress Agnes
    #[default = false]
    pub locomotress_agnes_defeated: bool,
    /// Lumenarks
    ///
    /// Split after defeating Lumenarks
    #[default = false]
    pub lumenarks_defeated: bool,
    /// The Congealed
    ///
    /// Split after defeating The Congealed
    #[default = false]
    pub the_congealed_defeated: bool,
    /// Furgus The Faithful
    ///
    /// Split after defeating Furgus The Faithful
    #[default = false]
    pub furgus_the_faithful_defeated: bool,
    /// Baron Lionel
    ///
    /// Split after defeating Baron Lionel
    #[default = false]
    pub baron_lionel_defeated: bool,
    /// Radiant Lionel
    ///
    /// Split after defeating Radiant Lionel
    #[default = false]
    pub radiant_lionel_defeated: bool,
    #[heading_level = 2]
    optional_bosses: Title,
    /// Midden
    ///
    /// Split after defeating Midden
    #[default = false]
    pub midden_defeated: bool,
    /// Maxi
    ///
    /// Split after defeating Maxi
    #[default = false]
    pub maxi_defeated: bool,
    /// Mock Moon
    ///
    /// Split after defeating Mock Moon
    #[default = false]
    pub mock_moon_defeated: bool,
    /// Dugin?
    ///
    /// Split after defeating Dugin?
    #[default = false]
    pub dugin_defeated: bool,
    /// Armand
    ///
    /// Split after defeating Armand
    #[default = false]
    pub armand_defeated: bool,
    /// Dark Deluxy
    ///
    /// Split after defeating Dark Deluxy
    #[default = false]
    pub dark_deluxy_defeated: bool,
    /// Mirren
    ///
    /// Split after defeating Mirren
    #[default = false]
    pub mirren_defeated: bool,
    /// ??????
    ///
    /// Split after defeating ??????
    #[default = false]
    pub thorne_3_defeated: bool,
    /// Evra
    ///
    /// Split after defeating Evra
    #[default = false]
    pub evra_defeated: bool,
    /// Wonder Willis
    ///
    /// Split after defeating Wonder Willis
    #[default = false]
    pub wonder_willis_defeated: bool,
    /// Thalassion
    ///
    /// Split after defeating Thalassion
    #[default = false]
    pub thalassion_defeated: bool,
    #[heading_level = 1]
    enter_area: Title,
    /// Astral Orrery
    ///
    /// Split when entering Astral Orrery for the first time
    #[default = false]
    pub astral_orrery_enter: bool,
    /// Backwaters
    ///
    /// Split when entering Backwaters for the first time
    #[default = false]
    pub backwaters_enter: bool,
    /// Nox's Bayou
    ///
    /// Split when entering Nox's Bayou for the first time
    #[default = false]
    pub noxs_bayou_enter: bool,
    /// Sandfalls
    ///
    /// Split when entering Sandfalls for the first time
    #[default = false]
    pub sandfalls_enter: bool,
    /// Bone Beach
    ///
    /// Split when entering Bone Beach for the first time
    #[default = false]
    pub bone_beach_enter: bool,
    /// Mourner's Mile
    ///
    /// Split when entering Mourner's Mile for the first time
    #[default = false]
    pub mourners_mile_enter: bool,
    /// Queensbury Crypt
    ///
    /// Split when entering Queensbury Crypt for the first time
    #[default = false]
    pub queensbury_crypt_enter: bool,
    /// Eastern Heath: Grassland
    ///
    /// Split when entering Eastern Heath for the first time
    #[default = false]
    pub eastern_heath_enter: bool,
    /// Coltrane Peak
    ///
    /// Split when entering Coltrane Peak for the first time
    #[default = false]
    pub coltrane_peak_enter: bool,
    /// Loner's Landing
    ///
    /// Split when entering Loner's Landing for the first time
    #[default = false]
    pub loners_landing_enter: bool,
    /// Radiant Manor Foyer: Grand Hall
    ///
    /// Split when entering Radiant Manor Foyer for the first time
    #[default = false]
    pub radiant_manor_foyer_enter: bool,
    /// Radiant Manor
    ///
    /// Split when entering Radiant Manor for the first time
    #[default = false]
    pub radiant_manor_enter: bool,
    /// Ossex
    ///
    /// Split when entering Ossex for the first time
    #[default = false]
    pub ossex_enter: bool,
    /// Kindlewood
    ///
    /// Split when entering Kindlewood for the first time
    #[default = false]
    pub kindlewood_enter: bool,
    /// Septemburg
    ///
    /// Split when entering Septemburg for the first time
    #[default = false]
    pub septemburg_enter: bool,
    /// Southern Outskirts
    ///
    /// Split when entering Southern Outskirts for the first time
    #[default = false]
    pub southern_outskirts_enter: bool,
    /// Western Wilds
    ///
    /// Split when entering Western Wilds for the first time
    #[default = false]
    pub western_wilds_enter: bool,
    #[heading_level = 1]
    trinkets: Title,
    /// Lace Glove
    ///
    /// Split on collecting the Lace Glove trinket
    #[default = false]
    pub lace_glove: bool,
    /// Twill Weave
    ///
    /// Split on collecting the Twill Weave trinket
    #[default = false]
    pub twill_weave: bool,
    /// Smelling Salts
    ///
    /// Split on collecting the Smelling Salts trinket
    #[default = false]
    pub smelling_salts: bool,
    /// Brisk Brew
    ///
    /// Split on collecting the Brisk Brew trinket
    #[default = false]
    pub brisk_brew: bool,
    /// Seismic Belt
    ///
    /// Split on collecting the Seismic Belt trinket
    #[default = false]
    pub seismic_belt: bool,
    /// Plasma Funnel
    ///
    /// Split on collecting the Plasma Funnel trinket
    #[default = false]
    pub plasma_funnel: bool,
    /// Deboning Wand
    ///
    /// Split on collecting the Deboning Wand trinket
    #[default = false]
    pub deboning_wand: bool,
    /// Steady Soles
    ///
    /// Split on collecting the Steady Soles trinket
    #[default = false]
    pub steady_soles: bool,
    /// Valor Medallion
    ///
    /// Split on collecting the Valor Medallion trinket
    #[default = false]
    pub valor_medallion: bool,
    /// Bell of Grace
    ///
    /// Split on collecting the Bell of Grace trinket
    #[default = false]
    pub bell_of_grace: bool,
    /// Willow the Wisp
    ///
    /// Split on collecting the Willow the Wisp trinket
    #[default = false]
    pub willow_the_wisp: bool,
    /// Helio the Wisp
    ///
    /// Split on collecting the Helio the Wisp trinket
    #[default = false]
    pub helio_the_wisp: bool,
    /// Keri the Wisp
    ///
    /// Split on collecting the Keri the Wisp trinket
    #[default = false]
    pub keri_the_wisp: bool,
    /// Windfall Charm
    ///
    /// Split on collecting the Windfall Charm trinket
    #[default = false]
    pub windfall_charm: bool,
    /// Chain Capacitor
    ///
    /// Split on collecting the Chain Capacitor trinket
    #[default = false]
    pub chain_capacitor: bool,
    /// Spike Spurs
    ///
    /// Split on collecting the Spike Spurs trinket
    #[default = false]
    pub spike_spurs: bool,
    /// Desperation Bonnet
    ///
    /// Split on collecting the Desperation Bonnet trinket
    #[default = false]
    pub desperation_bonnet: bool,
    /// Stolenoid
    ///
    /// Split on collecting the Stolenoid trinket
    #[default = false]
    pub stolenoid: bool,
    /// Fly Bait
    ///
    /// Split on collecting the Fly Bait trinket
    #[default = false]
    pub fly_bait: bool,
    /// Proto Spark
    ///
    /// Split on collecting the Proto Spark trinket
    #[default = false]
    pub proto_spark: bool,
    /// Primed Vial Pouch
    ///
    /// Split on collecting the Primed Vial Pouch trinket
    #[default = false]
    pub primed_vial_pouch: bool,
    /// Flame Guard
    ///
    /// Split on collecting the Flame Guard trinket
    #[default = false]
    pub flame_guard: bool,
    /// Spark Catcher
    ///
    /// Split on collecting the Spark Catcher trinket
    #[default = false]
    pub spark_catcher: bool,
    /// Evasion Powder
    ///
    /// Split on collecting the Evasion Powder trinket
    #[default = false]
    pub evasion_powder: bool,
    /// Vascular Syrup
    ///
    /// Split on collecting the Vascular Syrup trinket
    #[default = false]
    pub vascular_syrup: bool,
    /// Pit Preserver
    ///
    /// Split on collecting the Pit Preserver trinket
    #[default = false]
    pub pit_preserver: bool,
    /// Iron Lung
    ///
    /// Split on collecting the Iron Lung trinket
    #[default = false]
    pub iron_lung: bool,
    /// Tumbling Tutu
    ///
    /// Split on collecting the Tumbling Tutu trinket
    #[default = false]
    pub tumbling_tutu: bool,
    /// Plasma Jug
    ///
    /// Split on collecting the Plasma Jug trinket
    #[default = false]
    pub plasma_jug: bool,
    /// Uranium Bracelet
    ///
    /// Split on collecting the Uranium Bracelet trinket
    #[default = false]
    pub uranium_bracelet: bool,
    /// Bubble Ring
    ///
    /// Split on collecting the Bubble Ring trinket
    #[default = false]
    pub bubble_ring: bool,
    /// Shock Flint
    ///
    /// Split on collecting the Shock Flint trinket
    #[default = false]
    pub shock_flint: bool,
    /// Intravenous Vial
    ///
    /// Split on collecting the Intravenous Vial trinket
    #[default = false]
    pub intravenous_vial: bool,
    /// Pneumatic Armlet
    ///
    /// Split on collecting the Pneumatic Armlet trinket
    #[default = false]
    pub pneumatic_armlet: bool,
    /// Starving Beastium
    ///
    /// Split on collecting the Starving Beastium trinket
    #[default = false]
    pub starving_beastium: bool,
    /// Draining Beastium
    ///
    /// Split on collecting the Draining Beastium trinket
    #[default = false]
    pub draining_beastium: bool,
    /// Reckless Beastium
    ///
    /// Split on collecting the Reckless Beastium trinket
    #[default = false]
    pub reckless_beastium: bool,
    /// Volatile Beastium
    ///
    /// Split on collecting the Volatile Beastium trinket
    #[default = false]
    pub volatile_beastium: bool,
    /// Burning Beastium
    ///
    /// Split on collecting the Burning Beastium trinket
    #[default = false]
    pub burning_beastium: bool,
    /// Warding Beastium
    ///
    /// Split on collecting the Warding Beastium trinket
    #[default = false]
    pub warding_beastium: bool,
    /// Dummy Cache
    ///
    /// Split on collecting the Dummy Cache trinket
    #[default = false]
    pub dummy_cache: bool,
    /// Blinking Glass
    ///
    /// Split on collecting the Blinking Glass trinket
    #[default = false]
    pub blinking_glass: bool,
    /// Watchful Eye
    ///
    /// Split on collecting the Watchful Eye trinket
    #[default = false]
    pub watchful_eye: bool,
    /// Bridge Weaver
    ///
    /// Split on collecting the Bridge Weaver trinket
    #[default = false]
    pub bridge_weaver: bool,
    /// Vial Salvo
    ///
    /// Split on collecting the Vial Salvo trinket
    #[default = false]
    pub vial_salvo: bool,
    /// Dodging Pendulum
    ///
    /// Split on collecting the Dodging Pendulum trinket
    #[default = false]
    pub dodging_pendulum: bool,
    /// Spring Heels
    ///
    /// Split on collecting the Spring Heels trinket
    #[default = false]
    pub spring_heels: bool,
    /// Wallower's Gauntlets
    ///
    /// Split on collecting the Wallower's Gauntlets trinket
    #[default = false]
    pub wallowers_gauntlets: bool,
    /// Oozing Organ
    ///
    /// Split on collecting the Oozing Organ trinket
    #[default = false]
    pub oozing_organ: bool,
    /// Voltaic Guard
    ///
    /// Split on collecting the Voltaic Guard trinket
    #[default = false]
    pub voltaic_guard: bool,
    /// Repulsing Root
    ///
    /// Split on collecting the Repulsing Root trinket
    #[default = false]
    pub repulsing_root: bool,
    /// Lightning Grip
    ///
    /// Split on collecting the Lightning Grip trinket
    #[default = false]
    pub lightning_grip: bool,
    /// Dead Leaf
    ///
    /// Split on collecting the Dead Leaf trinket
    #[default = false]
    pub dead_leaf: bool,
    /// Niter Belt
    ///
    /// Split on collecting the Niter Belt trinket
    #[default = false]
    pub niter_belt: bool,
    /// Bellows Bustle
    ///
    /// Split on collecting the Bellows Bustle trinket
    #[default = false]
    pub bellows_bustle: bool,
    /// Tunneling Codex
    ///
    /// Split on collecting the Tunneling Codex trinket
    #[default = false]
    pub tunneling_codex: bool,
    /// Joule Syringe
    ///
    /// Split on collecting the Joule Syringe trinket
    #[default = false]
    pub joule_syringe: bool,
    /// Polyp Lamp
    ///
    /// Split on collecting the Polyp Lamp trinket
    #[default = false]
    pub polyp_lamp: bool,
    /// Thermal Pack
    ///
    /// Split on collecting the Thermal Pack trinket
    #[default = false]
    pub thermal_pack: bool,
    /// Counter Vial
    ///
    /// Split on collecting the Counter Vial trinket
    #[default = false]
    pub counter_vial: bool,
}
