use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use rand::Rng;

use dialoguer::Input;
use colored::Colorize; 
 
mod legal_words; 

const WORD_LENGTH : usize = 5; 

#[derive(Copy, Clone)]
enum Color {
  White,
  Green, 
  Yellow, 
}

fn main() {
  
  //list of words to choose from 
  let mut legal_answers: Vec<&str> = legal_words::LEGAL_ANSWERS.to_vec(); 
  let mut legal_guesses : Vec<&str> = legal_words::LEGAL_GUESSES.to_vec();

  // randomly pick
  let answer : &str = random_pick(legal_answers.clone());

  legal_guesses.append(&mut legal_answers); 
  let legal_set = vec_to_set(legal_guesses.clone()); 

  // game loop
  game_loop(answer, legal_set);
}


fn vec_to_set(vec: Vec<&str>) -> HashSet<&str> {
    HashSet::from_iter(vec)
}

// fn arr_to_set(arr: [&str; 8]) -> HashSet<&str>  {
//   HashSet::from_iter(arr)
// }

fn random_pick(legal_words : Vec<&str>) -> &str {
  let rng = rand::thread_rng().gen_range(0..legal_words.len());
  return legal_words[rng as usize];
}

fn game_loop( answer : &str, legal_set : HashSet<&str> ) {
  let mut num_tries = 6; 
  let mut dead_letters : HashSet<char> = HashSet::new();
  // do while format
  while { 
    println!("dead letters: {:?}", dead_letters);

    let input : String = Input::new()
      .with_prompt("Guess a 5 letter word")
      .interact_text()
      .unwrap(); 
    let guess  = input.trim().to_lowercase(); // trim and lowercase
    let guess: &str = guess.as_str(); 
    let mut answer_freq : HashMap<u8, i8> = build_frequency(answer); 
    
    if guess.len() != WORD_LENGTH {
      println!("{} is not a 5 letter word. Try again!", guess.to_string().red());
    } else if !legal_set.contains(&guess) {
      println!("{} is not a valid 5 letter word. Try again!", guess.to_string().red());
    }
    else if guess != answer {
      num_tries -= 1; 
      let colors : [(Color,u8);5] = color_code(guess, answer, &mut answer_freq); 
      
      for (color_code, byte) in colors {
        if !answer_freq.contains_key(&byte) {
          dead_letters.insert(byte as char);
        }
        match color_code {
          Color::Yellow => print!("{}", (byte as char).to_string().yellow() ),
          Color::Green => print!("{}", (byte as char).to_string().green() ),
          Color::White => print!("{}", (byte as char) ),
        }
      }
      println!(" is incorrect. {} tries left.", num_tries.to_string().blue());
    } else { 
      println!("");
      println!("{} is correct. You win!", guess.to_string().green())
    }

    if num_tries == 0 {
      println!("");
      println!("{} the answer was {}", "you lose".to_string().red(), answer.to_string().red());
    }
    guess != answer && num_tries > 0
  } { }
}

//(color_code, byte)
fn color_code(guess : &str, answer : &str, answer_freq : &mut HashMap<u8, i8>) -> [(Color, u8); WORD_LENGTH] {
  let guess_bytes = guess.as_bytes(); 
  let answer_bytes = answer.as_bytes(); 

  let mut colors : [(Color,u8);WORD_LENGTH] = [(Color::White,0);WORD_LENGTH]; 

  let mut i : usize = 0; 
  //green
  while i < WORD_LENGTH {
    if guess_bytes[i] == answer_bytes[i] {
      colors[i] = (Color::Green, guess_bytes[i]); 
      *answer_freq.entry(guess_bytes[i]).or_default() -= 1; 
    } 
    i+=1;
  }
  //yellow
  let mut j : usize = 0; 
  while j < WORD_LENGTH {
    match colors[j] {
      (Color::White,_) => {
        match answer_freq.get(&guess_bytes[j]) {
          Some(p) => if p > &0_i8 {
            colors[j] = (Color::Yellow, guess_bytes[j]); 
            *answer_freq.entry(guess_bytes[j]).or_default() -= 1; 
          } else {
            colors[j] = (Color::White, guess_bytes[j]); 
          },
          None => {
            colors[j] = (Color::White,guess_bytes[j]);
          },
        }
      },
      _ => (),
    }
    j+=1;
    
  }  
  return colors;
}

fn build_frequency(str: &str) -> HashMap<u8, i8> {
  let mut frequency : HashMap<u8, i8> = HashMap::new(); 
  for byte in str.bytes() {
    *frequency.entry(byte).or_insert(0) += 1;
  }
  return frequency;
}
