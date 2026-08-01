//! 1. Parking Lot System
//! Strategy pattern for spot assignment; a Vehicle trait hierarchy sized by SpotType.
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SpotType {
    Motorcycle,
    Compact,
    Large,
}

trait Vehicle {
    fn license_plate(&self) -> &str;
    fn spot_type(&self) -> SpotType;
}

struct Motorcycle {
    license_plate: String,
}
struct Car {
    license_plate: String,
}
#[allow(dead_code)] // part of the Vehicle hierarchy; not exercised by the reference test
struct Bus {
    license_plate: String,
}

impl Vehicle for Motorcycle {
    fn license_plate(&self) -> &str {
        &self.license_plate
    }
    fn spot_type(&self) -> SpotType {
        SpotType::Motorcycle
    }
}
impl Vehicle for Car {
    fn license_plate(&self) -> &str {
        &self.license_plate
    }
    fn spot_type(&self) -> SpotType {
        SpotType::Compact
    }
}
impl Vehicle for Bus {
    fn license_plate(&self) -> &str {
        &self.license_plate
    }
    fn spot_type(&self) -> SpotType {
        SpotType::Large
    }
}

struct ParkingSpot {
    spot_id: String,
    spot_type: SpotType,
    vehicle: Option<Box<dyn Vehicle>>,
}

impl ParkingSpot {
    fn new(spot_id: &str, spot_type: SpotType) -> Self {
        ParkingSpot {
            spot_id: spot_id.to_string(),
            spot_type,
            vehicle: None,
        }
    }

    fn is_free(&self) -> bool {
        self.vehicle.is_none()
    }

    fn can_fit(&self, vehicle: &dyn Vehicle) -> bool {
        // Ord is derived in declaration order (Motorcycle < Compact < Large), so a
        // motorcycle fits a compact/large spot but not vice versa, mirroring the
        // explicit `order.index(...)` comparison in solution.py.
        self.spot_type >= vehicle.spot_type()
    }

    fn park(&mut self, vehicle: Box<dyn Vehicle>) {
        self.vehicle = Some(vehicle);
    }

    fn vacate(&mut self) {
        self.vehicle = None;
    }
}

trait SpotAssignmentStrategy {
    /// Open/Closed: swap assignment algorithms without touching ParkingLot.
    fn find_spot(&self, spots: &[ParkingSpot], vehicle: &dyn Vehicle) -> Option<usize>;
}

struct NearestFitStrategy;

impl SpotAssignmentStrategy for NearestFitStrategy {
    /// Smallest spot that still fits the vehicle, first free one found.
    fn find_spot(&self, spots: &[ParkingSpot], vehicle: &dyn Vehicle) -> Option<usize> {
        let mut candidates: Vec<usize> = spots
            .iter()
            .enumerate()
            .filter(|(_, s)| s.is_free() && s.can_fit(vehicle))
            .map(|(i, _)| i)
            .collect();
        candidates.sort_by_key(|&i| spots[i].spot_type);
        candidates.first().copied()
    }
}

struct Ticket {
    #[allow(dead_code)]
    ticket_id: String,
    #[allow(dead_code)]
    license_plate: String,
    spot_id: String,
    entry_time: SystemTime,
    exit_time: Option<SystemTime>,
}

trait PaymentProcessor {
    fn charge(&self, amount: f64) -> bool;
}

struct CreditCardPayment;

impl PaymentProcessor for CreditCardPayment {
    fn charge(&self, amount: f64) -> bool {
        amount >= 0.0 // stand-in for a real payment gateway call
    }
}

const HOURLY_RATE: f64 = 2.0;

/// Coordinator: owns spots, tickets, and the assignment strategy.
///
/// Rust's borrow checker enforces exclusive (`&mut self`) access to `ParkingLot` for
/// every mutating method, which is what solution.py's `threading.Lock` guards
/// explicitly — there's no separate lock object here because the type system already
/// forbids two callers from mutating the same `ParkingLot` concurrently without one.
/// A real multi-threaded deployment would still need `Arc<Mutex<ParkingLot>>` (or a
/// DB-level compare-and-swap across processes) to serialize access across threads;
/// the single-process race this replicates is exactly the one solution.py's lock guards.
struct ParkingLot {
    spots: Vec<ParkingSpot>,
    strategy: Box<dyn SpotAssignmentStrategy>,
    active_tickets: HashMap<String, Ticket>,
    payment: Box<dyn PaymentProcessor>,
    next_ticket_id: u64,
}

impl ParkingLot {
    fn new(spots: Vec<ParkingSpot>, strategy: Box<dyn SpotAssignmentStrategy>) -> Self {
        ParkingLot {
            spots,
            strategy,
            active_tickets: HashMap::new(),
            payment: Box::new(CreditCardPayment),
            next_ticket_id: 1,
        }
    }

    fn park_vehicle(&mut self, vehicle: Box<dyn Vehicle>) -> Option<String> {
        let idx = self.strategy.find_spot(&self.spots, vehicle.as_ref())?;
        let ticket_id = format!("T{}", self.next_ticket_id);
        self.next_ticket_id += 1;
        let ticket = Ticket {
            ticket_id: ticket_id.clone(),
            license_plate: vehicle.license_plate().to_string(),
            spot_id: self.spots[idx].spot_id.clone(),
            entry_time: SystemTime::now(),
            exit_time: None,
        };
        self.spots[idx].park(vehicle);
        self.active_tickets.insert(ticket_id.clone(), ticket);
        Some(ticket_id)
    }

    fn exit_vehicle(&mut self, ticket_id: &str) -> Result<f64, String> {
        let mut ticket = self
            .active_tickets
            .remove(ticket_id)
            .ok_or_else(|| "no such ticket".to_string())?;
        let exit_time = SystemTime::now();
        ticket.exit_time = Some(exit_time);

        let spot = self
            .spots
            .iter_mut()
            .find(|s| s.spot_id == ticket.spot_id)
            .expect("ticket referenced a spot that must exist");

        let elapsed = exit_time
            .duration_since(ticket.entry_time)
            .unwrap_or(Duration::ZERO);
        let hours = (elapsed.as_secs() / 3600 + 1).max(1);
        let amount = hours as f64 * HOURLY_RATE;

        if !self.payment.charge(amount) {
            return Err("payment failed".to_string());
        }
        spot.vacate();
        Ok(amount)
    }

    fn available_count(&self, spot_type: SpotType) -> usize {
        self.spots
            .iter()
            .filter(|s| s.is_free() && s.spot_type == spot_type)
            .count()
    }
}

fn main() {
    let spots = vec![
        ParkingSpot::new("M1", SpotType::Motorcycle),
        ParkingSpot::new("C1", SpotType::Compact),
        ParkingSpot::new("C2", SpotType::Compact),
        ParkingSpot::new("L1", SpotType::Large),
    ];
    let mut lot = ParkingLot::new(spots, Box::new(NearestFitStrategy));
    let ticket_id = lot
        .park_vehicle(Box::new(Car {
            license_plate: "ABC-123".to_string(),
        }))
        .expect("a compact spot is free");
    println!("parked, ticket = {ticket_id}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_python_reference_behavior() {
        let spots = vec![
            ParkingSpot::new("M1", SpotType::Motorcycle),
            ParkingSpot::new("C1", SpotType::Compact),
            ParkingSpot::new("C2", SpotType::Compact),
            ParkingSpot::new("L1", SpotType::Large),
        ];
        let mut lot = ParkingLot::new(spots, Box::new(NearestFitStrategy));

        let ticket_id = lot
            .park_vehicle(Box::new(Car {
                license_plate: "ABC-123".to_string(),
            }))
            .expect("a compact spot is free");
        assert_eq!(lot.available_count(SpotType::Compact), 1);

        let bike_ticket_id = lot
            .park_vehicle(Box::new(Motorcycle {
                license_plate: "MOTO-1".to_string(),
            }))
            .expect("the motorcycle spot is free");
        assert_eq!(
            lot.active_tickets.get(&bike_ticket_id).unwrap().spot_id,
            "M1"
        );

        let amount = lot.exit_vehicle(&ticket_id).expect("payment succeeds");
        assert!(amount >= HOURLY_RATE);
        assert_eq!(lot.available_count(SpotType::Compact), 2);

        // Lot full of compacts, but a motorcycle can still overflow into one.
        let mut small_lot = ParkingLot::new(
            vec![ParkingSpot::new("C1", SpotType::Compact)],
            Box::new(NearestFitStrategy),
        );
        small_lot
            .park_vehicle(Box::new(Car {
                license_plate: "ZZZ-999".to_string(),
            }))
            .expect("first car takes the only compact spot");
        assert!(small_lot
            .park_vehicle(Box::new(Car {
                license_plate: "YYY-888".to_string(),
            }))
            .is_none());
    }
}
