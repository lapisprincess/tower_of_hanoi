/// A Tower, meant to hold "rings" represented as u32 values.
#[derive(PartialEq)]
pub struct Tower {
    rings: Vec<u32>,
    height: usize,
}

impl Tower {
    pub fn new(height: usize) -> Tower {
        Tower {
            rings: Vec::new(),
            height
        }
    }

    /// Add a ring to the tower, following TOH rules
    pub fn add(&mut self, ring: u32) -> bool {
        let next: Option<&u32> = self.rings.last();
        if next == None || next.unwrap() > &ring {
            self.rings.push(ring);
            return true;
        }
        false
    }

    /// Pull topmost ring from tower.
    pub fn pop(&mut self) -> Option<u32> { self.rings.pop() }

    /// Convert tower to a list of strings, each string
    /// representing a level of the tower when printed.
    pub fn to_string(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for i in 0..self.height {
            let mut line: String = String::new();
            let level: Option<&u32> = self.rings.get(i);
            let mut ring_size: u32 = 0;
            if level != None { ring_size = *level.unwrap(); }
            let num_spaces: u32 = self.largest_ring() - ring_size;
            for _ in 0..num_spaces { line += " "; }
            for _ in 0..ring_size { line += "("; }
            line += "|";
            for _ in 0..ring_size { line += ")"; }
            for _ in 0..num_spaces { line += " "; }
            out.push(line);
        }
        out.reverse();
        out
    }

    /// Get size of the bottom-most ring, which should be the largest
    pub fn largest_ring(&self) -> u32 { 
        if self.rings.len() == 0 { return 0; }
        *self.rings.get(0).unwrap()
    }

}