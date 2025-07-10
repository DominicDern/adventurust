use crate::Character;
use priority_queue::PriorityQueue;

#[derive(Debug)]
pub struct InitiativeQueue<'a> {
    queue: PriorityQueue<(&'a Character, usize), u16>,
    position: usize,
    current_id: usize,
}

impl<'a> InitiativeQueue<'a> {
    /// Creates a inititive queue with pre rolled inititive numbers
    pub fn new_pre_rolled(actors: Vec<(&'a Character, u16)>) -> Self {
        let mut queue: PriorityQueue<&Character, u16> = PriorityQueue::new();
        let mut index = 0;
        for roll in actors {
            queue.push(roll.0, roll.1);
            index += 1;
        }
        let current_id = queue.len();
        Self {
            queue,
            position: 0,
            current_id,
        }
    }
}

impl<'a> InitiativeQueue<'a> {
    // TODO add rolling inititive
    pub fn get_queue(&self) -> Option<Vec<((&'a Character, usize), u16)>> {
        if self.queue.is_empty() {
            None
        } else {
            let mut queue = Vec::new();
            for (character, initiative) in self.queue.clone().into_sorted_iter() {
                queue.push((character, initiative));
            }
            queue.rotate_left(self.position);
            Some(queue)
        }
    }

    pub fn add(&mut self, actor: &'a Character, initiative: u16) {
        todo!()
    }

    pub fn next_turn(&mut self) {
        // TODO add status effect turn addition
        if self.position == self.queue.len() - 1 {
            self.position = 0;
        } else {
            self.position += 1;
        }
    }
}
