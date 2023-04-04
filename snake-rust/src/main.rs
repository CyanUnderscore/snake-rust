use console::Term;

struct Player {
    len : i32,
    color : (i32, i32, i32),

}

fn init() {
   window  = [[" "; 10]; 10]; 
}


fn main() {
    if let Ok(character) = stdout.read_char() {
            match character {
                'z' => println!("up"),
                's' => println!("down"),
                'q' => println!("left"),
                'd' => println!("right"),
                _ => ()
            }
        }
}
