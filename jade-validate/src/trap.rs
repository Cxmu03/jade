use ringbuffer::{ConstGenericRingBuffer, RingBuffer};

pub struct TrapDetector {
    pc_buffer: ConstGenericRingBuffer<u16, 7>,
}

impl TrapDetector {
    pub fn new() -> TrapDetector {
        TrapDetector {
            pc_buffer: ConstGenericRingBuffer::new(),
        }
    }

    pub fn add_cycle(&mut self, pc: u16) {
        self.pc_buffer.push(pc);
    }

    pub fn is_trap(&self) -> bool {
        self.pc_buffer.len() == 7
            && self.pc_buffer[6] == self.pc_buffer[0]
            && self.pc_buffer[6] == self.pc_buffer[3]
    }
}

#[cfg(test)]
mod tests {
    use super::TrapDetector;

    #[test]
    fn trap_occurs() {
        let mut detector = TrapDetector::new();
        let cycles = [10, 15, 20, 10, 15, 20, 10];

        for cycle in cycles {
            detector.add_cycle(cycle);
        }

        assert_eq!(detector.is_trap(), true);
    }

    #[test]
    fn trap_does_not_occur() {
        let mut detector = TrapDetector::new();
        let cycles = [10, 15, 20, 10, 15, 20, 0];

        for cycle in cycles {
            detector.add_cycle(cycle);
        }

        assert_eq!(detector.is_trap(), false);
    }

    #[test]
    fn not_enough_values() {
        let mut detector = TrapDetector::new();
        let cycles = [10, 15, 20, 10, 15, 20];

        for cycle in cycles {
            detector.add_cycle(cycle);
        }

        assert_eq!(detector.is_trap(), false);
    }
}
