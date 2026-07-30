"""3. Vending Machine
Textbook State pattern: VendingMachine (context) delegates to its current State.
"""
from __future__ import annotations

from abc import ABC, abstractmethod
from dataclasses import dataclass


@dataclass
class Product:
    code: str
    name: str
    price: int  # cents
    quantity: int


class Inventory:
    def __init__(self, products: dict[str, Product]):
        self._products = products

    def get(self, code: str) -> Product | None:
        return self._products.get(code)

    def decrement(self, code: str) -> None:
        self._products[code].quantity -= 1


class VendingMachineState(ABC):
    @abstractmethod
    def select_product(self, machine: "VendingMachine", code: str) -> None:
        ...

    @abstractmethod
    def insert_coin(self, machine: "VendingMachine", cents: int) -> None:
        ...

    @abstractmethod
    def dispense(self, machine: "VendingMachine") -> str:
        ...

    @abstractmethod
    def cancel(self, machine: "VendingMachine") -> None:
        ...


class IdleState(VendingMachineState):
    def select_product(self, machine: "VendingMachine", code: str) -> None:
        product = machine.inventory.get(code)
        if product is None:
            raise ValueError(f"no such product: {code}")
        if product.quantity <= 0:
            machine.set_state(SoldOutState())
            return
        machine.selected_code = code
        machine.set_state(HasSelectionState())

    def insert_coin(self, machine: "VendingMachine", cents: int) -> None:
        raise RuntimeError("select a product before inserting coins")

    def dispense(self, machine: "VendingMachine") -> str:
        raise RuntimeError("no product selected")

    def cancel(self, machine: "VendingMachine") -> None:
        pass  # nothing to cancel


class HasSelectionState(VendingMachineState):
    def select_product(self, machine: "VendingMachine", code: str) -> None:
        machine.selected_code = code  # allow changing the mind before paying

    def insert_coin(self, machine: "VendingMachine", cents: int) -> None:
        machine.balance += cents
        product = machine.inventory.get(machine.selected_code)
        assert product is not None
        if machine.balance >= product.price:
            machine.set_state(HasEnoughMoneyState())

    def dispense(self, machine: "VendingMachine") -> str:
        raise RuntimeError("insert more coins first")

    def cancel(self, machine: "VendingMachine") -> None:
        machine.refund()
        machine.set_state(IdleState())


class HasEnoughMoneyState(VendingMachineState):
    def select_product(self, machine: "VendingMachine", code: str) -> None:
        raise RuntimeError("payment already sufficient; dispense or cancel")

    def insert_coin(self, machine: "VendingMachine", cents: int) -> None:
        machine.balance += cents  # allow overpaying; extra is refunded on dispense

    def dispense(self, machine: "VendingMachine") -> str:
        product = machine.inventory.get(machine.selected_code)
        assert product is not None
        machine.inventory.decrement(product.code)
        change = machine.balance - product.price
        machine.balance = 0
        machine.selected_code = None
        next_state = SoldOutState() if product.quantity - 1 <= 0 else IdleState()
        machine.set_state(next_state)
        return f"Dispensed {product.name}, change: {change} cents"

    def cancel(self, machine: "VendingMachine") -> None:
        machine.refund()
        machine.set_state(IdleState())


class SoldOutState(VendingMachineState):
    def select_product(self, machine: "VendingMachine", code: str) -> None:
        product = machine.inventory.get(code)
        if product and product.quantity > 0:
            machine.selected_code = code
            machine.set_state(HasSelectionState())
        else:
            raise RuntimeError("product sold out")

    def insert_coin(self, machine: "VendingMachine", cents: int) -> None:
        raise RuntimeError("product sold out")

    def dispense(self, machine: "VendingMachine") -> str:
        raise RuntimeError("product sold out")

    def cancel(self, machine: "VendingMachine") -> None:
        pass


class VendingMachine:
    def __init__(self, inventory: Inventory):
        self.inventory = inventory
        self.balance = 0
        self.selected_code: str | None = None
        self._state: VendingMachineState = IdleState()

    def set_state(self, state: VendingMachineState) -> None:
        self._state = state

    def refund(self) -> int:
        amount, self.balance = self.balance, 0
        return amount

    def select_product(self, code: str) -> None:
        self._state.select_product(self, code)

    def insert_coin(self, cents: int) -> None:
        self._state.insert_coin(self, cents)

    def dispense(self) -> str:
        return self._state.dispense(self)

    def cancel(self) -> int:
        refunded = self.balance
        self._state.cancel(self)
        return refunded


if __name__ == "__main__":
    inventory = Inventory({
        "A1": Product("A1", "Chips", price=150, quantity=1),
        "B2": Product("B2", "Soda", price=125, quantity=0),
    })
    machine = VendingMachine(inventory)

    machine.select_product("A1")
    machine.insert_coin(100)
    assert isinstance(machine._state, HasSelectionState)
    machine.insert_coin(100)
    assert isinstance(machine._state, HasEnoughMoneyState)
    result = machine.dispense()
    assert "Chips" in result and "50" in result
    assert isinstance(machine._state, SoldOutState)  # last unit consumed

    try:
        machine.select_product("A1")
        assert False, "should have raised on sold-out product"
    except RuntimeError:
        pass

    machine2 = VendingMachine(Inventory({"A1": Product("A1", "Chips", 150, 3)}))
    machine2.select_product("A1")
    machine2.insert_coin(50)
    refunded = machine2.cancel()
    assert refunded == 50
    assert isinstance(machine2._state, IdleState)

    print("All tests passed.")
