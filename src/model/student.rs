#[derive(Debug, sqlx::FromRow)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub class_name: String,
    pub age: u32,
}

impl Student {
    pub async fn get_all() -> Vec<Student> {
        let pool = crate::config::db::connect().await.unwrap();
        let students = sqlx::query_as::<sqlx::MySql, Student>("SELECT * FROM students")
            .fetch_all(&pool)
            .await
            .unwrap();
        students
    }

    pub async fn create(name: String, email: String, class_name: String, age: u32) -> Student {
        let pool = crate::config::db::connect().await.unwrap();
        let student = sqlx::query_as::<sqlx::MySql, Student>(
            "INSERT INTO students (name, email, class_name, age) VALUES (?, ?, ?, ?)",
        )
        .bind(name)
        .bind(email)
        .bind(class_name)
        .bind(age)
        .fetch_one(&pool)
        .await
        .unwrap();
        student
    }
}
