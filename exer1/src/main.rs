use std::collections::HashMap;
#[derive(Debug)]
pub struct School<T> {
    students: HashMap<String, T>
}

impl<T: Ord + Copy> School<T> {
    pub fn new() -> School<T> {
        School {
            students: HashMap::new()
        }
    }

    pub fn add(&mut self, grade: T, student: &str) {
        self.students.insert(student.to_string(), grade);
    }

    pub fn grades(&self) -> Vec<T> {
        let mut res: Vec<T> = Vec::new();
        for (_, value) in self.students.iter() {
            res.push(*value);
        }
        res.sort();
        res.dedup();
        res
    }


    pub fn grade(&self, grade: T) -> Vec<String> {
        let  mut res: Vec<String> = Vec::new();
        for (key, value) in self.students.iter() {
            if *value == grade {
                res.push(key.to_string());
            }
        }
        res.sort();
        res
    }
}

fn main() {
    let mut school = School::new();
    school.add(9, "Alice");
    school.add(8, "Bob");
    school.add(8, "Pi");
    println!("school empty: {:?}", school);

    let grades = school.grades();
    println!("grades: {:?}", grades);

    let names = school.grade(8);
    println!("grade: {:?}", names);
}