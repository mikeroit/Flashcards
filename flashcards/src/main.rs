extern crate rustbox;
extern crate rand;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::default::Default;
use std::error::Error;

use rustbox::{Color, RustBox};
use rustbox::Key;


//struct to represent a flashcard
#[derive(Debug)]
struct Flashcard {
    term: String,
    definition: String
}

impl Flashcard {
    //construcotr for flashcard
    pub fn new(t: String, d: String) -> Flashcard {
        Flashcard {
            term: t,
            definition: d
        }
    }

    //method to clone a flashcard (returns a copy)
    pub fn clone_card(&self) -> Flashcard {
        Flashcard {
            term: self.term.to_owned(),
            definition: self.definition.to_owned(),
        }
    }
}

//function to read flashcards from a file
//file should separate term and definition using '-'
fn read_cards(input: &str) -> Vec<Flashcard> {

    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let mut res: Vec<Flashcard> = vec!();

    for l in reader.lines() {
        let line = l.unwrap();
        let splits: Vec<&str> = line.split("-").collect();
        res.push(Flashcard::new(String::from(splits[0]), String::from(splits[1])));
    }

    res
}

fn main() {

	//init rustbox
    //we use rustbox to display our stud session
	let rustbox = match RustBox::init(Default::default()) {
		Result::Ok(v) => v,
		Result::Err(e) => panic!("{}", e),
    };
	
    //data structure to hold cards
    let mut cards: Vec<Flashcard> = vec!();

    //read cards from files
    for i in 0..std::env::args().len() {
        if i != 0 {
            cards.append(&mut read_cards(&std::env::args().nth(i).unwrap()));

        }
    }

    //counters to track progress
    let mut correct = 0; 
    let mut remaining = cards.len();

    //run study session until all the cards are removed
    while ! cards.is_empty() {
        //at first, we don't want to display the definition
        let mut show_answer = false;

		//grab a random card to display
        let i = rand::random::<usize>() % cards.len(); 
        let card = cards[i].clone_card();

		//display the card

		loop {

            //display the card
            rustbox.clear();
            rustbox.print(1, 1, rustbox::RB_BOLD, Color::Default, Color::Default, "USAGE: (q)=quit  (w)=wrong  (r)=right (<space>)=toggle answer"); 
            rustbox.print(1, 3, rustbox::RB_BOLD, Color::Default, Color::Default, "-------------------------------------------------------------"); 
            rustbox.print(1, 5, rustbox::RB_BOLD, Color::Green, Color::Black, &card.definition); 
            if show_answer { rustbox.print(1, 7, rustbox::RB_BOLD, Color::Blue, Color::Black, &card.term); }
            rustbox.print(50, 5, rustbox::RB_BOLD, Color::Yellow, Color::Black, &format!("Correct: {}", correct));
            rustbox.print(50, 7, rustbox::RB_BOLD, Color::Yellow, Color::Black, &format!("Remaining: {}", remaining));
            rustbox.present();

            match rustbox.poll_event(false) {
				Ok(rustbox::Event::KeyEvent(key)) => {
                    match key {
                        Key::Char('q') => { panic!("quit key"); }

                        Key::Char('w') => {
                            break;
                        }

                        Key::Char('r') => {
                            remaining -= 1;
                            correct += 1;
                            cards.remove(i);
                            break;
                        }

                        Key::Char(' ') => {
                            show_answer = ! show_answer;
                        }

                        _ => { }
                    }
                },

                Err(e) => panic!("{}", e.description()),
                _ => { }
			}
		}

	}

}
