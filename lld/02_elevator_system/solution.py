"""2. Elevator System
State pattern for elevator motion state; Strategy pattern for dispatch (which elevator
answers a hall call).
"""
from __future__ import annotations

from abc import ABC, abstractmethod
from enum import Enum, auto
from typing import Optional


class Direction(Enum):
    UP = auto()
    DOWN = auto()
    IDLE = auto()


class ElevatorState(ABC):
    """Delegate target for Elevator; each state knows how to handle a request and a step."""

    @abstractmethod
    def handle_request(self, elevator: "Elevator", floor: int) -> None:
        ...

    @abstractmethod
    def step(self, elevator: "Elevator") -> None:
        ...


class IdleState(ElevatorState):
    def handle_request(self, elevator: "Elevator", floor: int) -> None:
        elevator.destinations.add(floor)
        if floor > elevator.current_floor:
            elevator.set_state(MovingUpState())
        elif floor < elevator.current_floor:
            elevator.set_state(MovingDownState())

    def step(self, elevator: "Elevator") -> None:
        pass  # nothing to do while idle


class MovingUpState(ElevatorState):
    def handle_request(self, elevator: "Elevator", floor: int) -> None:
        elevator.destinations.add(floor)  # LOOK: pick it up on the way if along the path

    def step(self, elevator: "Elevator") -> None:
        elevator.current_floor += 1
        if elevator.current_floor in elevator.destinations:
            elevator.destinations.remove(elevator.current_floor)
        remaining_above = [f for f in elevator.destinations if f >= elevator.current_floor]
        if not remaining_above:
            remaining_below = [f for f in elevator.destinations if f < elevator.current_floor]
            elevator.set_state(MovingDownState() if remaining_below else IdleState())


class MovingDownState(ElevatorState):
    def handle_request(self, elevator: "Elevator", floor: int) -> None:
        elevator.destinations.add(floor)

    def step(self, elevator: "Elevator") -> None:
        elevator.current_floor -= 1
        if elevator.current_floor in elevator.destinations:
            elevator.destinations.remove(elevator.current_floor)
        remaining_below = [f for f in elevator.destinations if f <= elevator.current_floor]
        if not remaining_below:
            remaining_above = [f for f in elevator.destinations if f > elevator.current_floor]
            elevator.set_state(MovingUpState() if remaining_above else IdleState())


class Elevator:
    def __init__(self, elevator_id: str, current_floor: int = 0):
        self.elevator_id = elevator_id
        self.current_floor = current_floor
        self.destinations: set[int] = set()
        self._state: ElevatorState = IdleState()

    def set_state(self, state: ElevatorState) -> None:
        self._state = state

    @property
    def direction(self) -> Direction:
        if isinstance(self._state, MovingUpState):
            return Direction.UP
        if isinstance(self._state, MovingDownState):
            return Direction.DOWN
        return Direction.IDLE

    def request_floor(self, floor: int) -> None:
        self._state.handle_request(self, floor)

    def step(self) -> None:
        self._state.step(self)


class DispatchStrategy(ABC):
    @abstractmethod
    def select_elevator(self, elevators: list[Elevator], floor: int, direction: Direction) -> Elevator:
        ...


class NearestIdleOrSameDirectionStrategy(DispatchStrategy):
    """Prefer an idle elevator; else the closest one already moving the right way."""

    def select_elevator(self, elevators: list[Elevator], floor: int, direction: Direction) -> Elevator:
        idle = [e for e in elevators if e.direction == Direction.IDLE]
        if idle:
            return min(idle, key=lambda e: abs(e.current_floor - floor))

        same_direction = [
            e for e in elevators
            if e.direction == direction
            and (
                (direction == Direction.UP and e.current_floor <= floor)
                or (direction == Direction.DOWN and e.current_floor >= floor)
            )
        ]
        if same_direction:
            return min(same_direction, key=lambda e: abs(e.current_floor - floor))

        return min(elevators, key=lambda e: abs(e.current_floor - floor))


class ElevatorSystem:
    def __init__(self, num_elevators: int, strategy: Optional[DispatchStrategy] = None):
        self.elevators = [Elevator(f"E{i}") for i in range(num_elevators)]
        self.strategy = strategy or NearestIdleOrSameDirectionStrategy()

    def request_pickup(self, floor: int, direction: Direction) -> Elevator:
        chosen = self.strategy.select_elevator(self.elevators, floor, direction)
        chosen.request_floor(floor)
        return chosen

    def step_all(self) -> None:
        for e in self.elevators:
            e.step()


if __name__ == "__main__":
    system = ElevatorSystem(num_elevators=2)

    e0 = system.elevators[0]
    e1 = system.elevators[1]
    e1.current_floor = 5

    chosen = system.request_pickup(floor=2, direction=Direction.UP)
    assert chosen is e0  # e0 starts at floor 0, closer to floor 2

    e0.request_floor(6)
    assert e0.direction == Direction.UP
    for _ in range(6):
        system.step_all()
    assert e0.current_floor == 6
    assert e0.direction == Direction.IDLE

    # Dispatch prefers an idle car over one moving away from the request.
    e1.set_state(MovingDownState())
    e1.destinations = {2}
    chosen2 = system.request_pickup(floor=4, direction=Direction.UP)
    assert chosen2 is e0  # e0 is idle again; e1 is moving down, wrong direction

    print("All tests passed.")
