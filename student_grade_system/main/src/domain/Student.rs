use std::fmt;

pub struct Student {
    name: String,
    student_id: u32,
    score_list: Vec<u32>,
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.student_id)
    }
}

// 두 개 impl 가능하군
impl Student {
    pub fn new(name: String, student_id: u32, score_list: Vec<u32>) -> Self {
        Self {
            name,
            student_id,
            score_list,
        }
    }

    pub fn average_score(&self) -> f32 {
        if self.score_list.is_empty() {
            return 0.0;
        }

        let sum: u32 = self.score_list.iter().sum();
        sum as f32 / self.score_list.len() as f32
    }

    pub fn max_score(&self) -> Option<u32> {
        self.score_list.iter().copied().max()
    }

    pub fn min_score(&self) -> Option<u32> {
        self.score_list.iter().copied().min()
    }
}
