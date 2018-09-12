use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Rand {
    seed: usize,
    digit: u8,
}

impl Rand {
    pub fn default() -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .ok().unwrap().as_secs().to_le_bytes();

        Rand { seed: now[0] as usize, digit: 1 }
    }

    pub fn new(digit: u8) -> usize {
        let rand = Rand {
            seed: Rand::default().seed,
            digit: digit,
        };
        
        rand.next().patch()
    }

    fn expt(mut raw: usize, target: usize) -> usize {
        let _raw = raw;
        for _ in 0..target {
            raw *= _raw
        }
        raw
    }    
    
    fn next(self) -> Self {
        let mut seed = (self.seed^2) % 10;
        if seed == 0 { seed += 1 }
        Rand { seed: seed , digit: self.digit }
    }

    fn patch(self) -> usize {
        let len = self.digit as usize;
        let mut flower = self.next();
        let mut seed = flower.seed;
        
        for i in 1..len {
            flower = flower.next();            
            seed += flower.seed * Rand::expt(10, i - 1);
        }
        
        seed
    }
}


