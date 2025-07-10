use crate::Character;
use priority_queue::PriorityQueue;

#[derive(Debug)]
pub struct InitiativeQueue<'a> {
    queue: PriorityQueue<&'a Character, u16>,
    position: u16,
}

impl<'a> InitiativeQueue<'a> {
    /// Creates a inititive queue with pre rolled inititive numbers
    pub fn new_pre_rolled(actors: Vec<(&'a Character, u16)>) -> Self {
        let mut queue: PriorityQueue<&Character, u16> = PriorityQueue::new();
        for roll in actors {
            queue.push(roll.0, roll.1);
        }
        Self { queue, position: 0 }
    }
}

impl<'a> InitiativeQueue<'a> {
    // TODO add rolling inititive

    pub fn get_queue(&self) -> Self {
        todo!()
    }

    pub fn add(&mut self, actor: &'a Character, initiative: u16) {
        self.queue.push(actor, initiative);
    }

    pub fn next_turn(&mut self) {
        // TODO add status effect turn addition
        if self.position == self.queue.len() as u16 - 1 {
            self.position = 0;
        } else {
            self.position += 1;
        }
    }
}
