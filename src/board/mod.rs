pub mod tower;
use crate::board::tower::Tower;

/**
 * A board of towers, with functionality to play 
 * "Tower of Hanoi" on it.
 */
pub struct Board {
    towers: Vec<Tower>,
    height: usize
}

impl Board {

    /// Constructor, which creates a board with a series of towers of
    /// specified length. The first tower is populated with a specified
    /// number of rings.
    pub fn new(num_rings: usize, num_towers: usize) -> Board {
        let mut towers = Vec::new();
        let mut t = Tower::new(num_rings as usize);
        for i in (1..(num_rings+1)).rev() { t.add(i as u32); println!("{}", i);}
        towers.push(t);
        for _ in 1..num_towers {
            let t: Tower = Tower::new(num_rings as usize);
            towers.push(t);
        }
        Board { 
            towers, 
            height: num_rings 
        }
    }

    /// Move ring from one tower to another given specified indices,
    pub fn move_rings(&mut self, from: usize, to: usize) -> bool {
        if from == to { return false; }
        let from_tower = self.towers.get_mut(from);
        if from_tower == None { return false; }
        let ring1: Option<u32> = from_tower.unwrap().pop();
        if ring1 == None { return false; }
        let result = self.add_to_tower(ring1.unwrap(), to);
        if result == false {
            self.add_to_tower(ring1.unwrap(), from);
            return false;
        } result
    }

    /// Prints the full board, towers, rings and all, to the console.
    pub fn print(&self) {
        for i in 0..self.height {
            for tower in &self.towers {
                let diff: u32 = self.largest_ring() - tower.largest_ring();
                let full_string = tower.to_string();
                let line: Option<&String> = full_string.get(i);
                if line == None { continue; }
                else { 
                    for _ in 0..diff { print!(" "); }
                    print!("{}", line.unwrap()); 
                    for _ in 0..diff { print!(" "); }
                }
            }
            println!();
        }
    }

    /// Scans the board to see if it's in a victory state.
    pub fn check_victory(&self) -> bool {
        let mut condition = true;
        for (index, tower) in self.towers.iter().enumerate() {
            if index == self.towers.len() - 1 { break; }
            if tower.largest_ring() != 0 { condition = false; }
        } condition
    }

    /// Quick and easy private tower-adder.
    fn add_to_tower(&mut self, ring: u32, index: usize) -> bool {
        let tower: Option<&mut Tower> = self.towers.get_mut(index);
        if tower == None { return false; }
        tower.unwrap().add(ring)
    }

    /// Get the largest ring of all the towers.
    fn largest_ring(&self) -> u32 {
        let mut max: u32 = 0;
        for tower in &self.towers {
            let new_ring: u32 = tower.largest_ring();
            if tower.largest_ring() > max { max = new_ring; }
        }
        max
    }

}
