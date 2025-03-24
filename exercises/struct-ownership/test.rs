#[derive(Debug)]
struct CsStudent {
    name: String,
    email: String,
    coffee_addict: bool,
    total_commits: u64,
}

fn main() {
    let student1 = CsStudent {
        name: String::from("Jake"),
        email: String::from("ja250@st-andrews.ac.uk"),
        coffee_addict: true,
        total_commits: 400,
    };

    // This student only uses attributes with the Copy trait
    let student2 = CsStudent {
        name: String::from("Julia"),
        email: String::from("julia@st-andrews.ac.uk"),
        ..student1  // take definitions for other attributes from student1
    };
    println!("Jake:  {student1:?}");
    println!("Julia: {student2:?}");

    // But this student uses student1's email, which does not have the Copy trait
    let student3 = CsStudent {
        name: String::from("Naveen"),
        ..student1
    };
    println!("Jake:   {student1:?}");  // ERROR: student1.email has already moved
    println!("Naveen: {student3:?}");

    // We can fix this by copying email explicitly
    let student4 = CsStudent {
        name: String::from("Naveen"),
        email: student1.email.clone(),
        ..student1
    };
    println!("Jake:   {student1:?}");
    println!("Naveen: {student4:?}");
}
