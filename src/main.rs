extern crate rand;
use owo_colors::OwoColorize;
use rand::prelude::*;
use std::io::{self, BufReader, Read, Write};

struct Game {
    player: Choice,
    computer: Choice,
}

struct Result {
    choice: Choice,
    name: String,
}

#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Game {
    fn new(player: Choice, computer: Choice) -> Game {
        Game { player, computer }
    }

    fn compare(&self) -> Outcome {
        match (&self.player, &self.computer) {
            (Choice::Rock, Choice::Scissors) => Outcome::Win,
            (Choice::Rock, Choice::Paper) => Outcome::Lose,
            (Choice::Paper, Choice::Rock) => Outcome::Win,
            (Choice::Paper, Choice::Scissors) => Outcome::Lose,
            (Choice::Scissors, Choice::Paper) => Outcome::Win,
            (Choice::Scissors, Choice::Rock) => Outcome::Lose,
            _ => Outcome::Draw,
        }
    }
}

impl Choice {
    fn get_choice(num: u8) -> Option<Result> {
        match num {
            1 => Some(Result {
                choice: Choice::Rock,
                name: "Камінь".to_string(),
            }),
            2 => Some(Result {
                choice: Choice::Scissors,
                name: "Ножиці".to_string(),
            }),
            3 => Some(Result {
                choice: Choice::Paper,
                name: "Папір".to_string(),
            }),
            _ => None,
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
                    }
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

                let game = Game::new(player.choice, computer.choice);

                match game.compare() {
                    Outcome::Win => println!(
                        "{}. Ви: {}, Комп'ютер: {}",
                        "Ви виграли".green().bold(),
                        player.name,
                        computer.name
                    ),
                    Outcome::Lose => println!(
                        "{}. Ви: {}, Комп'ютер: {}",
                        "Ви програли".red().bold(),
                        player.name,
                        computer.name
                    ),
                    Outcome::Draw => println!(
                        "{}. Ви: {}, Комп'ютер: {}",
                        "Нічия".yellow().bold(),
                        player.name,
                        computer.name
                    ),
                }
            }
            Err(e) => {
                println!("Помилка: {}", e);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_win_scissors_paper() {
        assert_eq!(
            Game::new(Choice::Scissors, Choice::Paper).compare(),
            Outcome::Win
        );
    }

    #[test]
    fn test_win_paper_rock() {
        assert_eq!(
            Game::new(Choice::Paper, Choice::Rock).compare(),
            Outcome::Win
        );
    }

    #[test]
    fn test_win_rock_scissors() {
        assert_eq!(
            Game::new(Choice::Rock, Choice::Scissors).compare(),
            Outcome::Win
        );
    }

    #[test]
    fn test_lose_paper_scissors() {
        assert_eq!(
            Game::new(Choice::Paper, Choice::Scissors).compare(),
            Outcome::Lose
        );
    }

    #[test]
    fn test_lose_scissors_rock() {
        assert_eq!(
            Game::new(Choice::Scissors, Choice::Rock).compare(),
            Outcome::Lose
        );
    }

    #[test]
    fn test_lose_rock_paper() {
        assert_eq!(
            Game::new(Choice::Rock, Choice::Paper).compare(),
            Outcome::Lose
        );
    }

    #[test]
    fn test_draw_scissors_scissors() {
        assert_eq!(
            Game::new(Choice::Scissors, Choice::Scissors).compare(),
            Outcome::Draw
        );
    }

    #[test]
    fn test_draw_paper_paper() {
        assert_eq!(
            Game::new(Choice::Paper, Choice::Paper).compare(),
            Outcome::Draw
        );
    }

    #[test]
    fn test_draw_rock_rock() {
        assert_eq!(
            Game::new(Choice::Rock, Choice::Rock).compare(),
            Outcome::Draw
        );
    }
}
