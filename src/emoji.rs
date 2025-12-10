//! Emoji support for console output.
//!
//! This module provides emoji name-to-character mapping compatible with Python Rich.
//!
//! # Example
//!
//! ```rust,ignore
//! use richrs::emoji::Emoji;
//!
//! let emoji = Emoji::new("rocket").unwrap();
//! assert_eq!(emoji.to_string(), "\u{1F680}");
//! ```

use crate::errors::{Error, Result};
use std::collections::HashMap;
use std::fmt;
use std::sync::LazyLock;

/// A map of emoji names to their Unicode characters.
///
/// This includes the most commonly used emojis from Python Rich's emoji database.
static EMOJI_MAP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    // Generated from Python Rich's emoji database
    // Each entry maps an emoji name to its Unicode character(s)
    m.insert("100", "\u{1F4AF}");
    m.insert("abacus", "\u{1F9EE}");
    m.insert("abc", "\u{1F524}");
    m.insert("abcd", "\u{1F521}");
    m.insert("airplane", "\u{2708}");
    m.insert("alarm_clock", "\u{23F0}");
    m.insert("alien", "\u{1F47D}");
    m.insert("anger", "\u{1F4A2}");
    m.insert("angry", "\u{1F620}");
    m.insert("anguished", "\u{1F627}");
    m.insert("ant", "\u{1F41C}");
    m.insert("apple", "\u{1F34E}");
    m.insert("arrow_backward", "\u{25C0}");
    m.insert("arrow_double_down", "\u{23EC}");
    m.insert("arrow_double_up", "\u{23EB}");
    m.insert("arrow_down", "\u{2B07}");
    m.insert("arrow_down_small", "\u{1F53D}");
    m.insert("arrow_forward", "\u{25B6}");
    m.insert("arrow_heading_down", "\u{2935}");
    m.insert("arrow_heading_up", "\u{2934}");
    m.insert("arrow_left", "\u{2B05}");
    m.insert("arrow_lower_left", "\u{2199}");
    m.insert("arrow_lower_right", "\u{2198}");
    m.insert("arrow_right", "\u{27A1}");
    m.insert("arrow_right_hook", "\u{21AA}");
    m.insert("arrow_up", "\u{2B06}");
    m.insert("arrow_up_down", "\u{2195}");
    m.insert("arrow_up_small", "\u{1F53C}");
    m.insert("arrow_upper_left", "\u{2196}");
    m.insert("arrow_upper_right", "\u{2197}");
    m.insert("arrows_counterclockwise", "\u{1F504}");
    m.insert("astonished", "\u{1F632}");
    m.insert("avocado", "\u{1F951}");
    m.insert("axe", "\u{1FA93}");
    m.insert("baby_chick", "\u{1F424}");
    m.insert("bacon", "\u{1F953}");
    m.insert("badger", "\u{1F9A1}");
    m.insert("badminton", "\u{1F3F8}");
    m.insert("bagel", "\u{1F96F}");
    m.insert("baguette_bread", "\u{1F956}");
    m.insert("balance_scale", "\u{2696}");
    m.insert("ballot_box_with_check", "\u{2611}");
    m.insert("banana", "\u{1F34C}");
    m.insert("bangbang", "\u{203C}");
    m.insert("bar_chart", "\u{1F4CA}");
    m.insert("baseball", "\u{26BE}");
    m.insert("basketball", "\u{1F3C0}");
    m.insert("bat", "\u{1F987}");
    m.insert("bear", "\u{1F43B}");
    m.insert("bee", "\u{1F41D}");
    m.insert("beer", "\u{1F37A}");
    m.insert("beers", "\u{1F37B}");
    m.insert("beetle", "\u{1F41E}");
    m.insert("beginner", "\u{1F530}");
    m.insert("bento", "\u{1F371}");
    m.insert("beverage_box", "\u{1F9C3}");
    m.insert("bird", "\u{1F426}");
    m.insert("black_circle", "\u{26AB}");
    m.insert("black_heart", "\u{1F5A4}");
    m.insert("black_large_square", "\u{2B1B}");
    m.insert("black_medium_small_square", "\u{25FE}");
    m.insert("black_medium_square", "\u{25FC}");
    m.insert("black_nib", "\u{2712}");
    m.insert("black_small_square", "\u{25AA}");
    m.insert("black_square_button", "\u{1F532}");
    m.insert("blossom", "\u{1F33C}");
    m.insert("blowfish", "\u{1F421}");
    m.insert("blue_book", "\u{1F4D8}");
    m.insert("blue_heart", "\u{1F499}");
    m.insert("blush", "\u{1F60A}");
    m.insert("boar", "\u{1F417}");
    m.insert("bomb", "\u{1F4A3}");
    m.insert("bone", "\u{1F9B4}");
    m.insert("book", "\u{1F4D6}");
    m.insert("bookmark", "\u{1F516}");
    m.insert("bookmark_tabs", "\u{1F4D1}");
    m.insert("books", "\u{1F4DA}");
    m.insert("boom", "\u{1F4A5}");
    m.insert("bouquet", "\u{1F490}");
    m.insert("bow_and_arrow", "\u{1F3F9}");
    m.insert("bowl_with_spoon", "\u{1F963}");
    m.insert("bowling", "\u{1F3B3}");
    m.insert("boxing_glove", "\u{1F94A}");
    m.insert("brain", "\u{1F9E0}");
    m.insert("briefcase", "\u{1F4BC}");
    m.insert("broccoli", "\u{1F966}");
    m.insert("broken_heart", "\u{1F494}");
    m.insert("brown_heart", "\u{1F90E}");
    m.insert("bug", "\u{1F41B}");
    m.insert("bulb", "\u{1F4A1}");
    m.insert("burrito", "\u{1F32F}");
    m.insert("butter", "\u{1F9C8}");
    m.insert("butterfly", "\u{1F98B}");
    m.insert("cactus", "\u{1F335}");
    m.insert("calendar", "\u{1F4C6}");
    m.insert("call_me_hand", "\u{1F919}");
    m.insert("camera", "\u{1F4F7}");
    m.insert("candle", "\u{1F56F}");
    m.insert("canned_food", "\u{1F96B}");
    m.insert("capital_abcd", "\u{1F520}");
    m.insert("card_file_box", "\u{1F5C3}");
    m.insert("card_index", "\u{1F4C7}");
    m.insert("card_index_dividers", "\u{1F5C2}");
    m.insert("carrot", "\u{1F955}");
    m.insert("cat", "\u{1F431}");
    m.insert("cd", "\u{1F4BF}");
    m.insert("chains", "\u{26D3}");
    m.insert("chart", "\u{1F4B9}");
    m.insert("chart_with_downwards_trend", "\u{1F4C9}");
    m.insert("chart_with_upwards_trend", "\u{1F4C8}");
    m.insert("check", "\u{2714}");
    m.insert("check_mark", "\u{2714}");
    m.insert("cherries", "\u{1F352}");
    m.insert("cherry_blossom", "\u{1F338}");
    m.insert("chicken", "\u{1F414}");
    m.insert("chipmunk", "\u{1F43F}");
    m.insert("cinema", "\u{1F3A6}");
    m.insert("clamp", "\u{1F5DC}");
    m.insert("clap", "\u{1F44F}");
    m.insert("clapper", "\u{1F3AC}");
    m.insert("clinking_glasses", "\u{1F942}");
    m.insert("clipboard", "\u{1F4CB}");
    m.insert("clock1", "\u{1F550}");
    m.insert("clock10", "\u{1F559}");
    m.insert("clock11", "\u{1F55A}");
    m.insert("clock12", "\u{1F55B}");
    m.insert("clock2", "\u{1F551}");
    m.insert("clock3", "\u{1F552}");
    m.insert("clock4", "\u{1F553}");
    m.insert("clock5", "\u{1F554}");
    m.insert("clock6", "\u{1F555}");
    m.insert("clock7", "\u{1F556}");
    m.insert("clock8", "\u{1F557}");
    m.insert("clock9", "\u{1F558}");
    m.insert("closed_book", "\u{1F4D5}");
    m.insert("closed_lock_with_key", "\u{1F510}");
    m.insert("cloud", "\u{2601}");
    m.insert("cloud_with_lightning_and_rain", "\u{26C8}");
    m.insert("clown_face", "\u{1F921}");
    m.insert("cocktail", "\u{1F378}");
    m.insert("coconut", "\u{1F965}");
    m.insert("coffee", "\u{2615}");
    m.insert("cold_face", "\u{1F976}");
    m.insert("cold_sweat", "\u{1F630}");
    m.insert("collision", "\u{1F4A5}");
    m.insert("computer", "\u{1F4BB}");
    m.insert("computer_mouse", "\u{1F5B1}");
    m.insert("confounded", "\u{1F616}");
    m.insert("confused", "\u{1F615}");
    m.insert("cooking", "\u{1F373}");
    m.insert("cow", "\u{1F42E}");
    m.insert("cow2", "\u{1F404}");
    m.insert("cowboy_hat_face", "\u{1F920}");
    m.insert("crab", "\u{1F980}");
    m.insert("crayon", "\u{1F58D}");
    m.insert("credit_card", "\u{1F4B3}");
    m.insert("crescent_moon", "\u{1F319}");
    m.insert("cricket", "\u{1F997}");
    m.insert("cricket_game", "\u{1F3CF}");
    m.insert("crocodile", "\u{1F40A}");
    m.insert("croissant", "\u{1F950}");
    m.insert("cross_mark", "\u{274C}");
    m.insert("crossed_fingers", "\u{1F91E}");
    m.insert("crossed_swords", "\u{2694}");
    m.insert("cry", "\u{1F622}");
    m.insert("crying_cat_face", "\u{1F63F}");
    m.insert("cucumber", "\u{1F952}");
    m.insert("cup_with_straw", "\u{1F964}");
    m.insert("cupid", "\u{1F498}");
    m.insert("curly_loop", "\u{27B0}");
    m.insert("curry", "\u{1F35B}");
    m.insert("cut_of_meat", "\u{1F969}");
    m.insert("dagger", "\u{1F5E1}");
    m.insert("dango", "\u{1F361}");
    m.insert("dart", "\u{1F3AF}");
    m.insert("dash", "\u{1F4A8}");
    m.insert("date", "\u{1F4C5}");
    m.insert("deciduous_tree", "\u{1F333}");
    m.insert("desktop_computer", "\u{1F5A5}");
    m.insert("diamond_shape_with_a_dot_inside", "\u{1F4A0}");
    m.insert("disappointed", "\u{1F61E}");
    m.insert("disappointed_relieved", "\u{1F625}");
    m.insert("diving_mask", "\u{1F93F}");
    m.insert("diya_lamp", "\u{1FA94}");
    m.insert("dizzy", "\u{1F4AB}");
    m.insert("dizzy_face", "\u{1F635}");
    m.insert("dog", "\u{1F436}");
    m.insert("dollar", "\u{1F4B5}");
    m.insert("dolphin", "\u{1F42C}");
    m.insert("dragon", "\u{1F409}");
    m.insert("dragon_face", "\u{1F432}");
    m.insert("drooling_face", "\u{1F924}");
    m.insert("droplet", "\u{1F4A7}");
    m.insert("duck", "\u{1F986}");
    m.insert("dumpling", "\u{1F95F}");
    m.insert("dvd", "\u{1F4C0}");
    m.insert("e-mail", "\u{1F4E7}");
    m.insert("eagle", "\u{1F985}");
    m.insert("ear", "\u{1F442}");
    m.insert("ear_of_rice", "\u{1F33E}");
    m.insert("ear_with_hearing_aid", "\u{1F9BB}");
    m.insert("egg", "\u{1F373}");
    m.insert("eggplant", "\u{1F346}");
    m.insert("eight_pointed_black_star", "\u{2734}");
    m.insert("eight_spoked_asterisk", "\u{2733}");
    m.insert("eject_button", "\u{23CF}");
    m.insert("elephant", "\u{1F418}");
    m.insert("email", "\u{2709}");
    m.insert("envelope_with_arrow", "\u{1F4E9}");
    m.insert("euro", "\u{1F4B6}");
    m.insert("evergreen_tree", "\u{1F332}");
    m.insert("exclamation", "\u{2757}");
    m.insert("exploding_head", "\u{1F92F}");
    m.insert("expressionless", "\u{1F611}");
    m.insert("eye", "\u{1F441}");
    m.insert("eyes", "\u{1F440}");
    m.insert("face_vomiting", "\u{1F92E}");
    m.insert("face_with_thermometer", "\u{1F912}");
    m.insert("falafel", "\u{1F9C6}");
    m.insert("fallen_leaf", "\u{1F342}");
    m.insert("fast_forward", "\u{23E9}");
    m.insert("fearful", "\u{1F628}");
    m.insert("feet", "\u{1F43E}");
    m.insert("field_hockey", "\u{1F3D1}");
    m.insert("file_cabinet", "\u{1F5C4}");
    m.insert("file_folder", "\u{1F4C1}");
    m.insert("film_projector", "\u{1F4FD}");
    m.insert("fire", "\u{1F525}");
    m.insert("first_quarter_moon", "\u{1F313}");
    m.insert("first_quarter_moon_with_face", "\u{1F31B}");
    m.insert("fish", "\u{1F41F}");
    m.insert("fish_cake", "\u{1F365}");
    m.insert("fishing_pole_and_fish", "\u{1F3A3}");
    m.insert("fist", "\u{270A}");
    m.insert("flamingo", "\u{1F9A9}");
    m.insert("flashlight", "\u{1F526}");
    m.insert("floppy_disk", "\u{1F4BE}");
    m.insert("flushed", "\u{1F633}");
    m.insert("flying_disc", "\u{1F94F}");
    m.insert("fog", "\u{1F32B}");
    m.insert("folder", "\u{1F4C1}");
    m.insert("foot", "\u{1F9B6}");
    m.insert("football", "\u{1F3C8}");
    m.insert("fortune_cookie", "\u{1F960}");
    m.insert("fountain_pen", "\u{1F58B}");
    m.insert("four_leaf_clover", "\u{1F340}");
    m.insert("fox_face", "\u{1F98A}");
    m.insert("fried_shrimp", "\u{1F364}");
    m.insert("fries", "\u{1F35F}");
    m.insert("frog", "\u{1F438}");
    m.insert("frowning", "\u{1F626}");
    m.insert("frowning_face", "\u{2639}");
    m.insert("full_moon", "\u{1F315}");
    m.insert("garlic", "\u{1F9C4}");
    m.insert("gear", "\u{2699}");
    m.insert("ghost", "\u{1F47B}");
    m.insert("gift_heart", "\u{1F49D}");
    m.insert("giraffe", "\u{1F992}");
    m.insert("globe_with_meridians", "\u{1F310}");
    m.insert("goal_net", "\u{1F945}");
    m.insert("golf", "\u{26F3}");
    m.insert("gorilla", "\u{1F98D}");
    m.insert("grapes", "\u{1F347}");
    m.insert("green_apple", "\u{1F34F}");
    m.insert("green_book", "\u{1F4D7}");
    m.insert("green_heart", "\u{1F49A}");
    m.insert("green_salad", "\u{1F957}");
    m.insert("grey_exclamation", "\u{2755}");
    m.insert("grey_question", "\u{2754}");
    m.insert("grimacing", "\u{1F62C}");
    m.insert("grinning", "\u{1F600}");
    m.insert("gun", "\u{1F52B}");
    m.insert("hamburger", "\u{1F354}");
    m.insert("hammer", "\u{1F528}");
    m.insert("hammer_and_pick", "\u{2692}");
    m.insert("hammer_and_wrench", "\u{1F6E0}");
    m.insert("hamster", "\u{1F439}");
    m.insert("hand", "\u{270B}");
    m.insert("hand_with_fingers_splayed", "\u{1F590}");
    m.insert("handshake", "\u{1F91D}");
    m.insert("hatched_chick", "\u{1F425}");
    m.insert("hatching_chick", "\u{1F423}");
    m.insert("hear_no_evil", "\u{1F649}");
    m.insert("heart", "\u{2764}");
    m.insert("heart_decoration", "\u{1F49F}");
    m.insert("heart_eyes", "\u{1F60D}");
    m.insert("heart_eyes_cat", "\u{1F63B}");
    m.insert("heartbeat", "\u{1F493}");
    m.insert("heartpulse", "\u{1F497}");
    m.insert("heavy_check_mark", "\u{2714}");
    m.insert("heavy_division_sign", "\u{2797}");
    m.insert("heavy_minus_sign", "\u{2796}");
    m.insert("heavy_multiplication_x", "\u{2716}");
    m.insert("heavy_plus_sign", "\u{2795}");
    m.insert("hedgehog", "\u{1F994}");
    m.insert("herb", "\u{1F33F}");
    m.insert("hibiscus", "\u{1F33A}");
    m.insert("high_brightness", "\u{1F506}");
    m.insert("hippopotamus", "\u{1F99B}");
    m.insert("honeybee", "\u{1F41D}");
    m.insert("horse", "\u{1F434}");
    m.insert("hot_face", "\u{1F975}");
    m.insert("hourglass", "\u{231B}");
    m.insert("hourglass_flowing_sand", "\u{23F3}");
    m.insert("hushed", "\u{1F62F}");
    m.insert("ice_hockey", "\u{1F3D2}");
    m.insert("ice_skate", "\u{26F8}");
    m.insert("imp", "\u{1F47F}");
    m.insert("inbox_tray", "\u{1F4E5}");
    m.insert("incoming_envelope", "\u{1F4E8}");
    m.insert("information_source", "\u{2139}");
    m.insert("innocent", "\u{1F607}");
    m.insert("interrobang", "\u{2049}");
    m.insert("izakaya_lantern", "\u{1F3EE}");
    m.insert("japanese_goblin", "\u{1F47A}");
    m.insert("japanese_ogre", "\u{1F479}");
    m.insert("joy", "\u{1F602}");
    m.insert("joy_cat", "\u{1F639}");
    m.insert("kangaroo", "\u{1F998}");
    m.insert("key", "\u{1F511}");
    m.insert("keyboard", "\u{2328}");
    m.insert("kissing", "\u{1F617}");
    m.insert("kissing_cat", "\u{1F63D}");
    m.insert("kissing_closed_eyes", "\u{1F61A}");
    m.insert("kissing_heart", "\u{1F618}");
    m.insert("kissing_smiling_eyes", "\u{1F619}");
    m.insert("kite", "\u{1FA81}");
    m.insert("kiwi_fruit", "\u{1F95D}");
    m.insert("koala", "\u{1F428}");
    m.insert("label", "\u{1F3F7}");
    m.insert("lacrosse", "\u{1F94D}");
    m.insert("large_blue_circle", "\u{1F535}");
    m.insert("large_blue_diamond", "\u{1F537}");
    m.insert("large_orange_diamond", "\u{1F536}");
    m.insert("last_quarter_moon", "\u{1F317}");
    m.insert("last_quarter_moon_with_face", "\u{1F31C}");
    m.insert("laughing", "\u{1F606}");
    m.insert("leafy_green", "\u{1F96C}");
    m.insert("leaves", "\u{1F343}");
    m.insert("ledger", "\u{1F4D2}");
    m.insert("left_right_arrow", "\u{2194}");
    m.insert("leftwards_arrow_with_hook", "\u{21A9}");
    m.insert("leg", "\u{1F9B5}");
    m.insert("lemon", "\u{1F34B}");
    m.insert("leopard", "\u{1F406}");
    m.insert("link", "\u{1F517}");
    m.insert("lips", "\u{1F444}");
    m.insert("lizard", "\u{1F98E}");
    m.insert("llama", "\u{1F999}");
    m.insert("lobster", "\u{1F99E}");
    m.insert("lock", "\u{1F512}");
    m.insert("lock_with_ink_pen", "\u{1F50F}");
    m.insert("loop", "\u{27BF}");
    m.insert("low_brightness", "\u{1F505}");
    m.insert("lying_face", "\u{1F925}");
    m.insert("mag", "\u{1F50D}");
    m.insert("mag_right", "\u{1F50E}");
    m.insert("magnet", "\u{1F9F2}");
    m.insert("mailbox", "\u{1F4EB}");
    m.insert("mailbox_closed", "\u{1F4EA}");
    m.insert("mailbox_with_mail", "\u{1F4EC}");
    m.insert("mailbox_with_no_mail", "\u{1F4ED}");
    m.insert("mango", "\u{1F96D}");
    m.insert("maple_leaf", "\u{1F341}");
    m.insert("martial_arts_uniform", "\u{1F94B}");
    m.insert("mask", "\u{1F637}");
    m.insert("mate", "\u{1F9C9}");
    m.insert("meat_on_bone", "\u{1F356}");
    m.insert("mechanical_arm", "\u{1F9BE}");
    m.insert("mechanical_leg", "\u{1F9BF}");
    m.insert("melon", "\u{1F348}");
    m.insert("memo", "\u{1F4DD}");
    m.insert("minidisc", "\u{1F4BD}");
    m.insert("mobile_phone_off", "\u{1F4F4}");
    m.insert("money_with_wings", "\u{1F4B8}");
    m.insert("moneybag", "\u{1F4B0}");
    m.insert("monkey", "\u{1F412}");
    m.insert("monkey_face", "\u{1F435}");
    m.insert("moon_cake", "\u{1F96E}");
    m.insert("mosquito", "\u{1F99F}");
    m.insert("mouse", "\u{1F42D}");
    m.insert("mouse2", "\u{1F401}");
    m.insert("movie_camera", "\u{1F3A5}");
    m.insert("muscle", "\u{1F4AA}");
    m.insert("mushroom", "\u{1F344}");
    m.insert("musical_note", "\u{1F3B5}");
    m.insert("nail_care", "\u{1F485}");
    m.insert("name_badge", "\u{1F4DB}");
    m.insert("nauseated_face", "\u{1F922}");
    m.insert("nerd_face", "\u{1F913}");
    m.insert("neutral_face", "\u{1F610}");
    m.insert("new_moon", "\u{1F311}");
    m.insert("new_moon_with_face", "\u{1F31A}");
    m.insert("newspaper", "\u{1F4F0}");
    m.insert("no_entry", "\u{26D4}");
    m.insert("no_mouth", "\u{1F636}");
    m.insert("nose", "\u{1F443}");
    m.insert("notebook", "\u{1F4D3}");
    m.insert("notebook_with_decorative_cover", "\u{1F4D4}");
    m.insert("notes", "\u{1F3B6}");
    m.insert("nut_and_bolt", "\u{1F529}");
    m.insert("o", "\u{2B55}");
    m.insert("ocean", "\u{1F30A}");
    m.insert("octopus", "\u{1F419}");
    m.insert("oden", "\u{1F362}");
    m.insert("ok", "\u{1F44C}");
    m.insert("ok_hand", "\u{1F44C}");
    m.insert("old_key", "\u{1F5DD}");
    m.insert("onion", "\u{1F9C5}");
    m.insert("open_book", "\u{1F4D6}");
    m.insert("open_file_folder", "\u{1F4C2}");
    m.insert("open_hands", "\u{1F450}");
    m.insert("open_mouth", "\u{1F62E}");
    m.insert("orange_book", "\u{1F4D9}");
    m.insert("orange_heart", "\u{1F9E1}");
    m.insert("orangutan", "\u{1F9A7}");
    m.insert("otter", "\u{1F9A6}");
    m.insert("outbox_tray", "\u{1F4E4}");
    m.insert("owl", "\u{1F989}");
    m.insert("ox", "\u{1F402}");
    m.insert("package", "\u{1F4E6}");
    m.insert("page_facing_up", "\u{1F4C4}");
    m.insert("page_with_curl", "\u{1F4C3}");
    m.insert("paintbrush", "\u{1F58C}");
    m.insert("palm_tree", "\u{1F334}");
    m.insert("palms_up_together", "\u{1F932}");
    m.insert("pancakes", "\u{1F95E}");
    m.insert("panda_face", "\u{1F43C}");
    m.insert("paperclip", "\u{1F4CE}");
    m.insert("parrot", "\u{1F99C}");
    m.insert("part_alternation_mark", "\u{303D}");
    m.insert("partly_sunny", "\u{26C5}");
    m.insert("partying_face", "\u{1F973}");
    m.insert("peach", "\u{1F351}");
    m.insert("peacock", "\u{1F99A}");
    m.insert("pear", "\u{1F350}");
    m.insert("pen", "\u{1F58A}");
    m.insert("pencil", "\u{270F}");
    m.insert("pencil2", "\u{270F}");
    m.insert("penguin", "\u{1F427}");
    m.insert("pensive", "\u{1F614}");
    m.insert("persevere", "\u{1F623}");
    m.insert("pick", "\u{26CF}");
    m.insert("pig", "\u{1F437}");
    m.insert("pig2", "\u{1F416}");
    m.insert("pig_nose", "\u{1F43D}");
    m.insert("pinching_hand", "\u{1F90F}");
    m.insert("pineapple", "\u{1F34D}");
    m.insert("ping_pong", "\u{1F3D3}");
    m.insert("pizza", "\u{1F355}");
    m.insert("pleading_face", "\u{1F97A}");
    m.insert("point_down", "\u{1F447}");
    m.insert("point_left", "\u{1F448}");
    m.insert("point_right", "\u{1F449}");
    m.insert("point_up", "\u{261D}");
    m.insert("point_up_2", "\u{1F446}");
    m.insert("pool_8_ball", "\u{1F3B1}");
    m.insert("poop", "\u{1F4A9}");
    m.insert("popcorn", "\u{1F37F}");
    m.insert("postbox", "\u{1F4EE}");
    m.insert("potato", "\u{1F954}");
    m.insert("poultry_leg", "\u{1F357}");
    m.insert("pound", "\u{1F4B7}");
    m.insert("pouting_cat", "\u{1F63E}");
    m.insert("pray", "\u{1F64F}");
    m.insert("pretzel", "\u{1F968}");
    m.insert("printer", "\u{1F5A8}");
    m.insert("probing_cane", "\u{1F9AF}");
    m.insert("prohibited", "\u{1F6AB}");
    m.insert("purple_heart", "\u{1F49C}");
    m.insert("pushpin", "\u{1F4CC}");
    m.insert("question", "\u{2753}");
    m.insert("rabbit", "\u{1F430}");
    m.insert("rabbit2", "\u{1F407}");
    m.insert("radio_button", "\u{1F518}");
    m.insert("rage", "\u{1F621}");
    m.insert("rainbow", "\u{1F308}");
    m.insert("raised_back_of_hand", "\u{1F91A}");
    m.insert("raised_hands", "\u{1F64C}");
    m.insert("ram", "\u{1F40F}");
    m.insert("ramen", "\u{1F35C}");
    m.insert("rat", "\u{1F400}");
    m.insert("receipt", "\u{1F9FE}");
    m.insert("recycle", "\u{267B}");
    m.insert("red_circle", "\u{1F534}");
    m.insert("red_heart", "\u{2764}");
    m.insert("relaxed", "\u{263A}");
    m.insert("relieved", "\u{1F60C}");
    m.insert("repeat", "\u{1F501}");
    m.insert("repeat_one", "\u{1F502}");
    m.insert("revolving_hearts", "\u{1F49E}");
    m.insert("rewind", "\u{23EA}");
    m.insert("rhinoceros", "\u{1F98F}");
    m.insert("rice", "\u{1F35A}");
    m.insert("rice_ball", "\u{1F359}");
    m.insert("rice_cracker", "\u{1F358}");
    m.insert("robot", "\u{1F916}");
    m.insert("rocket", "\u{1F680}");
    m.insert("rolling_on_the_floor_laughing", "\u{1F923}");
    m.insert("rose", "\u{1F339}");
    m.insert("rosette", "\u{1F3F5}");
    m.insert("round_pushpin", "\u{1F4CD}");
    m.insert("rugby_football", "\u{1F3C9}");
    m.insert("running_shirt_with_sash", "\u{1F3BD}");
    m.insert("sake", "\u{1F376}");
    m.insert("salt", "\u{1F9C2}");
    m.insert("sandwich", "\u{1F96A}");
    m.insert("sauropod", "\u{1F995}");
    m.insert("scissors", "\u{2702}");
    m.insert("scorpion", "\u{1F982}");
    m.insert("scream", "\u{1F631}");
    m.insert("scream_cat", "\u{1F640}");
    m.insert("scroll", "\u{1F4DC}");
    m.insert("see_no_evil", "\u{1F648}");
    m.insert("seedling", "\u{1F331}");
    m.insert("selfie", "\u{1F933}");
    m.insert("shallow_pan_of_food", "\u{1F958}");
    m.insert("shamrock", "\u{2618}");
    m.insert("shark", "\u{1F988}");
    m.insert("sheep", "\u{1F411}");
    m.insert("shield", "\u{1F6E1}");
    m.insert("shrimp", "\u{1F990}");
    m.insert("shushing_face", "\u{1F92B}");
    m.insert("signal_strength", "\u{1F4F6}");
    m.insert("ski", "\u{1F3BF}");
    m.insert("skull", "\u{1F480}");
    m.insert("skull_and_crossbones", "\u{2620}");
    m.insert("skunk", "\u{1F9A8}");
    m.insert("sled", "\u{1F6F7}");
    m.insert("sleeping", "\u{1F634}");
    m.insert("sleepy", "\u{1F62A}");
    m.insert("slightly_frowning_face", "\u{1F641}");
    m.insert("slightly_smiling_face", "\u{1F642}");
    m.insert("sloth", "\u{1F9A5}");
    m.insert("small_blue_diamond", "\u{1F539}");
    m.insert("small_orange_diamond", "\u{1F538}");
    m.insert("small_red_triangle", "\u{1F53A}");
    m.insert("small_red_triangle_down", "\u{1F53B}");
    m.insert("smile", "\u{1F604}");
    m.insert("smile_cat", "\u{1F638}");
    m.insert("smiley_cat", "\u{1F63A}");
    m.insert("smiling_imp", "\u{1F608}");
    m.insert("smirk", "\u{1F60F}");
    m.insert("smirk_cat", "\u{1F63C}");
    m.insert("snail", "\u{1F40C}");
    m.insert("snake", "\u{1F40D}");
    m.insert("sneezing_face", "\u{1F927}");
    m.insert("snowflake", "\u{2744}");
    m.insert("snowman", "\u{2603}");
    m.insert("sob", "\u{1F62D}");
    m.insert("soccer", "\u{26BD}");
    m.insert("softball", "\u{1F94E}");
    m.insert("space_invader", "\u{1F47E}");
    m.insert("spaghetti", "\u{1F35D}");
    m.insert("sparkle", "\u{2747}");
    m.insert("sparkles", "\u{2728}");
    m.insert("sparkling_heart", "\u{1F496}");
    m.insert("speak_no_evil", "\u{1F64A}");
    m.insert("speech_balloon", "\u{1F4AC}");
    m.insert("spider", "\u{1F577}");
    m.insert("spider_web", "\u{1F578}");
    m.insert("spiral_calendar", "\u{1F5D3}");
    m.insert("spiral_notepad", "\u{1F5D2}");
    m.insert("squid", "\u{1F991}");
    m.insert("star", "\u{2B50}");
    m.insert("star2", "\u{1F31F}");
    m.insert("stew", "\u{1F372}");
    m.insert("stop_sign", "\u{1F6D1}");
    m.insert("stopwatch", "\u{23F1}");
    m.insert("straight_ruler", "\u{1F4CF}");
    m.insert("strawberry", "\u{1F353}");
    m.insert("stuck_out_tongue", "\u{1F61B}");
    m.insert("stuck_out_tongue_closed_eyes", "\u{1F61D}");
    m.insert("stuck_out_tongue_winking_eye", "\u{1F61C}");
    m.insert("stuffed_flatbread", "\u{1F959}");
    m.insert("sun", "\u{2600}");
    m.insert("sunflower", "\u{1F33B}");
    m.insert("sunglasses", "\u{1F60E}");
    m.insert("sunny", "\u{2600}");
    m.insert("sushi", "\u{1F363}");
    m.insert("swan", "\u{1F9A2}");
    m.insert("sweat", "\u{1F613}");
    m.insert("sweat_drops", "\u{1F4A6}");
    m.insert("sweat_smile", "\u{1F605}");
    m.insert("sweet_potato", "\u{1F360}");
    m.insert("symbols", "\u{1F523}");
    m.insert("taco", "\u{1F32E}");
    m.insert("takeout_box", "\u{1F961}");
    m.insert("tangerine", "\u{1F34A}");
    m.insert("tea", "\u{1F375}");
    m.insert("tennis", "\u{1F3BE}");
    m.insert("thought_balloon", "\u{1F4AD}");
    m.insert("thumbs_down", "\u{1F44E}");
    m.insert("thumbs_up", "\u{1F44D}");
    m.insert("thumbsdown", "\u{1F44E}");
    m.insert("thumbsup", "\u{1F44D}");
    m.insert("tiger", "\u{1F42F}");
    m.insert("tiger2", "\u{1F405}");
    m.insert("timer_clock", "\u{23F2}");
    m.insert("tired_face", "\u{1F62B}");
    m.insert("tomato", "\u{1F345}");
    m.insert("tongue", "\u{1F445}");
    m.insert("toolbox", "\u{1F9F0}");
    m.insert("tooth", "\u{1F9B7}");
    m.insert("tornado", "\u{1F32A}");
    m.insert("trackball", "\u{1F5B2}");
    m.insert("triangular_ruler", "\u{1F4D0}");
    m.insert("trident", "\u{1F531}");
    m.insert("triumph", "\u{1F624}");
    m.insert("tropical_drink", "\u{1F379}");
    m.insert("tropical_fish", "\u{1F420}");
    m.insert("tulip", "\u{1F337}");
    m.insert("tumbler_glass", "\u{1F943}");
    m.insert("turtle", "\u{1F422}");
    m.insert("tv", "\u{1F4FA}");
    m.insert("twisted_rightwards_arrows", "\u{1F500}");
    m.insert("two_hearts", "\u{1F495}");
    m.insert("umbrella", "\u{2602}");
    m.insert("unamused", "\u{1F612}");
    m.insert("unlock", "\u{1F513}");
    m.insert("v", "\u{270C}");
    m.insert("vhs", "\u{1F4FC}");
    m.insert("vibration_mode", "\u{1F4F3}");
    m.insert("video_camera", "\u{1F4F9}");
    m.insert("volleyball", "\u{1F3D0}");
    m.insert("vulcan_salute", "\u{1F596}");
    m.insert("waffle", "\u{1F9C7}");
    m.insert("waning_crescent_moon", "\u{1F318}");
    m.insert("waning_gibbous_moon", "\u{1F316}");
    m.insert("warning", "\u{26A0}");
    m.insert("wastebasket", "\u{1F5D1}");
    m.insert("watch", "\u{231A}");
    m.insert("water_buffalo", "\u{1F403}");
    m.insert("watermelon", "\u{1F349}");
    m.insert("wave", "\u{1F44B}");
    m.insert("waxing_crescent_moon", "\u{1F312}");
    m.insert("waxing_gibbous_moon", "\u{1F314}");
    m.insert("weary", "\u{1F629}");
    m.insert("whale", "\u{1F433}");
    m.insert("whale2", "\u{1F40B}");
    m.insert("white_check_mark", "\u{2705}");
    m.insert("white_circle", "\u{26AA}");
    m.insert("white_flower", "\u{1F4AE}");
    m.insert("white_heart", "\u{1F90D}");
    m.insert("white_large_square", "\u{2B1C}");
    m.insert("white_medium_small_square", "\u{25FD}");
    m.insert("white_medium_square", "\u{25FB}");
    m.insert("white_small_square", "\u{25AB}");
    m.insert("white_square_button", "\u{1F533}");
    m.insert("wilted_flower", "\u{1F940}");
    m.insert("wind_face", "\u{1F32C}");
    m.insert("wine_glass", "\u{1F377}");
    m.insert("wink", "\u{1F609}");
    m.insert("wolf", "\u{1F43A}");
    m.insert("woozy_face", "\u{1F974}");
    m.insert("worried", "\u{1F61F}");
    m.insert("wrench", "\u{1F527}");
    m.insert("writing_hand", "\u{270D}");
    m.insert("x", "\u{274C}");
    m.insert("yawning_face", "\u{1F971}");
    m.insert("yellow_heart", "\u{1F49B}");
    m.insert("yen", "\u{1F4B4}");
    m.insert("yum", "\u{1F60B}");
    m.insert("zany_face", "\u{1F92A}");
    m.insert("zap", "\u{26A1}");
    m.insert("zebra", "\u{1F993}");
    m.insert("zzz", "\u{1F4A4}");
    m
});

/// An emoji that can be rendered to the console.
///
/// # Example
///
/// ```rust,ignore
/// use richrs::emoji::Emoji;
///
/// let rocket = Emoji::new("rocket")?;
/// println!("{}", rocket);  // Prints: 🚀
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Emoji {
    /// The name of the emoji.
    name: String,
    /// The Unicode character(s) for this emoji.
    char: String,
}

impl Emoji {
    /// Creates a new emoji from a name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the emoji (e.g., "rocket", "smile", "fire")
    ///
    /// # Returns
    ///
    /// Returns `Ok(Emoji)` if the name is found, or an error if not.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use richrs::emoji::Emoji;
    ///
    /// let emoji = Emoji::new("thumbs_up")?;
    /// assert_eq!(emoji.to_string(), "👍");
    /// ```
    #[inline]
    pub fn new(name: &str) -> Result<Self> {
        EMOJI_MAP.get(name).map_or_else(
            || Err(Error::NoEmoji(name.to_owned())),
            |&char| {
                Ok(Self {
                    name: name.to_owned(),
                    char: char.to_owned(),
                })
            },
        )
    }

    /// Returns the name of this emoji.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the Unicode character(s) for this emoji.
    #[inline]
    #[must_use]
    pub fn char(&self) -> &str {
        &self.char
    }

    /// Checks if an emoji name exists in the database.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use richrs::emoji::Emoji;
    ///
    /// assert!(Emoji::exists("rocket"));
    /// assert!(!Emoji::exists("nonexistent"));
    /// ```
    #[inline]
    #[must_use]
    pub fn exists(name: &str) -> bool {
        EMOJI_MAP.contains_key(name)
    }

    /// Returns the number of emojis in the database.
    #[inline]
    #[must_use]
    pub fn count() -> usize {
        EMOJI_MAP.len()
    }

    /// Returns an iterator over all emoji names.
    #[inline]
    pub fn names() -> impl Iterator<Item = &'static str> {
        EMOJI_MAP.keys().copied()
    }

    /// Replaces emoji codes in text with their Unicode characters.
    ///
    /// Emoji codes are specified using the `:name:` syntax.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use richrs::emoji::Emoji;
    ///
    /// let text = "Hello :rocket: World :fire:!";
    /// let result = Emoji::replace(text);
    /// assert_eq!(result, "Hello 🚀 World 🔥!");
    /// ```
    #[must_use]
    pub fn replace(text: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut chars = text.chars().peekable();

        while let Some(c) = chars.next() {
            if c == ':' {
                // Try to find the closing colon
                let mut name = String::new();
                let mut found_closing = false;
                let mut invalid_char: Option<char> = None;

                for next_c in chars.by_ref() {
                    if next_c == ':' {
                        found_closing = true;
                        break;
                    }
                    // Only allow valid emoji name characters
                    if next_c.is_ascii_alphanumeric() || next_c == '_' || next_c == '-' {
                        name.push(next_c);
                    } else {
                        // Invalid character, not an emoji code
                        invalid_char = Some(next_c);
                        break;
                    }
                }

                if found_closing {
                    if let Some(&emoji_char) = EMOJI_MAP.get(name.as_str()) {
                        result.push_str(emoji_char);
                    } else {
                        // Not a valid emoji, keep the original text
                        result.push(':');
                        result.push_str(&name);
                        result.push(':');
                    }
                } else {
                    // No closing colon found, keep original
                    result.push(':');
                    result.push_str(&name);
                    // Also push the invalid character that broke the sequence
                    if let Some(invalid) = invalid_char {
                        result.push(invalid);
                    }
                }
            } else {
                result.push(c);
            }
        }

        result
    }
}

impl fmt::Display for Emoji {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.char)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_emoji_new_valid() {
        let emoji = Emoji::new("rocket").unwrap();
        assert_eq!(emoji.name(), "rocket");
        assert_eq!(emoji.char(), "\u{1F680}");
        assert_eq!(emoji.to_string(), "\u{1F680}");
    }

    #[test]
    fn test_emoji_new_invalid() {
        let result = Emoji::new("nonexistent_emoji_name");
        assert!(result.is_err());
    }

    #[test]
    fn test_emoji_exists() {
        assert!(Emoji::exists("rocket"));
        assert!(Emoji::exists("fire"));
        assert!(Emoji::exists("smile"));
        assert!(!Emoji::exists("nonexistent"));
    }

    #[test]
    fn test_emoji_count() {
        // We should have a substantial number of emojis
        assert!(Emoji::count() > 600);
    }

    #[test]
    fn test_emoji_names() {
        let names: Vec<_> = Emoji::names().collect();
        assert!(names.contains(&"rocket"));
        assert!(names.contains(&"fire"));
        assert!(names.contains(&"thumbs_up"));
    }

    #[test]
    fn test_emoji_display() {
        let emoji = Emoji::new("smile").unwrap();
        assert_eq!(format!("{emoji}"), "\u{1F604}");
    }

    #[test]
    fn test_common_emojis() {
        // Test common emojis that users are likely to use
        let test_cases = [
            ("smile", "\u{1F604}"),
            ("thumbs_up", "\u{1F44D}"),
            ("thumbs_down", "\u{1F44E}"),
            ("heart", "\u{2764}"),
            ("fire", "\u{1F525}"),
            ("rocket", "\u{1F680}"),
            ("star", "\u{2B50}"),
            ("warning", "\u{26A0}"),
            ("x", "\u{274C}"),
            ("white_check_mark", "\u{2705}"),
            ("heavy_check_mark", "\u{2714}"),
            ("coffee", "\u{2615}"),
            ("bug", "\u{1F41B}"),
            ("gear", "\u{2699}"),
            ("lock", "\u{1F512}"),
            ("key", "\u{1F511}"),
            ("bulb", "\u{1F4A1}"),
            ("boom", "\u{1F4A5}"),
            ("sparkles", "\u{2728}"),
            ("zap", "\u{26A1}"),
        ];

        for (name, expected_char) in test_cases {
            let emoji = Emoji::new(name).unwrap();
            assert_eq!(
                emoji.char(),
                expected_char,
                "Emoji {name} should be {expected_char}"
            );
        }
    }

    #[test]
    fn test_emoji_replace_simple() {
        let text = "Hello :rocket: World!";
        let result = Emoji::replace(text);
        assert_eq!(result, "Hello \u{1F680} World!");
    }

    #[test]
    fn test_emoji_replace_multiple() {
        let text = ":fire: Code :rocket: Launch :star:";
        let result = Emoji::replace(text);
        assert_eq!(result, "\u{1F525} Code \u{1F680} Launch \u{2B50}");
    }

    #[test]
    fn test_emoji_replace_invalid() {
        let text = "Hello :nonexistent: World";
        let result = Emoji::replace(text);
        assert_eq!(result, "Hello :nonexistent: World");
    }

    #[test]
    fn test_emoji_replace_no_emojis() {
        let text = "Hello World!";
        let result = Emoji::replace(text);
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_emoji_replace_unclosed() {
        let text = "Hello :rocket World";
        let result = Emoji::replace(text);
        assert_eq!(result, "Hello :rocket World");
    }

    #[test]
    fn test_emoji_replace_adjacent() {
        let text = ":fire::rocket:";
        let result = Emoji::replace(text);
        assert_eq!(result, "\u{1F525}\u{1F680}");
    }

    #[test]
    fn test_emoji_clone() {
        let emoji = Emoji::new("rocket").unwrap();
        let cloned = emoji.clone();
        assert_eq!(emoji, cloned);
    }

    #[test]
    fn test_emoji_equality() {
        let emoji1 = Emoji::new("rocket").unwrap();
        let emoji2 = Emoji::new("rocket").unwrap();
        let emoji3 = Emoji::new("fire").unwrap();

        assert_eq!(emoji1, emoji2);
        assert_ne!(emoji1, emoji3);
    }

    #[test]
    fn test_emoji_aliases() {
        // Test some common aliases
        assert_eq!(
            Emoji::new("thumbsup").unwrap().char(),
            Emoji::new("thumbs_up").unwrap().char()
        );
        assert_eq!(
            Emoji::new("ok").unwrap().char(),
            Emoji::new("ok_hand").unwrap().char()
        );
    }
}
