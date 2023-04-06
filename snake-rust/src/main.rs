use rand::{self, Rng};
use std::process::Command;
use console::Term;

enum Message {
    Sucess,
    Error(i32), // return a Error code
    Warning(Box<str>), // send a Warning
    Message(Box<str>) // send a message to the user
} impl Message {
    fn message_handler(&self) { //will handle the messages
        match self {
            Message::Sucess => println!("Sucess"),
            Message::Error(error_code) => println!("Error Code : {} refer to the documentation for more information", error_code),
            Message::Warning(str) => println!("Warning : {}", str),
            Message::Message(str) => println!("System Message : {}", str)
        }
    }   
}

enum orientaion {
    Up,
    Down,
    Right,
    Left
}

struct Player {
    life : u32,
    lenght : u32,
    color : (u32, u32, u32),
    location : (usize , usize),
    orientaion : orientaion
}


fn init_game() -> ([[char; 40]; 20] , Player, u32) {
    let coin_num :u32 = 10; 
    let mut score :u32  = 0;
    
    Message::Message("initialisation of the game ...".into()).message_handler();
    
    print!("loading screen ... : ");
    const rows: usize = 40;
    const lines: usize = 20;
    let screen = [[' ';rows];lines];
    Message::Sucess.message_handler();
    
    print!("loading player ... : ");
    let player = Player{
        life: 3,
        lenght: 2,
        color: (255, 0, 0),
        location: (5, 5),
        orientaion: orientaion::Down
    };
    Message::Sucess.message_handler();
    return (screen, player, score);
}

fn restart() {
    println!("restarting ... : ");
    Message::Sucess.message_handler();
    main();
}

fn load_coins(mut window : [[char ; 40]; 20]) -> [[char ; 40]; 20] {
    let mut placed : bool = false;
    while !placed {
        let x = rand::thread_rng().gen_range(0..40);
        let y = rand::thread_rng().gen_range(0..20);
        if window[y][x] == ' ' {
            window[y][x] = 'C';
            placed = true;
        }}
    return window;
}


fn main() {
    // first we will simulate a laoding
    
    let (mut window, mut player, mut score) = init_game(); 
    let mut blanck_window = load_coins(window);
    let stdout = Term::buffered_stdout(); 

    'game_loop: loop {
        let mut child = Command::new("sleep").arg("0.1").spawn().unwrap();
        let _result = child.wait().unwrap();
        window = blanck_window;
        if let Ok(character) = stdout.read_char() {
            match character {
                'z' => player.orientaion = orientaion::Up,
                's' => player.orientaion = orientaion::Down,
                'q' => player.orientaion = orientaion::Left,
                'd' => player.orientaion = orientaion::Right,
                'r' => restart(),
                _ => ()
            }
        }
        match window[player.location.1][player.location.0] {
            'C' => {blanck_window[player.location.1][player.location.0] = ' '; score += 1; println!("got one"); player.lenght += 1;},      
            _ => () 
        }
        window[player.location.1][player.location.0] = '#';
        print!("{}[2J", 27 as char);
        println!("____________________________________________");
        for line in window {
            print!("|");
            for case in line {
                print!("{}", case);
            }
            println!("|");
        }
        println!("--------------------------------------------");
        println!("score : {}", score);
    }
}
