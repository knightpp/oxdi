use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Language {
    /// British English
    EnGb,
    /// American English
    EnUs,
    /// Arabic
    Ar,
    /// Chinese
    Zh,
    /// Farsi
    Fa,
    /// French
    Fr,
    /// Georgian
    Ka,
    /// German
    De,
    /// Greek
    El,
    /// Gujarati
    Gu,
    /// Hausa
    Ha,
    /// Hindi
    Hi,
    /// Igbo
    Ig,
    /// Indonesian
    Id,
    /// IsiXhosa
    Xh,
    /// IsiZulu
    Zu,
    /// Italian
    It,
    /// Latvian
    Lv,
    /// Malay
    Ms,
    /// Marathi
    Mr,
    /// NorthernSotho
    Nso,
    /// Portuguese
    Pt,
    /// Quechua
    Qu,
    /// Romanian
    Ro,
    /// Russian
    Ru,
    /// Setswana
    Tn,
    /// Spanish
    Es,
    /// Swahili
    Sw,
    /// Tajik
    Tg,
    /// Tamil
    Ta,
    /// Tatar
    Tt,
    /// Telugu
    Te,
    /// TokPisin
    Tpi,
    /// Turkmen
    Tk,
    /// Urdu
    Ur,
    /// Yoruba
    Yo,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

macro_rules! lang {
    (en-gb) => {
        Language::EnGb
    };
    (en-us) => {
        Language::EnUs
    };
    (ar) => {
        Language::Ar
    };
    (zh) => {
        Language::Zh
    };
    (fa) => {
        Language::Fa
    };
    (fr) => {
        Language::Fr
    };
    (ka) => {
        Language::Ka
    };
    (de) => {
        Language::De
    };
    (el) => {
        Language::El
    };
    (gu) => {
        Language::Gu
    };
    (ha) => {
        Language::Ha
    };
    (hi) => {
        Language::Hi
    };
    (ig) => {
        Language::Ig
    };
    (id) => {
        Language::Id
    };
    (xh) => {
        Language::Xh
    };
    (zu) => {
        Language::Zu
    };
    (it) => {
        Language::It
    };
    (lv) => {
        Language::Lv
    };
    (ms) => {
        Language::Ms
    };
    (mr) => {
        Language::Mr
    };
    (nso) => {
        Language::Nso
    };
    (pt) => {
        Language::Pt
    };
    (qu) => {
        Language::Qu
    };
    (ro) => {
        Language::Ro
    };
    (ru) => {
        Language::Ru
    };
    (tn) => {
        Language::Tn
    };
    (es) => {
        Language::Es
    };
    (sw) => {
        Language::Sw
    };
    (tg) => {
        Language::Tg
    };
    (ta) => {
        Language::Ta
    };
    (tt) => {
        Language::Tt
    };
    (te) => {
        Language::Te
    };
    (tpi) => {
        Language::Tpi
    };
    (tk) => {
        Language::Tk
    };
    (ur) => {
        Language::Ur
    };
    (yo) => {
        Language::Yo
    };
}

impl Language {
    pub fn from_str(string: &str) -> Option<Language> {
        match string {
            "en-gb" => Some(Language::EnGb),
            "en-us" => Some(Language::EnUs),
            "ar" => Some(Language::Ar),
            "zh" => Some(Language::Zh),
            "fa" => Some(Language::Fa),
            "fr" => Some(Language::Fr),
            "ka" => Some(Language::Ka),
            "de" => Some(Language::De),
            "el" => Some(Language::El),
            "gu" => Some(Language::Gu),
            "ha" => Some(Language::Ha),
            "hi" => Some(Language::Hi),
            "ig" => Some(Language::Ig),
            "id" => Some(Language::Id),
            "xh" => Some(Language::Xh),
            "zu" => Some(Language::Zu),
            "it" => Some(Language::It),
            "lv" => Some(Language::Lv),
            "ms" => Some(Language::Ms),
            "mr" => Some(Language::Mr),
            "nso" => Some(Language::Nso),
            "pt" => Some(Language::Pt),
            "qu" => Some(Language::Qu),
            "ro" => Some(Language::Ro),
            "ru" => Some(Language::Ru),
            "tn" => Some(Language::Tn),
            "es" => Some(Language::Es),
            "sw" => Some(Language::Sw),
            "tg" => Some(Language::Tg),
            "ta" => Some(Language::Ta),
            "tt" => Some(Language::Tt),
            "te" => Some(Language::Te),
            "tpi" => Some(Language::Tpi),
            "tk" => Some(Language::Tk),
            "ur" => Some(Language::Ur),
            "yo" => Some(Language::Yo),
            _ => None,
        }
    }

    pub const fn code(&self) -> &'static str {
        match self {
            Language::EnGb => "en-gb",
            Language::EnUs => "en-us",
            Language::Ar => "ar",
            Language::Zh => "zh",
            Language::Fa => "fa",
            Language::Fr => "fr",
            Language::Ka => "ka",
            Language::De => "de",
            Language::El => "el",
            Language::Gu => "gu",
            Language::Ha => "ha",
            Language::Hi => "hi",
            Language::Ig => "ig",
            Language::Id => "id",
            Language::Xh => "xh",
            Language::Zu => "zu",
            Language::It => "it",
            Language::Lv => "lv",
            Language::Ms => "ms",
            Language::Mr => "mr",
            Language::Nso => "nso",
            Language::Pt => "pt",
            Language::Qu => "qu",
            Language::Ro => "ro",
            Language::Ru => "ru",
            Language::Tn => "tn",
            Language::Es => "es",
            Language::Sw => "sw",
            Language::Tg => "tg",
            Language::Ta => "ta",
            Language::Tt => "tt",
            Language::Te => "te",
            Language::Tpi => "tpi",
            Language::Tk => "tk",
            Language::Ur => "ur",
            Language::Yo => "yo",
        }
    }

    pub const fn target_languages(&self) -> Option<&'static [Language]> {
        let en_to_x = &[
            lang!(ar),
            lang!(de),
            lang!(el),
            lang!(es),
            lang!(fa),
            lang!(ha),
            lang!(hi),
            lang!(id),
            lang!(ig),
            lang!(it),
            lang!(ka),
            lang!(mr),
            lang!(ms),
            lang!(nso),
            lang!(pt),
            lang!(ro),
            lang!(ru),
            lang!(tg),
            lang!(tk),
            lang!(tn),
            lang!(tpi),
            lang!(tt),
            lang!(xh),
            lang!(yo),
            lang!(zh),
            lang!(zu),
        ];
        let en = &[lang!(en - gb), lang!(en - us)];
        match self {
            Language::EnGb => Some(en_to_x),
            Language::EnUs => Some(en_to_x),
            Language::Ar => Some(en),
            Language::Zh => Some(en),
            Language::De => Some(en),
            Language::El => Some(en),
            Language::Ha => Some(en),
            Language::Hi => Some(en),
            Language::Id => Some(en),
            Language::Xh => Some(en),
            Language::Zu => Some(en),
            Language::It => Some(en),
            Language::Ms => Some(en),
            Language::Mr => Some(en),
            Language::Nso => Some(en),
            Language::Pt => Some(en),
            Language::Qu => Some(&[lang!(es)]),
            Language::Ru => Some(en),
            Language::Tn => Some(en),
            Language::Es => Some(&[lang!(en - gb), lang!(en - us), lang!(qu)]),
            Language::Tt => Some(en),
            Language::Te => Some(en),
            Language::Tpi => Some(en),
            Language::Tk => Some(en),
            Language::Ur => Some(en),
            _ => None,
        }
    }

    pub const fn entries_api(&self) -> bool {
        match self {
            Language::EnGb => true,
            Language::EnUs => true,
            Language::Fr => true,
            Language::Gu => true,
            Language::Hi => true,
            Language::Lv => true,
            Language::Ro => true,
            Language::Es => true,
            Language::Sw => true,
            Language::Ta => true,
            _ => false,
        }
    }

    // pub const fn words_api(&self) -> bool {
    //     todo!()
    // }
    // pub const fn inflections_api(&self) -> bool {
    //     todo!()
    // }
    // pub const fn lemmas_api(&self) -> bool {
    //     todo!()
    // }
    // pub const fn search_api(&self) -> bool {
    //     todo!()
    // }
    // pub const fn search_translations_api(&self) -> bool {
    //     todo!()
    // }
    // pub const fn search_thesaurus_api(&self) -> bool {
    //     todo!()
    // }
    // pub const fn translations_api(&self) -> bool {
    //     todo!()
    // }
    // pub const fn thesaurus_api(&self) -> bool {
    //     todo!()
    // }
    // pub const fn sentences_api(&self) -> bool {
    //     todo!()
    // }
}
