"""1. Parking Lot System
Strategy pattern for spot assignment; a Vehicle hierarchy sized by SpotType.
"""
from __future__ import annotations

import threading
from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum, auto
from typing import Optional


class SpotType(Enum):
    MOTORCYCLE = auto()
    COMPACT = auto()
    LARGE = auto()


class Vehicle(ABC):
    def __init__(self, license_plate: str):
        self.license_plate = license_plate

    @property
    @abstractmethod
    def spot_type(self) -> SpotType:
        ...


class Motorcycle(Vehicle):
    spot_type = SpotType.MOTORCYCLE


class Car(Vehicle):
    spot_type = SpotType.COMPACT


class Bus(Vehicle):
    spot_type = SpotType.LARGE


class ParkingSpot:
    def __init__(self, spot_id: str, spot_type: SpotType):
        self.spot_id = spot_id
        self.spot_type = spot_type
        self.vehicle: Optional[Vehicle] = None

    @property
    def is_free(self) -> bool:
        return self.vehicle is None

    def can_fit(self, vehicle: Vehicle) -> bool:
        # A vehicle can use its own spot size or any larger one, mirroring how a
        # motorcycle can park in a compact/large spot but not vice versa.
        order = [SpotType.MOTORCYCLE, SpotType.COMPACT, SpotType.LARGE]
        return order.index(self.spot_type) >= order.index(vehicle.spot_type)

    def park(self, vehicle: Vehicle) -> None:
        self.vehicle = vehicle

    def vacate(self) -> None:
        self.vehicle = None


class SpotAssignmentStrategy(ABC):
    """Open/Closed: swap assignment algorithms without touching ParkingLot."""

    @abstractmethod
    def find_spot(self, spots: list[ParkingSpot], vehicle: Vehicle) -> Optional[ParkingSpot]:
        ...


class NearestFitStrategy(SpotAssignmentStrategy):
    """Smallest spot that still fits the vehicle, first free one found."""

    def find_spot(self, spots: list[ParkingSpot], vehicle: Vehicle) -> Optional[ParkingSpot]:
        order = [SpotType.MOTORCYCLE, SpotType.COMPACT, SpotType.LARGE]
        candidates = [s for s in spots if s.is_free and s.can_fit(vehicle)]
        candidates.sort(key=lambda s: order.index(s.spot_type))
        return candidates[0] if candidates else None


@dataclass
class Ticket:
    ticket_id: str
    license_plate: str
    spot_id: str
    entry_time: datetime = field(default_factory=datetime.now)
    exit_time: Optional[datetime] = None


class PaymentProcessor(ABC):
    @abstractmethod
    def charge(self, amount: float) -> bool:
        ...


class CreditCardPayment(PaymentProcessor):
    def charge(self, amount: float) -> bool:
        return amount >= 0  # stand-in for a real payment gateway call


class ParkingLot:
    """Singleton coordinator: owns spots, tickets, and the assignment strategy."""

    HOURLY_RATE = 2.0

    def __init__(self, spots: list[ParkingSpot], strategy: SpotAssignmentStrategy):
        self._spots = spots
        self._strategy = strategy
        self._active_tickets: dict[str, Ticket] = {}
        self._payment = CreditCardPayment()
        self._lock = threading.Lock()
        self._next_ticket_id = 1

    def park_vehicle(self, vehicle: Vehicle) -> Optional[Ticket]:
        with self._lock:  # guards the read-then-write race on spot assignment
            spot = self._strategy.find_spot(self._spots, vehicle)
            if spot is None:
                return None
            spot.park(vehicle)
            ticket = Ticket(
                ticket_id=f"T{self._next_ticket_id}",
                license_plate=vehicle.license_plate,
                spot_id=spot.spot_id,
            )
            self._next_ticket_id += 1
            self._active_tickets[ticket.ticket_id] = ticket
            return ticket

    def exit_vehicle(self, ticket_id: str) -> float:
        with self._lock:
            ticket = self._active_tickets.pop(ticket_id)
            ticket.exit_time = datetime.now()
            spot = next(s for s in self._spots if s.spot_id == ticket.spot_id)
            hours = max(1, (ticket.exit_time - ticket.entry_time).seconds // 3600 + 1)
            amount = hours * self.HOURLY_RATE
            if not self._payment.charge(amount):
                raise RuntimeError("payment failed")
            spot.vacate()
            return amount

    def available_count(self, spot_type: SpotType) -> int:
        return sum(1 for s in self._spots if s.is_free and s.spot_type == spot_type)


if __name__ == "__main__":
    spots = [
        ParkingSpot("M1", SpotType.MOTORCYCLE),
        ParkingSpot("C1", SpotType.COMPACT),
        ParkingSpot("C2", SpotType.COMPACT),
        ParkingSpot("L1", SpotType.LARGE),
    ]
    lot = ParkingLot(spots, NearestFitStrategy())

    car = Car("ABC-123")
    ticket = lot.park_vehicle(car)
    assert ticket is not None
    assert lot.available_count(SpotType.COMPACT) == 1

    bike = Motorcycle("MOTO-1")
    bike_ticket = lot.park_vehicle(bike)
    assert bike_ticket is not None
    assert bike_ticket.spot_id == "M1"

    amount = lot.exit_vehicle(ticket.ticket_id)
    assert amount >= lot.HOURLY_RATE
    assert lot.available_count(SpotType.COMPACT) == 2

    # Lot full of compacts, but a motorcycle can still overflow into one.
    small_lot = ParkingLot([ParkingSpot("C1", SpotType.COMPACT)], NearestFitStrategy())
    small_lot.park_vehicle(Car("ZZZ-999"))
    assert small_lot.park_vehicle(Car("YYY-888")) is None
    print("All tests passed.")
