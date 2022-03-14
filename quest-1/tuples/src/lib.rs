#[derive(Debug)]
pub struct Student(pub i32, pub String, pub String);

impl Student {}

pub fn id(student: &Student) -> i32 {
    student.0
}

pub fn first_name(student: &Student) -> String {
    student.1.to_string()
}

pub fn last_name(student: &Student) -> String {
    student.2.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_id() {
        let student = Student(20, "Pedro".to_string(), "Domingos".to_string());

        assert_eq!(20, id(&student))
    }

    #[test]
    fn get_name() {
        let student = Student(20, "Pedro".to_string(), "Domingos".to_string());

        assert_eq!("Pedro".to_string(), first_name(&student))
    }

    #[test]
    fn get_surname() {
        let student = Student(20, "Pedro".to_string(), "Domingos".to_string());

        assert_eq!("Domingos".to_string(), last_name(&student))
    }

    
}
