use std::{
    io::{Cursor, Write, stdin, stdout},
    thread::spawn,
};

use random::Source;
use rodio::{DeviceSinkBuilder, PlayError, mixer::Mixer, play};

macro_rules! sound_map {
    ($($name:literal -> [$($file_name:literal,)*],)*) => {
        (
            &[$(
                ($name, &[$(include_bytes!(concat!("../assets/Flowery_voiceclip_", $file_name, ".wav")),)*]),
            )*],

            concat!($(concat!($name, " -> ", concat!($($file_name,)*)), "\n",)*)
        )
    };
}

type TupleMap<'a, K, V> = &'a [(K, V)];

static CMD_MAPS: (TupleMap<&str, &[&[u8]]>, &str) = sound_map!(
    "f" -> ["Flowery_2",],
    "if" -> ["I'm_falling",],
    "th" -> ["I'm_only_trying_to_help_you",],
    "aslw" -> ["I'm_sorry_once_again_I_kept_a_lady_in_waiting",],
    "jo" -> ["Ja-Orange_1", "Ja-Orange_2",],
    "jk" -> ["Ja-st_kidding",],
    "j" -> ["Jarona_1", "Jarona_2", "Jarona_3", "Jarona_4",],
    "k" -> ["Kris",],
    "of" -> ["Omega_Flowery",],
    "s" -> ["Susie",],
    "aap" -> ["all_according_to_all_according_to_plant",],
    "sh" -> ["don't_you_like_serving_humans",],
    "fh" -> ["flowers_blooms_in_your_heart",],
    "fi" -> ["forget_it",],
    "gc" -> ["get_a_chance_1", "get_a_chance_2",],
    "gy" -> ["give_to_you",],
    "g" -> ["glue",],
    "gh" -> ["go_home",],
    "gb" -> ["goodbye",],
    "gs" -> ["great_style",],
    "tr" -> ["grown_like_a_turnip",],
    "hj" -> ["heh,_it's_my_Jarona",],
    "sd" -> ["here_I_come_San_Frandisc", "here_I_come_San_Frandisco",],
    "hr" -> ["hey_Raly",],
    "hb" -> ["hey_boys",],
    "hgg" -> ["hey_guys,_I_think_I_found_a_glue",],
    "hg" -> ["hey_guys",],
    "hlg" -> ["hey_there_little_guy",],
    "h" -> ["hoo", "huh",],
    "in" -> ["it's_all_in_a_name",],
    "iy" -> ["it's_all_yours",],
    "imf" -> ["it's_me,_Flowery",],
    "im" -> ["it's_me",],
    "ih" -> ["it's_so_human",],
    "lj" -> ["last_Jarona",],
    "lm" -> ["leaf_it_to_me",],
    "lp" -> ["lend_me_your_power",],
    "mp" -> ["mini_peppers",],
    "m" -> ["mostlys",],
    "f2" -> ["my_favorite_two",],
    "mh" -> ["my_human",],
    "mk" -> ["my_king",],
    "mw" -> ["mysterious_wind",],
    "n" -> ["no,_no,_no",],
    "nc" -> ["no_way,_it's_your_children",],
    "of" -> ["one_more_for_the_fans",],
    "pb" -> ["prism_blow",],
    "sta" -> ["say_that_again",],
    "sa" -> ["smile_again",],
    "sgs" -> ["sorry_about_that_guys",],
    "slg" -> ["sorry_about_that_little_guy",],
    "sg" -> ["sorry_about_the_guy",],
    "slw" -> ["sorry_to_keep_a_lady_in_waiting",],
    "sl" -> ["sorry_to_keep_you_ladies",],
    "sw" -> ["sorry_to_keep_you_waiting_1", "sorry_to_keep_you_waiting_2",],
    "sus" -> ["stingus",],
    "su" -> ["suckle_it_up",],
    "tt" -> ["take_that",],
    "tg" -> ["that's_great",],
    "tmd" -> ["that's_my_dreams",],
    "tb" -> ["the_boys",],
    "td" -> ["the_diner",],
    "ef" -> ["they're_eating_my_flesh",],
    "tbf" -> ["this_guy's_your_best_friend",],
    "tf" -> ["try_my_flavor",],
    "pc" -> ["what_a_predictable_creature",],
    "wpc" -> ["with_your_powers_combined",],
    "w" -> ["wow",],
    "y" -> ["yes",],
    "yh" -> ["you're_a_hero",],
    "dbf" -> ["your_dad's_my_best_friend",],
    "yd" -> ["your_dad",],
);

static SOUND_MAP: TupleMap<&str, &[&[u8]]> = CMD_MAPS.0;
static NAME_MAP: &str = CMD_MAPS.1;

fn play_command(cmd: &str, rng: &mut impl Source, mixer: &Mixer) -> Result<(), PlayError> {
    if let Some(sounds) = SOUND_MAP
        .iter()
        .find_map(|(name, sounds)| (*name == cmd).then_some(sounds))
    {
        let sound_id = rng.read::<usize>() % sounds.len();
        let player = play(mixer, Cursor::new(sounds[sound_id]))?;
        spawn(move || {
            player.play();
            player.sleep_until_end();
        });
    } else {
        println!("Can't find command: {cmd}");
    }
    Ok(())
}

fn main() {
    let mut rng = random::default(1225);
    let mut line = String::new();
    let Ok(sink_handle) = DeviceSinkBuilder::open_default_sink().map(|mut handle| {
        handle.log_on_drop(false);
        handle
    }) else {
        eprintln!("Can't open default audio stream");
        return;
    };
    let mixer = sink_handle.mixer();

    loop {
        print!("> ");
        stdout().lock().flush().unwrap();

        if stdin().read_line(&mut line).is_ok() {
            match line.as_str().strip_suffix('\n').unwrap_or_default() {
                "quit" | "exit" | "close" => {
                    break;
                }
                "help" => {
                    print!("{NAME_MAP}");
                }
                cmd => {
                    if play_command(cmd, &mut rng, mixer).is_err() {
                        eprintln!("Can't create a sound player");
                        break;
                    }
                }
            }
            line.clear();
        } else {
            eprintln!("Error when reading from command line");
            break;
        }
    }
}
