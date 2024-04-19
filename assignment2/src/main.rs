struct Student{
    name: String,
    class: String,
    grades: Vec<u32>,

}

impl Student {
    fn new(name: String, class: String, grades: Vec<u32>) -> Self {
        Student {
            name,
            class,
            grades,
        }
    }

    fn average_grade(&self) -> f64 {
        if self.grades.is_empty() {
            return 0.0;
        }

        let sum: u32 = self.grades.iter().sum();
        sum as f64 / self.grades.len() as f64

    }

    fn danish_scale(&self) -> Option<u32> {
        if self.grades.is_empty() {
            return None;
        }
        let average = self.average_grade();
        if average >= 10.5 {
            Some(12)
        } else if average >= 9.5 {
            Some(10)
        } else if average >= 7.5 {
            Some(7)
        } else if average >= 4.0 {
            Some(4)
        } else {
            Some(2)
        }
    }
}

fn main() {
    let student = Student::new(
        String::from("John Doe"),
        String::from("Mathematics"),
        vec![10, 12, 7, 4, 2],

    );

    println!("Student: {}", student.name);
    println!("Class: {}", student.class);
    println!("Grades: {:?}", student.grades);
    println!("Average Grade: {:.2}", student.average_grade());
    if let Some(grade) = student.danish_scale() {
        println!("Danish Scale Grade: {}", grade);
    } else {
        println!("No grades available to calculate Danish Scale Grade.");
    }
}