use std::cmp::max;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str;

fn convert(a:&[u8]) -> [u8;5]{
    let mut b = [0,0,0,0,0];

    b[0] = a[0];
    b[1] = a[1];
    b[2] = a[2];
    b[3] = a[3];
    b[4] = a[4];

    return b;
}

fn read_words_as_bytes(path:&str) -> Vec<[u8;5]>{
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut words = Vec::new();

    for line in reader.lines() {
        let word:[u8;5] = convert(line.unwrap().as_bytes());
        words.push(word);
    }
    return words;
}

struct GuessData{
    correct : Vec<(usize,u8)>,
    extent : Vec<u8>,
    incorrect : Vec<u8>,
}

impl GuessData{

    fn make() -> GuessData{
        let correct = Vec::new();
        let extent = Vec::new();
        let incorrect =Vec::new();

        return GuessData{correct, extent, incorrect};
    }

    fn calculate_result(self: &mut Self,guess_word:&[u8;5], target_word:&[u8;5]) -> (){

        for index in 0..5 {
            let letter = guess_word[index];

            if target_word.contains(&letter){
                if target_word[index] == letter{
                    self.correct.push((index, letter));
                }else{
                    self.extent.push(letter);
                }
            }else{
                self.incorrect.push(letter);
            }
        }
    }

    fn check_word(self: &Self, word:&[u8;5])-> bool{

        for (index, letter) in self.correct.iter(){
            if word[*index] != *letter{
                return false;
            }
        }

        for letter in self.extent.iter(){
            if !word.contains(&letter){
                return false;
            }
        }

        for letter in self.incorrect.iter(){
            if word.contains(&letter){
                return false;
            }
        }

        return true;
    }

    fn count_valid(self: &Self, target_dictionary: &Vec<[u8;5]>) -> usize{
        target_dictionary.iter().filter(|&word| self.check_word(word)).count()
    }

    fn refine_targets(self: &Self, target_dictionary: &Vec<[u8;5]>) -> Vec<[u8;5]>{
        target_dictionary.clone().into_iter().filter(|&word| self.check_word(&word)).collect::<Vec<[u8;5]>>()
    }

}


fn check_guess_against_targets(guess_word: &[u8;5], target_dictionary:&Vec<[u8;5]>) -> (usize, [u8;5]){
    let mut current_score = 0;
    let mut current_word:[u8;5] = [0,0,0,0,0];
    for target_word in target_dictionary.iter(){

        let mut g = GuessData::make();
        g.calculate_result(&guess_word, &target_word);
        let new_count = g.count_valid(&target_dictionary);

        if current_score <= new_count{
            current_score = new_count;
            current_word = target_word.clone();

        }
    }

    return (current_score, current_word);
}

fn best_guess(guess_dictionary:&Vec<[u8;5]>, target_dictionary:&Vec<[u8;5]>) -> (usize, [u8;5]){

    let mut best_word = [0,0,0,0,0];
    let mut best_score = target_dictionary.len();

    for guess_word in guess_dictionary.iter(){

        let mut current_score = 0;

        for target_word in target_dictionary.iter(){

            let mut g = GuessData::make();
            g.calculate_result(&guess_word, &target_word);
            current_score = max(current_score, g.count_valid( &target_dictionary));
        }

        if current_score <= best_score{
            best_score = current_score;
            best_word = *guess_word;
        }

    }

    return (best_score, best_word);
}


fn main() -> io::Result<()> {

    let guess_dictionary = read_words_as_bytes("allowed_guesses.txt");
    let target_dictionary = read_words_as_bytes("allowed_answers.txt");

    // best first guess is known
    let first_guess = convert("arise".as_bytes());

    for (index, target) in target_dictionary.iter().enumerate(){

        // println!("Starting game with correct word {}", str::from_utf8(&target.clone()).unwrap());

        let mut guess = GuessData::make();
        let mut targets = target_dictionary.clone();

        for i in 1..6{

            let guessed_word = match i {
                1 => first_guess,
                _ => best_guess(&guess_dictionary, &targets).1
            };

            guess.calculate_result(&guessed_word, &target);

            targets = guess.refine_targets(&targets);

            // println!("Guessed {}", str::from_utf8(&guessed_word).unwrap());

            if targets.len() <= 1{
                // println!("Solved problem with answer {}", str::from_utf8(&targets[0]).unwrap());
                break;
            }

            if i == 6 && targets.len() > 1 {
                println!("FAILED THE GAME!!!!!!!!!!");
            }
        }
        if index % 100 == 0{
            println!("{}/ {}",index, target_dictionary.len());
        }
    }


    let guess_tuple = best_guess(&guess_dictionary, &target_dictionary);

    let best_word = guess_tuple.1;
    let best_score = guess_tuple.0;

    println!("{}", str::from_utf8(&best_word).unwrap());

    Ok(())
}