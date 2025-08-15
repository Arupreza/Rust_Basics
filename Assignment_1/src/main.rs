// use std::fmt::Display;
// #[derive(Debug)]

trait Course {
    fn get_overview(&self) -> Result <String, &'static str>;
}

// #[derive(Debug)]
struct Workshop {
    title: String,
    instructor: String,
    duration: String, // in hours
}

// #[derive(Debug)]
struct Seminar {
    title: String,
    speaker: String,
    location: String,
}

enum CourseType {
    Workshop(Workshop),
    Seminar(Seminar),
}

impl Course for CourseType {
    fn get_overview(&self) -> Result <String, &'static str> {
        match self {
            CourseType::Workshop(workshop) => {
                if workshop.title.is_empty() || 
                   workshop.instructor.is_empty() || 
                   workshop.duration.is_empty() {
                    Err("Workshop details are incomplete")
                } else {
                    Ok(format!("Workshop: {}, Instructor: {}, Duration: {} hours", 
                        workshop.title, workshop.instructor, workshop.duration))
                }
            }

            CourseType::Seminar(seminar) => {
                if seminar.title.is_empty() || 
                   seminar.speaker.is_empty() || 
                   seminar.location.is_empty() {
                    Err("Seminar details are incomplete")
                } else {
                    Ok(format!("Seminar: {}, Speaker: {}, Location: {}", 
                        seminar.title, seminar.speaker, seminar.location))
                }
            }
        }
    }
}

fn print_course_overview<T: Course>(course: T) {
    match course.get_overview() {
        Ok(overview) => println!("{}", overview),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let workshop = Workshop {
        title: "Rust Programming Workshop".to_owned(),
        instructor: "Alice Smith".to_owned(),
        duration: "3".to_owned(),
    };
    
    let seminar = Seminar {
        title: "Advanced Rust Seminar".to_owned(),
        speaker: "Bob Johnson".to_owned(),
        location: "Room 101".to_owned(),
    };
    
    let course_1 = CourseType::Workshop(workshop);
    let course_2 = CourseType::Seminar(seminar);
    
    print_course_overview(course_1);
    print_course_overview(course_2);
}





// // use std::fmt::Display;

// // #[derive(Debug)]
// trait Course {
//     fn get_overview(&self) -> String;
// }

// // #[derive(Debug)]
// struct Workshop {
//     title: String,
//     instructor: String,
//     duration: String // in hours
// }

// // #[derive(Debug)]
// struct Seminar {
//     title: String,
//     speaker: String,
//     location: String,
// }

// enum CourseType {
//     Workshop(Workshop),
//     Seminar(Seminar),
// }

// impl Course for Workshop{
//     fn get_overview(&self) -> String {
//         format!("Workshop: {}, Instructor: {}, Duration: {} hours", self.title, self.instructor, self.duration)
//     }
// }


// impl Course for Seminar {
//     fn get_overview(&self) -> String {
//         format!("Seminar: {}, Speaker: {}, Location: {}", self.title, self.speaker, self.location)
//     }
// }

// fn print_course_overview <T:Course> (course:T){
//     println!("{}", course.get_overview());
// }


// fn main() {
//     let workshop = Workshop{title: "Rust Programming Workshop".to_owned(),
//                             instructor: "Alice Smith".to_owned(),
//                             duration: "3".to_owned()};

//     let seminar = Seminar{title: "Advanced Rust Seminar".to_owned(),
//                           speaker: "Bob Johnson".to_owned(),
//                           location: "Room 101".to_owned()};

//     print_course_overview(workshop);
//     print_course_overview(seminar);
// }