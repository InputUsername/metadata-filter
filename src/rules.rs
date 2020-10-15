use std::borrow::Cow;

use once_cell::sync::Lazy;
use regex::Regex;

pub(crate) struct FilterRule(Regex, &'static str);

impl FilterRule {
    pub(crate) fn apply<'t>(&self, text: &'t str) -> Cow<'t, str> {
        self.0.replace(text, self.1)
    }
}

macro_rules! filter_rules {
    ($name:ident, $rules:expr) => {
        pub(crate) static $name: Lazy<Vec<FilterRule>> = Lazy::new(|| {
            $rules
            .iter()
            .map(|rule| FilterRule(Regex::new(rule.0).unwrap(), rule.1))
            .collect()
        });
    };
}

filter_rules!(YOUTUBE_TRACK_FILTER_RULES, [
    // Trim whitespaces
    (r"^\s+", ""),
    (r"\s+$", ""),
    // **NEW**
    (r"\*+\s?\S+\s?\*+$", ""),
    // [whatever]
    (r"\[[^\]]+\]", ""),
    // (whatever version)
    (r"(?i)\([^)]*version\)$", ""),
    // video extensions
    (r"(?i)\.(avi|wmv|mpg|mpeg|flv)$", ""),
    // (LYRICs VIDEO)
    (r"(?i)\(.*lyrics?\s*(video)?\)", ""),
    // (Official Track Stream)
    (r"(?i)\((of+icial\s*)?(track\s*)?stream\)", ""),
    // (official)? (music)? video
    (r"(?i)\((of+icial\s*)?(music\s*)?video\)", ""),
    // (official)? (music)? audio
    (r"(?i)\((of+icial\s*)?(music\s*)?audio\)", ""),
    // (ALBUM TRACK)
    (r"(?i)(ALBUM TRACK\s*)?(album track\s*)", ""),
    // (Cover Art)
    (r"(?i)(COVER ART\s*)?(Cover Art\s*)", ""),
    // (official)
    (r"(?i)\(\s*of+icial\s*\)", ""),
    // (1999)
    (r"(?i)\(\s*[0-9]{4}\s*\)", ""),
    // HD (HQ)
    (r"(HD|HQ)\s*$", ""),
    // video clip officiel or video clip official
    (r"(?i)(vid[\u00E9e]o)?\s?clip\sof+ici[ae]l", ""),
    // offizielles
    (r"(?i)of+iziel+es\s*video", ""),
    // video clip
    (r"(?i)vid[\u00E9e]o\s?clip", ""),
    // clip
    (r"(?i)\sclip", ""),
    // Full Album
    (r"(?i)full\s*album", ""),
    // (live)
    (r"(?i)\(live.*?\)$", ""),
    // | something
    (r"(?i)\|.*$", ""),
    // Artist - The new "Track title" featuring someone
    (r#"^(|.*\s)"(.{5,})"(\s.*|)$, "#, "$2"),
    // 'Track title'
    (r"^(|.*\s)'(.{5,})'(\s.*|)$, ", "$2"),
    // (*01/01/1999*)
    (r"(?i)\(.*[0-9]{1,2}\/[0-9]{1,2}\/[0-9]{2,4}.*\)", ""),
    // Sub Español
    (r"(?i)sub\s*español", ""),
    // (Letra/Lyrics)
    (r"(?i)\s\(Letra\/Lyrics\)", ""),
    // (Letra)
    (r"(?i)\s\(Letra\)", ""),
    // (En vivo)
    (r"(?i)\s\(En\svivo\)", ""),
]);

filter_rules!(TRIM_SYMBOLS_FILTER_RULES, [
    // Leftovers after e.g. (official video)
    (r"\(+\s*\)+", ""),
    // trim starting white chars and dash
    (r#"^[/,:;~-\s"]+"#, ""),
    // trim trailing white chars and dash
    (r#"[/,:;~-\s"]+$"#, ""),
]);

filter_rules!(REMASTERED_FILTER_RULES, [
    // Here Comes The Sun - Remastered
    (r"-\sRemastered$", ""),
    // Hey Jude - Remastered 2015
    (r"-\sRemastered\s\d+$", ""),
    // Let It Be (Remastered 2009)
    // Red Rain (Remaster 2012)
    (r"\(Remaster(ed)?\s\d+\)$", ""),
    // Pigs On The Wing (Part One) [2011 - Remaster]
    (r"\[\d+\s-\sRemaster\]$", ""),
    // Comfortably Numb (2011 - Remaster)
    // Dancing Days (2012 Remaster)
    (r"\(\d+(\s-)?\sRemaster\)$", ""),
    // Outside The Wall - 2011 - Remaster
    // China Grove - 2006 Remaster
    (r"-\s\d+(\s-)?\sRemaster$", ""),
    // Learning To Fly - 2001 Digital Remaster
    (r"-\s\d+\s.+?\sRemaster$", ""),
    // Your Possible Pasts - 2011 Remastered Version
    (r"-\s\d+\sRemastered Version$", ""),
    // Roll Over Beethoven (Live / Remastered)
    (r"\(Live\s/\sRemastered\)$", ""),
    // Ticket To Ride - Live / Remastered
    (r"-\sLive\s/\sRemastered$", ""),
    // Mothership (Remastered)
    // How The West Was Won [Remastered]
    (r"[(\[]Remastered[)\]]$", ""),
    // A Well Respected Man (2014 Remastered Version)
    // A Well Respected Man [2014 Remastered Version]
    (r"[(\[]\d{4} Re[Mm]astered Version[)\]]$", ""),
    // She Was Hot (2009 Re-Mastered Digital Version)
    // She Was Hot (2009 Remastered Digital Version)
    (r"[(\[]\d{4} Re-?[Mm]astered Digital Version[)\]]$", ""),
    // In The Court Of The Crimson King (Expanded & Remastered Original Album Mix)
    (r"\([^(]*Remaster[^)]*\)$", ""),
]);

filter_rules!(LIVE_FILTER_RULES, [
    // Track - Live
    (r"-\sLive?$", ""),
    // Track - Live at
    (r"-\sLive\s.+?$", ""),
]);

filter_rules!(CLEAN_EXPLICIT_FILTER_RULES, [
    // (Explicit) or [Explicit]
    (r"(?i)\s[([]Explicit[)\]]", ""),
    // (Clean) or [Clean]
    (r"(?i)\s[([]Clean[)\]]", ""),
]);

filter_rules!(FEATURE_FILTER_RULES, [
    // [Feat. Artist] or (Feat. Artist)
    (r"(?i)\s[([]feat. .+[)\]]", "$1"),
]);

filter_rules!(NORMALIZE_FEATURE_FILTER_RULES, [
    // [Feat. Artist] or (Feat. Artist) -> Feat. Artist
    (r"\s[([](feat. .+)[)\]]", "$1"),
]);

filter_rules!(VERSION_FILTER_RULES, [
    // Love Will Come To You (Album Version)
    (r"[([]Album Version[)\]]$", ""),
    // I Melt With You (Rerecorded)
    // When I Need You [Re-Recorded]
    (r"[([]Re-?recorded[)\]]$", ""),
    // Your Cheatin' Heart (Single Version)
    (r"[([]Single Version[)\]]$", ""),
    // All Over Now (Edit)
    (r"[([]Edit[)\]]$", ""),
    // (I Can't Get No) Satisfaction - Mono Version
    (r"-\sMono Version$", ""),
    // Ruby Tuesday - Stereo Version
    (r"-\sStereo Version$", ""),
    // Pure McCartney (Deluxe Edition)
    (r"\(Deluxe Edition\)$", ""),
    // 6 Foot 7 Foot (Explicit Version)
    (r"(?i)[([]Explicit Version[)\]]/", ""),
]);

filter_rules!(SUFFIX_FILTER_RULES, [
    // "- X Remix" -> "(X Remix)" and similar
    (r"(?i)-\s(.+?)\s((Re)?mix|edit|dub|mix|vip|version)$", "($1 $2)"),
    (r"-\s(Remix|VIP)$", "($1)"),
]);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remastered_filter_rules() {
        let titles = vec![
            ("Here Comes The Sun - Remastered", "Here Comes The Sun "),
            ("Hey Jude - Remastered 2015", "Hey Jude "),
            ("Let It Be (Remastered 2009)", "Let It Be "),
            ("Red Rain (Remaster 2012)", "Red Rain "),
            ("Pigs On The Wing (Part One) [2011 - Remaster]", "Pigs On The Wing (Part One) "),
            ("Comfortably Numb (2011 - Remaster)", "Comfortably Numb "),
            ("Dancing Days (2012 Remaster)", "Dancing Days "),
            ("Outside The Wall - 2011 - Remaster", "Outside The Wall "),
            ("China Grove - 2006 Remaster", "China Grove "),
            ("Learning To Fly - 2001 Digital Remaster", "Learning To Fly "),
            ("Your Possible Pasts - 2011 Remastered Version", "Your Possible Pasts "),
            ("Roll Over Beethoven (Live / Remastered)", "Roll Over Beethoven "),
            ("Ticket To Ride - Live / Remastered", "Ticket To Ride "),
            ("Mothership (Remastered)", "Mothership "),
            ("How The West Was Won [Remastered]", "How The West Was Won "),
            ("A Well Respected Man (2014 Remastered Version)", "A Well Respected Man "),
            ("A Well Respected Man [2014 Remastered Version]", "A Well Respected Man "),
            ("She Was Hot (2009 Re-Mastered Digital Version)", "She Was Hot "),
            ("She Was Hot (2009 Remastered Digital Version)", "She Was Hot "),
            ("In The Court Of The Crimson King (Expanded & Remastered Original Album Mix)",
                "In The Court Of The Crimson King "),
        ];

        for title in titles {
            for rule in REMASTERED_FILTER_RULES.iter() {
                let filtered = rule.apply(title.0);
                if filtered != title.0 {
                    assert_eq!(filtered, title.1);
                }
            }
        }
    }
}