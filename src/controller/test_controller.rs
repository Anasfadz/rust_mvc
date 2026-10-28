use crate::model::student;

pub async fn index() -> String {
    let students = student::Student::get_all().await;
    format!("{:?}", students)
}

pub async fn create_student() -> String {
    let student = student::Student::create(
        "Anas".to_string(),
        "anas@gmail.com".to_string(),
        "10th".to_string(),
        16,
    ).await;
    format!("{:?}", student)
}