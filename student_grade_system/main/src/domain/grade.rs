pub enum Grade {
    A = 90,
    B = 80,
    C = 70,
    D = 60,
    E = 50,
    F = 40
}

impl Grade {
    pub fn from_score(score: u32) -> Grade {
        match score {
            90..100 => Grade::A,
            80..90 => Grade::B,
            70..80 => Grade::C,
            60..70 => Grade::D,
            50..60 => Grade::E,
            // kotlin when처럼 굉장히 exhaustive하군
            _ => Grade::F,
        }
    }
}