extern crate rand;
use owo_colors::OwoColorize;
use rand::prelude::*;
use std::io::{self, BufReader, Read, Write};

struct Game {
    player: Choice,
    computer: Choice
}

enum Choice {
    Rock(String),
    Paper(String),
    Scissors(String),
}

#[derive(PartialEq, Debug)]
enum Outcome {
    Win(String, String),
    Lose(String, String),
    Draw(String, String),
}

impl Game {
    fn new (player: Choice, computer: Choice) -> Game {
        Game {player, computer}
    }

    fn compare(self) -> Outcome {
        match (self.player, self.computer) {
            (Choice::Rock(p), Choice::Scissors(c)) => Outcome::Win(p, c),
            (Choice::Rock(p), Choice::Paper(c)) => Outcome::Lose(p, c),
            (Choice::Paper(p), Choice::Rock(c)) => Outcome::Win(p, c),
            (Choice::Paper(p), Choice::Scissors(c)) => Outcome::Lose(p, c),
            (Choice::Scissors(p), Choice::Paper(c)) => Outcome::Win(p, c),
            (Choice::Scissors(p), Choice::Rock(c)) => Outcome::Lose(p, c),
            (Choice::Rock(p), Choice::Rock(c)) => Outcome::Draw(p, c),
            (Choice::Paper(p), Choice::Paper(c)) => Outcome::Draw(p, c),
            (Choice::Scissors(p), Choice::Scissors(c)) => Outcome::Draw(p, c),
        }
    }
}

impl Choice {
    fn get_choice(num: u8) -> Option<Choice> {
        match num {
            1 => Some(Choice::Rock("Камінь".to_string())),
            2 => Some(Choice::Scissors("Ножиці".to_string())),
            3 => Some(Choice::Paper("Папір".to_string())),
            _ => None
        }
    }
}

fn main() {
    loop {
        println!("Введіть число (1: Камінь, 2: Ножиці, 3: Папір):");
        let mut buffer = [0; 1];
        
        io::stdout().flush().unwrap(); // Очистка вихідного буфера

        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin);

        match reader.read(&mut buffer) {
            Ok(size) => {
                let input = buffer[..size][0] as char;
                let input = match input.to_digit(10) {
                    Some(n) => n as u8,
                    None => {
                        println!("Не число");
                        continue;
                    },
                };
                   
                let player = match Choice::get_choice(input) {
                    Some(s) => s,
                    None => {
                        println!("Не правильний символ");
                        continue;
                    }
                };

                let rng = thread_rng().gen_range(1..4);
                let computer = match Choice::get_choice(rng) {
                    Some(s) => s,
                    None => {
                        println!("Не правильний символ");
                        continue;
                    }
                };

                let game = Game::new(player, computer);

                match game.compare() {
                    Outcome::Win(p, c) => println!("{}. Ви: {}, Комп'ютер: {}", "Ви виграли".green().bold(), p, c),
                    Outcome::Lose(p, c) => println!("{}. Ви: {}, Комп'ютер: {}", "Ви програли".red().bold(), p, c),
                    Outcome::Draw(p, c) => println!("{}. Ви: {}, Комп'ютер: {}", "Нічия".yellow().bold(), p, c),
                }
            },
            Err(e) => {
                println!("Помилка: {}", e);
                break;
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_win_scissors_paper() {
        assert_eq!(
            Game::new(Choice::Scissors(String::new()), Choice::Paper(String::new()))
            .compare(), 
            Outcome::Win(String::new(), String::new())
        );
    }

    #[test]
    fn test_win_paper_rock() {
        assert_eq!(
            Game::new(Choice::Paper(String::new()), Choice::Rock(String::new()))
            .compare(), 
            Outcome::Win(String::new(), String::new())
        );
    }

    #[test]
    fn test_win_rock_scissors() {
        assert_eq!(
            Game::new(Choice::Rock(String::new()), Choice::Scissors(String::new()))
            .compare(), 
            Outcome::Win(String::new(), String::new())
        );
    }


    #[test]
    fn test_lose_paper_scissors() {
        assert_eq!(
            Game::new(Choice::Paper(String::new()), Choice::Scissors(String::new()))
            .compare(), 
            Outcome::Lose(String::new(), String::new())
        );
    }

    #[test]
    fn test_lose_scissors_rock() {
        assert_eq!(
            Game::new(Choice::Scissors(String::new()), Choice::Rock(String::new()))
            .compare(), 
            Outcome::Lose(String::new(), String::new())
        );
    }

    #[test]
    fn test_lose_rock_paper() {
        assert_eq!(
            Game::new(Choice::Rock(String::new()), Choice::Paper(String::new()))
            .compare(), 
            Outcome::Lose(String::new(), String::new())
        );
    }

    #[test]
    fn test_draw_scissors_scissors() {
        assert_eq!(
            Game::new(Choice::Scissors(String::new()), Choice::Scissors(String::new()))
            .compare(), 
            Outcome::Draw(String::new(), String::new())
        );
    }

    #[test]
    fn test_draw_paper_paper() {
        assert_eq!(
            Game::new(Choice::Paper(String::new()), Choice::Paper(String::new()))
            .compare(), 
            Outcome::Draw(String::new(), String::new())
        );
    }

    #[test]
    fn test_draw_rock_rock() {
        assert_eq!(
            Game::new(Choice::Rock(String::new()), Choice::Rock(String::new()))
            .compare(), 
            Outcome::Draw(String::new(), String::new())
        );
    }
 
}
