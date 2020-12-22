use std::fs::File;
use std::io::prelude::*;

struct Deck {
    content: Vec<usize>,
}

impl Deck {
    fn deck_score(&self) -> usize {
        self.content
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, card)| (idx + 1) * card)
            .sum()
    }
}

#[derive(Copy, Clone)]
enum GameWinner {
    Player1,
    Player2,
}

struct Game {
    player1: Deck,
    player2: Deck,
}

impl Game {
    fn winner(&self) -> Option<GameWinner> {
        if self.player2.content.len() == 0 {
            Some(GameWinner::Player1)
        } else if self.player1.content.len() == 0 {
            Some(GameWinner::Player2)
        } else {
            None
        }
    }

    fn play_round(&mut self) -> Option<GameWinner> {
        let p1_card = self.player1.content[0];
        let p2_card = self.player2.content[0];
        self.player1.content = self.player1.content[1..].to_vec();
        self.player2.content = self.player2.content[1..].to_vec();
        if p1_card > p2_card {
            self.player1.content.push(p1_card);
            self.player1.content.push(p2_card);
        } else {
            self.player2.content.push(p2_card);
            self.player2.content.push(p1_card);
        }
        self.winner()
    }

    fn winner_score(&self) -> usize {
        match self.winner().unwrap() {
            GameWinner::Player1 => self.player1.deck_score(),
            GameWinner::Player2 => self.player2.deck_score(),
        }
    }
}

struct RecursiveGame {
    game: Game,
    previous_states: Vec<(Vec<usize>, Vec<usize>)>,
}

impl RecursiveGame {
    fn new(game: Game) -> RecursiveGame {
        RecursiveGame {
            game,
            previous_states: vec![],
        }
    }

    fn play_round(&mut self) -> Option<GameWinner> {
        let current_state = (
            self.game.player1.content.to_vec(),
            self.game.player2.content.to_vec(),
        );
        if self.previous_states.contains(&current_state) {
            return Some(GameWinner::Player1);
        }
        self.previous_states.push((
            self.game.player1.content.to_vec(),
            self.game.player2.content.to_vec(),
        ));

        let player1_top_card = self.game.player1.content[0];
        let player2_top_card = self.game.player2.content[0];

        let turn_winner;
        let mut game_winner = None;

        if self.game.player1.content.len() > player1_top_card
            && self.game.player2.content.len() > player2_top_card
        {
            let player1_subdeck = self.game.player1.content[1..(player1_top_card + 1)].to_vec();
            let player2_subdeck = self.game.player2.content[1..(player2_top_card + 1)].to_vec();

            let mut subgame = RecursiveGame::new(Game {
                player1: Deck {
                    content: player1_subdeck.to_vec(),
                },
                player2: Deck {
                    content: player2_subdeck.to_vec(),
                },
            });

            turn_winner = subgame.play_game();
        } else {
            turn_winner = if player1_top_card > player2_top_card {
                GameWinner::Player1
            } else {
                GameWinner::Player2
            };
        }

        self.game.player1.content = self.game.player1.content[1..].to_vec();
        self.game.player2.content = self.game.player2.content[1..].to_vec();

        match turn_winner {
            GameWinner::Player1 => {
                self.game.player1.content.push(player1_top_card);
                self.game.player1.content.push(player2_top_card);
            }
            GameWinner::Player2 => {
                self.game.player2.content.push(player2_top_card);
                self.game.player2.content.push(player1_top_card);
            }
        }

        if self.game.player1.content.len() == 0 {
            game_winner = Some(GameWinner::Player2);
        } else if self.game.player2.content.len() == 0 {
            game_winner = Some(GameWinner::Player1);
        }

        game_winner
    }

    fn play_game(&mut self) -> GameWinner {
        let mut res = self.play_round();
        while let None = res {
            res = self.play_round();
        }
        res.unwrap()
    }
}

fn read_file() -> Game {
    let mut file = File::open("./input/input22.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let all_lines: Vec<&str> = contents.lines().collect();

    let p1deck = Deck {
        content: all_lines[1..26]
            .iter()
            .map(|n| n.parse().unwrap())
            .collect(),
    };

    let p2deck = Deck {
        content: all_lines[28..53]
            .iter()
            .map(|n| n.parse().unwrap())
            .collect(),
    };

    Game {
        player1: p1deck,
        player2: p2deck,
    }
}

fn solve_part_1(mut hands: Game) -> usize {
    let mut res = hands.winner();
    while let None = res {
        hands.play_round();
        res = hands.winner();
    }
    hands.winner_score()
}

pub fn part_1() -> usize {
    let hands = read_file();
    solve_part_1(hands)
}

fn solve_part_2(hands: Game) -> usize {
    let mut game = RecursiveGame::new(hands);
    let winner = game.play_game();
    match winner {
        GameWinner::Player1 => game.game.player1.deck_score(),
        GameWinner::Player2 => game.game.player2.deck_score(),
    }
}

pub fn part_2() -> usize {
    let hands = read_file();
    solve_part_2(hands)
}
