use rand::prelude::*;

pub struct Oracle {
    chain: Vec<Box<Block>>,
}

trait Emittable {
    fn emit(&self) -> Option<Vec<char>>;
}

enum Token {
    SingleChar(char),
    Range { from: char, to: char },
}

struct Block {
    kind: BlockKind,
}

impl Block {
    pub fn new(kind: BlockKind) -> Self {
        Self { kind }
    }
}

enum BlockKind {
    SingleCharBlock(char),
    RangeBlock { from: char, to: char },
}

impl Emittable for Block {
    fn emit(&self) -> Option<Vec<char>> {
        let mut rng = rand::rng();
        match self.kind {
            BlockKind::SingleCharBlock(c) => Some(vec![c]),
            BlockKind::RangeBlock { from, to } => Some(vec![rng.random_range(from..to)]),
        }
    }
}

impl Oracle {
    pub fn new(tokens: &Vec<Token>) -> Self {
        Self {
            chain: tokens_2_blocks(tokens),
        }
    }

    pub fn exec(&self) -> String {
        self.chain
            .iter()
            .map(|ch| match ch.emit() {
                Some(v) => v.iter().map(|el| *el).collect::<String>(),
                _ => todo!("not implemented"),
            })
            .collect()
    }
}

fn tokens_2_blocks(tokens: &Vec<Token>) -> Vec<Box<Block>> {
    tokens
        .iter()
        .map(|token| match *token {
            Token::SingleChar(c) => Box::new(Block::new(BlockKind::SingleCharBlock(c))),
            Token::Range { from, to } => Box::new(Block::new(BlockKind::RangeBlock { from, to })),
        })
        .collect()
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let oracle = Oracle::new(&vec![
            Token::SingleChar('a'),
            Token::SingleChar('b'),
            Token::Range { from: 'a', to: 'z' },
        ]);

        println!("ORACLE SAID: {}", oracle.exec());
    }
}
