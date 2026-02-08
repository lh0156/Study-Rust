mod domain;
use crate::domain::student::Student;

/**
**과제 1: 학생 성적 관리 시스템**
아래 요구사항을 충족하는 프로그램을 작성하세요.

- Student struct를 정의하세요 (이름: String, 학번: u32, 점수 목록: Vec<u32>) (Done)
- Grade enum을 정의하세요 (A, B, C, D, F 각 variant에 최소 점수 기준을 포함) (Done)
- 점수를 Grade로 변환하는 함수 to_grade(score: u32) -> Grade를 구현하세요 (Done)
- Student에 대해 fmt::Display를 직접 구현하여, 아래 형식으로 출력되도록 하세요 (Done)
- Student에 평균 점수, 최고 점수, 최저 점수를 계산하는 메서드를 추가하세요
- 여러 Student를 Vec에 저장하고, 평균 점수 기준 내림차순으로 정렬하여 출력하세요
**/

fn main() {
    let yunseop = Student::new("윤섭".to_string(), 20240002, vec![95, 92, 88]);
    let busik = Student::new("부식".to_string(), 20240001, vec![100, 100, 100]);
    let mut students = vec![busik, yunseop];

    students.sort_by(|a, b| b.average_score().total_cmp(&a.average_score()));

    println!("Average score descending:");
    for student in &students {
        println!(
            "{} | avg: {:.2}, max: {:?}, min: {:?}",
            student,
            student.average_score(),
            student.max_score(),
            student.min_score()
        );
    }
}
