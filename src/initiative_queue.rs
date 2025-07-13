use crate::character::ID;
use std::collections::VecDeque;
// use VecDeque;

#[derive(Debug, Clone)]
pub struct InitiativeQueue {
    queue: VecDeque<(ID, u16)>,
    position: usize,
}

impl InitiativeQueue {
    /// Creates a inititive queue with pre rolled inititive numbers
    pub fn new_pre_rolled(mut actors: Vec<(ID, u16)>) -> Option<Self> {
        let mut queue = VecDeque::new();

        if actors.is_empty() {
            None
        } else if actors.len() == 1 {
            queue.push_front(actors.pop().unwrap());
            let queue = InitiativeQueue { queue, position: 0 };
            Some(queue)
        } else {
            queue.push_front(actors.pop().unwrap());
            let mut queue = InitiativeQueue { queue, position: 0 };
            for (id, initiative) in actors {
                queue.add(id, initiative);
            }
            Some(queue)
        }
    }
}

impl InitiativeQueue {
    // TODO add rolling inititive
    pub fn get_queue(&self) -> Option<VecDeque<(ID, u16)>> {
        if self.queue.is_empty() {
            None
        } else {
            let mut queue = self.queue.clone();
            queue.rotate_left(self.position);
            Some(queue)
        }
    }

    /// Adds an ID to the initiative. If the queue already contains an ID of equal initiative
    /// the new initiative is added immediately after.
    pub fn add(&mut self, id: ID, initiative: u16) {
        let mut index = 0;
        for (_, initiative_check) in self.queue.clone() {
            if initiative == initiative_check {
                self.queue.insert(index + 1, (id, initiative));
                break;
            } else if initiative > initiative_check {
                self.queue.insert(index, (id.clone(), initiative));
                break;
            } else if initiative < initiative_check && index == self.queue.len() - 1 {
                self.queue.push_back((id.clone(), initiative));
            } else {
                index += 1;
            }
        }
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
