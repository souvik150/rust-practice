use std::ops::Add;


// struct Point<T, U> {
//     x: T,
//     y: U,
// }

trait Overview {
    fn overview(&self) -> String {
        String::from("No overview available")
    }
}

struct Course {
    headline: String,
    author: String,
}

impl Clone for Course {
    fn clone(&self) -> Self {
        println!("Cloning Course");
        Course {
            headline: self.headline.clone(),
            author: self.author.clone(),
        }
    }
}

impl Clone for AnotherCourse {
    fn clone(&self) -> Self {
        println!("Cloning AnotherCourse");
        AnotherCourse {
            headline: self.headline.clone(),
            author: self.author.clone(),
        }
    }
}

impl From<Course> for AnotherCourse {
    fn from(course: Course) -> Self {
        println!("Converting Course to AnotherCourse");
        AnotherCourse {
            headline: course.headline,
            author: course.author,
        }
    }
}

impl Into<Course> for AnotherCourse {
    fn into(self) -> Course {
        println!("Converting AnotherCourse to Course");
        Course {
            headline: self.headline,
            author: self.author,
        }
    }
}

struct AnotherCourse {
    headline: String,
    author: String,
}

impl Overview for Course {}

impl Overview for AnotherCourse {
    fn overview(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

#[derive(Debug)]
struct Point<T>{
    x: T,
    y: T,
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {

    let coord = Point { x: 10, y: 20 };
    let coord2 = Point { x: 10, y: 20 };
    //let coord3 = Point { x: 10, y: 20.0 };

    let sum = coord + coord2;
    println!("Sum of coord and coord2 is: {:?}", sum);


    let course1 = Course {
        headline: String::from("Rust"),
        author: String::from("Souvik"),
    };

    let course2 = AnotherCourse {
        headline: String::from("New Rust"),
        author: String::from("Souvik"),
    };

    println!("Course1: {}", course1.overview());
    println!("Course2: {}", course2.overview());

    call_overview(&course1);
    call_overview(&course2);

    // CLONE

    let course3 = course1.clone();
    println!("Course3: {}", course3.overview());

    // FROM and INTO

    let course4 = AnotherCourse::from(course1.clone());
    println!("Course4: {}", course4.overview());

    let course5: Course = course2.clone().into();
    println!("Course5: {}", course5.overview());

    // DROP

    drop(course1);
    drop(course2);

    //println!("Course1: {}", course1.overview());
    // Above will not work as course1 has been dropped

    
}

fn call_overview(item: &impl Overview) {
    println!("Overview: {}", item.overview());
}
