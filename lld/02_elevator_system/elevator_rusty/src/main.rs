//! 2. Elevator System
//! State pattern for elevator motion state; Strategy pattern for dispatch (which
//! elevator answers a hall call).
//!
//! solution.py implements State as a trait hierarchy (`ElevatorState` with
//! `IdleState`/`MovingUpState`/`MovingDownState`), where each state's `step`/
//! `handle_request` method takes `&mut Elevator` — the very object holding a reference
//! to the state calling it. That's fine in Python (everything is a reference), but in
//! Rust it's a double-mutable-borrow: you can't hold `&mut self.state` as a trait
//! object *and* pass `&mut self` into one of its methods at the same time. The
//! idiomatic Rust translation of a self-referential State pattern is a closed `enum`
//! plus a `match` — each state's logic still lives in exactly one place (one match arm
//! per method), preserving the property that a new state doesn't require touching
//! existing states' code paths, without fighting the borrow checker to get there.
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Idle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MotionState {
    Idle,
    MovingUp,
    MovingDown,
}

struct Elevator {
    #[allow(dead_code)]
    id: String,
    current_floor: i32,
    destinations: HashSet<i32>,
    state: MotionState,
}

impl Elevator {
    fn new(id: &str) -> Self {
        Elevator {
            id: id.to_string(),
            current_floor: 0,
            destinations: HashSet::new(),
            state: MotionState::Idle,
        }
    }

    fn direction(&self) -> Direction {
        match self.state {
            MotionState::Idle => Direction::Idle,
            MotionState::MovingUp => Direction::Up,
            MotionState::MovingDown => Direction::Down,
        }
    }

    fn request_floor(&mut self, floor: i32) {
        match self.state {
            MotionState::Idle => {
                self.destinations.insert(floor);
                if floor > self.current_floor {
                    self.state = MotionState::MovingUp;
                } else if floor < self.current_floor {
                    self.state = MotionState::MovingDown;
                }
            }
            // LOOK algorithm: pick it up on the way if it's along the current path.
            MotionState::MovingUp | MotionState::MovingDown => {
                self.destinations.insert(floor);
            }
        }
    }

    fn step(&mut self) {
        match self.state {
            MotionState::Idle => {} // nothing to do while idle
            MotionState::MovingUp => {
                self.current_floor += 1;
                self.destinations.remove(&self.current_floor);
                let remaining_above = self.destinations.iter().any(|&f| f >= self.current_floor);
                if !remaining_above {
                    let remaining_below =
                        self.destinations.iter().any(|&f| f < self.current_floor);
                    self.state = if remaining_below {
                        MotionState::MovingDown
                    } else {
                        MotionState::Idle
                    };
                }
            }
            MotionState::MovingDown => {
                self.current_floor -= 1;
                self.destinations.remove(&self.current_floor);
                let remaining_below = self.destinations.iter().any(|&f| f <= self.current_floor);
                if !remaining_below {
                    let remaining_above =
                        self.destinations.iter().any(|&f| f > self.current_floor);
                    self.state = if remaining_above {
                        MotionState::MovingUp
                    } else {
                        MotionState::Idle
                    };
                }
            }
        }
    }
}

trait DispatchStrategy {
    fn select_elevator(&self, elevators: &[Elevator], floor: i32, direction: Direction) -> usize;
}

struct NearestIdleOrSameDirectionStrategy;

impl DispatchStrategy for NearestIdleOrSameDirectionStrategy {
    /// Prefer an idle elevator; else the closest one already moving the right way.
    fn select_elevator(&self, elevators: &[Elevator], floor: i32, direction: Direction) -> usize {
        let idle: Vec<usize> = elevators
            .iter()
            .enumerate()
            .filter(|(_, e)| e.direction() == Direction::Idle)
            .map(|(i, _)| i)
            .collect();
        if !idle.is_empty() {
            return *idle
                .iter()
                .min_by_key(|&&i| (elevators[i].current_floor - floor).abs())
                .unwrap();
        }

        let same_direction: Vec<usize> = elevators
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                e.direction() == direction
                    && ((direction == Direction::Up && e.current_floor <= floor)
                        || (direction == Direction::Down && e.current_floor >= floor))
            })
            .map(|(i, _)| i)
            .collect();
        if !same_direction.is_empty() {
            return *same_direction
                .iter()
                .min_by_key(|&&i| (elevators[i].current_floor - floor).abs())
                .unwrap();
        }

        (0..elevators.len())
            .min_by_key(|&i| (elevators[i].current_floor - floor).abs())
            .unwrap()
    }
}

struct ElevatorSystem {
    elevators: Vec<Elevator>,
    strategy: Box<dyn DispatchStrategy>,
}

impl ElevatorSystem {
    fn new(num_elevators: usize) -> Self {
        let elevators = (0..num_elevators)
            .map(|i| Elevator::new(&format!("E{i}")))
            .collect();
        ElevatorSystem {
            elevators,
            strategy: Box::new(NearestIdleOrSameDirectionStrategy),
        }
    }

    fn request_pickup(&mut self, floor: i32, direction: Direction) -> usize {
        let idx = self.strategy.select_elevator(&self.elevators, floor, direction);
        self.elevators[idx].request_floor(floor);
        idx
    }

    fn step_all(&mut self) {
        for e in self.elevators.iter_mut() {
            e.step();
        }
    }
}

fn main() {
    let mut system = ElevatorSystem::new(2);
    let chosen = system.request_pickup(2, Direction::Up);
    println!("elevator {chosen} dispatched");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_python_reference_behavior() {
        let mut system = ElevatorSystem::new(2);
        system.elevators[1].current_floor = 5;

        let chosen = system.request_pickup(2, Direction::Up);
        assert_eq!(chosen, 0); // e0 starts at floor 0, closer to floor 2

        system.elevators[0].request_floor(6);
        assert_eq!(system.elevators[0].direction(), Direction::Up);
        for _ in 0..6 {
            system.step_all();
        }
        assert_eq!(system.elevators[0].current_floor, 6);
        assert_eq!(system.elevators[0].direction(), Direction::Idle);

        // Dispatch prefers an idle car over one moving away from the request.
        system.elevators[1].state = MotionState::MovingDown;
        system.elevators[1].destinations = HashSet::from([2]);
        let chosen2 = system.request_pickup(4, Direction::Up);
        assert_eq!(chosen2, 0); // e0 is idle again; e1 is moving down, wrong direction
    }
}
