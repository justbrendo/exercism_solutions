#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct Frame {
    rolls: Vec<u16>,
    over: bool,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            rolls: Vec::new(),
            over: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.over {
            return Err(Error::GameComplete);
        }
        
        self.rolls.push(pins);

        if self.rolls.len() == 1 && pins == 10 {
            self.over = true;
        } else if self.rolls.len() == 2 {
            self.over = true;
        }

        Ok(())
    }

    pub fn is_over(&self) -> bool {
        self.over
    }
}

pub struct BowlingGame {
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: vec![Frame::new()],
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.frames.len() == 10 && self.frames.last().unwrap().is_over() {
            return Err(Error::GameComplete);
        }
        
        let frame = self.frames.last_mut().unwrap();
        
        frame.roll(pins)?;
        
        if frame.is_over() && self.frames.len() < 10 {
            self.frames.push(Frame::new());
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames.len() != 10 || !self.frames.last().unwrap().is_over() {
            return None;
        }
        
        let mut score = 0;
        for frame in &self.frames {
            score += frame.rolls.iter().sum::<u16>();
        }
        
        Some(score)
    }
}