use reslt::prelude::*;

fn main() {
    match extract_classes_from_css("tailwind-safelist.txt") {
        Ok(_) => println!("Safelist generated successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
