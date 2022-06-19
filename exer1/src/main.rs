use std::collections::HashMap;

struct Student<T> {
    name: String,
    score: T
}

pub struct School<T> {
    students: HashMap<String, Student<T>>
}

impl<T> School<T>
where T: Clone + Ord,
{
    pub fn new(&mut self) -> School<T> {
        School::<T>{
            students: HashMap::new();
        }
    }

    pub fn add(&mut self, student: Student<T>) {
        self.students.insert(String::from(self.students.len()), student);
    }

    pub fn grades(&self) -> Vec<T> {
        let stus: Vec<Student<T>> = &self.students.values().clone().collect();
        let mut grads: Vec<T> = Vec::<T>::new();
        for s in &stus {
            grads.push(s.score);
        }
        grads.sort();
        grads.dedup();
        grads
    }


    pub fn grade(&self, grade: T) -> Vec<String> {
        let stus = self.students;
        let mut rs = Vec::<String>::new();

        for s in stus.inter() {
            if s.grade == grade {
                rs.push(s.name.toString());
            }
        }
        rs.sort();
        rs
    }
}

fn main() {
    let stu1 = Student{name: String::from("A"), score: 1};
    let stu2 = Student{name: String::from("B"), score: 5};
    let stu3 = Student{name: String::from("C"), score: 2};
    let stu4 = Student{name: String::from("D"), score: 3};
    let stu5 = Student{name: String::from("E"), score: String::from("A+")};

    let school = School::new();
    school.students.insert(stu1);
    school.students.insert(stu2);
    school.students.insert(stu3);
    school.students.insert(stu4);
    school.students.insert(stu5);
}