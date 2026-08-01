# LLD Fundamentals: OOP Pillars, SOLID, and Design Patterns from First Principles

[`OOD_FRAMEWORK.md`](OOD_FRAMEWORK.md) covers the *process* — the step-by-step sequence
you'd narrate in a live LLD round. [`LLD_VS_HLD.md`](LLD_VS_HLD.md) covers the *checklist*
— a compact bullet inventory of every concept and pattern, and the full question bank. This
doc is neither of those: it's the primer underneath both — each pillar, principle, and
pattern taught problem → mechanism → why it matters, with a short runnable code example,
for anyone who wants to actually understand *why* the checklist items work before reciting
them in a room. Read this once; `OOD_FRAMEWORK.md` and `LLD_VS_HLD.md` should feel like a
natural continuation afterward, not a jump.

Examples are Python, matching this repo's `solution.py` convention elsewhere.

## The Four OOP Pillars

### Encapsulation

**Problem**: if every field on an object is freely readable and writable from outside, the
object can be pushed into an invalid state by code that has no idea what "valid" means for
that object — a bank account balance set negative by a caller that never should have had
direct write access to it.

**Mechanism**: hide internal state behind a public interface (methods), and let the object
itself enforce its own invariants on every mutation.

```python
class BankAccount:
    def __init__(self, balance: float = 0.0):
        self._balance = balance  # "private" by convention

    def withdraw(self, amount: float) -> None:
        if amount > self._balance:
            raise ValueError("insufficient funds")
        self._balance -= amount

    @property
    def balance(self) -> float:
        return self._balance
```

**Why it matters**: `_balance` can only change through `withdraw`/`deposit`-style methods
that the class controls, so "can this object ever be invalid" becomes a question you can
answer by reading one class instead of auditing every caller. This is the property every
other pillar and every SOLID principle below quietly assumes holds.

### Abstraction

**Problem**: a caller of `PaymentProcessor` shouldn't need to know whether it's calling
Stripe, a bank API, or a mock in a test — coupling calling code to a concrete
implementation makes every implementation swap a change to every call site.

**Mechanism**: define an interface (in Python, an `ABC` or a `Protocol`) that states *what*
a class does; concrete classes state *how*.

```python
from abc import ABC, abstractmethod

class PaymentProcessor(ABC):
    @abstractmethod
    def charge(self, amount: float) -> bool: ...

class CreditCardProcessor(PaymentProcessor):
    def charge(self, amount: float) -> bool:
        return True  # real implementation elsewhere
```

**Why it matters**: this is the mechanism that makes Dependency Inversion (below) and the
Strategy pattern possible at all — without an abstraction to depend on, "swap the
implementation without touching the caller" has nothing to hang on.

### Inheritance (is-a)

**Problem**: `Car` and `Motorcycle` share behavior (`start_engine`, `park`) — writing it
twice is both duplication and a maintenance trap (fix a bug in one, forget the other).

**Mechanism**: a subclass inherits behavior from a parent it has a genuine **is-a**
relationship with.

```python
class Vehicle:
    def start_engine(self) -> None:
        print("engine started")

class Car(Vehicle):
    pass  # inherits start_engine for free
```

**Why it matters, and the trap**: inheritance is the *most misused* tool in this round —
reached for whenever two classes merely share some code, even without a true is-a
relationship. `Motorcycle(Vehicle)` is fine; `ParkingSpot(Vehicle)` to "reuse a size field"
is not. The test is Liskov Substitution (below): if a subclass can't be used everywhere its
parent is expected without surprising behavior, the relationship is wrong — reach for
composition instead.

### Polymorphism

**Problem**: calling code that branches on type (`if isinstance(v, Car): ... elif
isinstance(v, Motorcycle): ...`) grows a new branch for every new type forever, and it's
exactly the `if/elif` chain Open/Closed (below) exists to eliminate.

**Mechanism**: calling code invokes a method through a shared interface; each concrete type
supplies its own behavior for that same call.

```python
class Vehicle(ABC):
    @abstractmethod
    def required_spots(self) -> int: ...

class Car(Vehicle):
    def required_spots(self) -> int:
        return 1

class Truck(Vehicle):
    def required_spots(self) -> int:
        return 2

def assign(vehicle: Vehicle) -> None:
    spots_needed = vehicle.required_spots()  # no branching on type at all
```

**Why it matters**: this is the language feature that makes the Strategy and State patterns
possible — "swap the algorithm/state without touching the caller" is polymorphism applied
to a specific design problem, not a separate idea.

---

## SOLID, Each Shown as a Violation Then a Fix

### Single Responsibility Principle

**Problem**: a class with more than one reason to change becomes hard to modify safely —
changing how payment works risks breaking notification, because they live in the same
class.

```python
# Violation: one class does assignment, payment, AND notification
class ParkingLot:
    def park_and_charge_and_notify(self, vehicle): ...

# Fix: split by responsibility
class SpotAssigner:
    def assign(self, vehicle): ...

class PaymentService:
    def charge(self, vehicle): ...

class NotificationService:
    def notify(self, vehicle): ...
```

**Why it matters**: the fastest way to fail this round is a god-class where every
requirement lands as an edit to the same file — SRP is what keeps step 7 of the LLD process
(the "now add X" follow-up) cheap.

### Open/Closed Principle

**Problem**: a spot-assignment algorithm implemented as a growing `if/elif` chain requires
editing `ParkingLot` itself every time a new strategy (e.g. "prefer spots near the
elevator") is added.

```python
# Violation
class ParkingLot:
    def assign(self, vehicle, strategy: str):
        if strategy == "nearest":
            ...
        elif strategy == "prefer_covered":
            ...
        # every new strategy edits this method

# Fix: depend on an interface, add new strategies as new classes
class SpotAssignmentStrategy(ABC):
    @abstractmethod
    def assign(self, vehicle, spots): ...

class NearestSpotStrategy(SpotAssignmentStrategy):
    def assign(self, vehicle, spots): ...

class ParkingLot:
    def __init__(self, strategy: SpotAssignmentStrategy):
        self._strategy = strategy
```

**Why it matters**: open for extension (add a new `SpotAssignmentStrategy` subclass), closed
for modification (`ParkingLot` never changes). This *is* the Strategy pattern — Open/Closed
is the principle, Strategy is the pattern that implements it.

### Liskov Substitution Principle

**Problem**: a subclass that breaks an assumption every other sibling satisfies makes
calling code unsafe to write generically — callers can no longer trust the parent type's
contract.

```python
# Violation: every Vehicle is assumed to need exactly one spot, but this one doesn't
class Vehicle(ABC):
    def required_spots(self) -> int:
        return 1

class Bicycle(Vehicle):
    def required_spots(self) -> int:
        raise NotImplementedError("bicycles use racks, not spots")  # breaks the contract
```

**Fix**: this isn't a bug to patch with a special case — it's a signal the hierarchy itself
is wrong. `Bicycle` doesn't belong under `Vehicle` if it can't honor `Vehicle`'s contract; it
needs its own type (or `Vehicle`'s contract needs to change for *everyone*, not be
special-cased for one subclass).

**Why it matters**: LSP is the test for whether inheritance was the right call in the first
place — "does the subclass work everywhere the parent is expected, with no surprises" is
more precise than "do they share some fields."

### Interface Segregation Principle

**Problem**: a fat interface forces implementers to define methods they don't need,
usually with a stub that throws — a signal the interface actually models two unrelated
capabilities.

```python
# Violation: not every vehicle can be towed AND charged
class Vehicle(ABC):
    @abstractmethod
    def start_engine(self): ...
    @abstractmethod
    def charge_battery(self): ...  # gas-powered Car has no business implementing this

# Fix: split into capability-specific interfaces
class Drivable(ABC):
    @abstractmethod
    def start_engine(self): ...

class Chargeable(ABC):
    @abstractmethod
    def charge_battery(self): ...

class ElectricCar(Drivable, Chargeable): ...
class GasCar(Drivable): ...
```

**Why it matters**: a class should never be forced to implement (or raise
`NotImplementedError` from) a method it has no meaningful behavior for — that's the tell
this principle was skipped.

### Dependency Inversion Principle

**Problem**: a high-level coordinator that directly instantiates a concrete low-level class
is welded to that specific implementation — swapping `CreditCardPayment` for
`WalletPayment` means editing `ParkingLot` itself.

```python
# Violation
class ParkingLot:
    def __init__(self):
        self.payment = CreditCardPayment()  # concrete dependency, hardcoded

# Fix: depend on the abstraction; inject the concrete implementation
class ParkingLot:
    def __init__(self, payment: PaymentProcessor):  # depends on the interface
        self.payment = payment

lot = ParkingLot(payment=WalletPayment())  # swap freely, ParkingLot never changes
```

**Why it matters**: this is what makes Open/Closed's "add a new implementation without
editing the caller" actually possible in practice — high-level modules depend on
abstractions, concrete implementations depend on those same abstractions, and nothing
depends on a concrete class except the line that constructs it.

---

## Design Patterns, Problem → Mechanism → Code

### Creational

**Singleton** — *problem*: exactly one `ParkingLot` coordinator should exist; two instances
would each think they own the full set of spots. *Mechanism*: control construction so only
one instance is ever created.

```python
class ParkingLot:
    _instance: "ParkingLot | None" = None

    def __new__(cls):
        if cls._instance is None:
            cls._instance = super().__new__(cls)
        return cls._instance
```
Watch for thread-safety in the check-then-create above — under concurrent first access,
guard it with a lock.

**Factory Method** — *problem*: calling code that does `Car()` vs. `Motorcycle()` directly
is coupled to every concrete vehicle type it might ever create. *Mechanism*: centralize
creation behind one method that returns the abstraction.

```python
class VehicleFactory:
    @staticmethod
    def create(kind: str) -> Vehicle:
        return {"car": Car, "motorcycle": Motorcycle}[kind]()
```

**Builder** — *problem*: a class with many optional fields (`Pizza(size, crust, cheese,
toppings, ...)`) makes constructor calls unreadable and error-prone (positional args in the
wrong order). *Mechanism*: build the object step by step through a fluent interface.

```python
class Pizza:
    def __init__(self):
        self.toppings: list[str] = []

class PizzaBuilder:
    def __init__(self):
        self._pizza = Pizza()

    def add_topping(self, topping: str) -> "PizzaBuilder":
        self._pizza.toppings.append(topping)
        return self

    def build(self) -> Pizza:
        return self._pizza

pizza = PizzaBuilder().add_topping("cheese").add_topping("olives").build()
```

### Structural

**Adapter** — *problem*: an existing `LegacyPrinter.print_old(text)` doesn't match the
`Printer.print(text)` interface your code expects, and you can't change the legacy class.
*Mechanism*: wrap it in a class that translates one interface into the other.

```python
class Printer(ABC):
    @abstractmethod
    def print(self, text: str) -> None: ...

class LegacyPrinterAdapter(Printer):
    def __init__(self, legacy: LegacyPrinter):
        self._legacy = legacy

    def print(self, text: str) -> None:
        self._legacy.print_old(text)
```

**Decorator** — *problem*: a `Coffee` with every combination of add-ons (`MilkCoffee`,
`MilkSugarCoffee`, `MilkSugarWhipCoffee`, ...) explodes combinatorially as a class
hierarchy. *Mechanism*: wrap the object at runtime, layering behavior.

```python
class Coffee(ABC):
    @abstractmethod
    def cost(self) -> float: ...

class SimpleCoffee(Coffee):
    def cost(self) -> float:
        return 2.0

class MilkDecorator(Coffee):
    def __init__(self, coffee: Coffee):
        self._coffee = coffee

    def cost(self) -> float:
        return self._coffee.cost() + 0.5

order = MilkDecorator(SimpleCoffee())  # $2.50, no new class needed
```

**Composite** — *problem*: a filesystem has `File`s and `Folder`s (which contain more files
and folders), and calling code shouldn't need to special-case "is this a file or a folder"
every time it computes total size. *Mechanism*: give both a shared interface so they can be
treated uniformly.

```python
class FileSystemNode(ABC):
    @abstractmethod
    def size(self) -> int: ...

class File(FileSystemNode):
    def __init__(self, size: int):
        self._size = size
    def size(self) -> int:
        return self._size

class Folder(FileSystemNode):
    def __init__(self):
        self.children: list[FileSystemNode] = []
    def size(self) -> int:
        return sum(c.size() for c in self.children)  # recurses into sub-folders for free
```

**Facade** — *problem*: starting a car requires coordinating the ignition, fuel injection,
and starter motor subsystems — exposing all three to calling code leaks implementation
detail and invites misuse. *Mechanism*: expose one simple method that hides the
orchestration.

```python
class CarFacade:
    def __init__(self):
        self._ignition = Ignition()
        self._fuel = FuelInjector()
        self._starter = StarterMotor()

    def start(self) -> None:
        self._ignition.on()
        self._fuel.inject()
        self._starter.crank()
```

**Proxy** — *problem*: loading a large image eagerly, or hitting a remote/expensive
resource, on every access is wasteful when many accesses never actually need it.
*Mechanism*: a stand-in object controlling access, deferring or gating the real work.

```python
class Image(ABC):
    @abstractmethod
    def display(self) -> None: ...

class RealImage(Image):
    def __init__(self, path: str):
        self._load_from_disk(path)  # expensive
    def display(self) -> None: ...

class ImageProxy(Image):
    def __init__(self, path: str):
        self._path = path
        self._real: RealImage | None = None

    def display(self) -> None:
        if self._real is None:
            self._real = RealImage(self._path)  # loaded only on first real use
        self._real.display()
```

### Behavioral

**Strategy** — *problem*: an algorithm (spot assignment, elevator dispatch, rate-limiting
rule) needs to vary independently of the class that uses it, without an `if/elif` chain.
*Mechanism*: extract the algorithm behind an interface, inject an implementation.
**The single most-reused pattern in this round** — already shown fully under Open/Closed
above.

**State** — *problem*: a `VendingMachine`'s valid operations depend entirely on its current
state (`Idle`, `HasMoney`, `Dispensing`) — encoding that as a field plus branching methods
scatters the state machine's logic across every method. *Mechanism*: model each state as its
own class implementing a shared interface; the context delegates to whichever state is
current.

```python
class VendingState(ABC):
    @abstractmethod
    def insert_coin(self, machine: "VendingMachine") -> None: ...

class IdleState(VendingState):
    def insert_coin(self, machine: "VendingMachine") -> None:
        machine.state = HasMoneyState()

class HasMoneyState(VendingState):
    def insert_coin(self, machine: "VendingMachine") -> None:
        print("coin already inserted")

class VendingMachine:
    def __init__(self):
        self.state: VendingState = IdleState()

    def insert_coin(self) -> None:
        self.state.insert_coin(self)  # delegates; VendingMachine never branches on state
```
**The second most-reused pattern** after Strategy — note the shape is identical
(polymorphism swapping behavior behind an interface); the two patterns differ only in *what*
varies (an algorithm vs. which state you're in) and in that State transitions typically
mutate which implementation is active, while Strategy is usually set once.

**Observer** — *problem*: when a `StockTicker`'s price changes, an unknown number of
displays need to know, and the ticker shouldn't be coupled to concrete display classes.
*Mechanism*: subscribers register interest; the subject notifies all of them on change.

```python
class Observer(ABC):
    @abstractmethod
    def update(self, price: float) -> None: ...

class StockTicker:
    def __init__(self):
        self._observers: list[Observer] = []

    def subscribe(self, observer: Observer) -> None:
        self._observers.append(observer)

    def set_price(self, price: float) -> None:
        for obs in self._observers:
            obs.update(price)  # StockTicker knows nothing about what a Display does with this
```

**Command** — *problem*: an undo/redo stack needs to treat "an action that was taken" as a
first-class object it can store, queue, and reverse — a bare method call can't be stored or
undone. *Mechanism*: encapsulate a request (receiver + action + args) as an object with a
uniform interface.

```python
class Command(ABC):
    @abstractmethod
    def execute(self) -> None: ...
    @abstractmethod
    def undo(self) -> None: ...

class InsertTextCommand(Command):
    def __init__(self, doc, text: str, pos: int):
        self._doc, self._text, self._pos = doc, text, pos

    def execute(self) -> None:
        self._doc.insert(self._pos, self._text)

    def undo(self) -> None:
        self._doc.delete(self._pos, len(self._text))

history: list[Command] = []
cmd = InsertTextCommand(doc, "hello", 0)
cmd.execute()
history.append(cmd)  # undo stack is just a list of Command objects
```

**Chain of Responsibility** — *problem*: a request (a support ticket, a logging call)
should be handled by the first of several possible handlers able to handle it, without the
caller knowing which one that will be. *Mechanism*: link handlers in a chain; each either
handles the request or passes it along.

```python
class Handler(ABC):
    def __init__(self):
        self._next: "Handler | None" = None

    def set_next(self, handler: "Handler") -> "Handler":
        self._next = handler
        return handler

    def handle(self, level: str) -> None:
        if self._next:
            self._next.handle(level)

class DebugHandler(Handler):
    def handle(self, level: str) -> None:
        if level == "DEBUG":
            print("logged at debug")
        else:
            super().handle(level)  # not mine, pass it on
```

**Template Method** — *problem*: several subclasses share an algorithm's overall shape
(`prepare()` → `cook()` → `plate()`) but differ in one or two steps — duplicating the whole
sequence per subclass repeats the parts that are actually identical. *Mechanism*: define the
skeleton in the base class as a non-overridable sequence; defer specific steps to
subclasses.

```python
class Recipe(ABC):
    def make(self) -> None:  # the template — not overridden
        self.prepare()
        self.cook()
        self.plate()

    def prepare(self) -> None:
        print("gather ingredients")  # shared default

    @abstractmethod
    def cook(self) -> None: ...  # each subclass supplies this

    def plate(self) -> None:
        print("plate the dish")  # shared default
```

**Iterator** — *problem*: code that traverses a custom collection (a linked list, a tree)
shouldn't need to know that collection's internal structure to walk it. *Mechanism*: expose
a uniform traversal interface (`__iter__`/`__next__` in Python) independent of the
underlying storage.

```python
class LinkedListIterator:
    def __init__(self, head):
        self._current = head

    def __iter__(self):
        return self

    def __next__(self):
        if self._current is None:
            raise StopIteration
        value = self._current.value
        self._current = self._current.next
        return value
```

---

## How the Relationships (is-a / has-a) Show Up in Code

The single biggest quality signal in this round, per `OOD_FRAMEWORK.md`, is getting
inheritance vs. composition right. In code, the three UML relationship types collapse to:

| UML relationship | Code shape | Lifetime coupling |
|---|---|---|
| Inheritance (is-a) | `class Car(Vehicle):` | Subclass and parent share identity — there's only one object |
| Composition (has-a, owns) | `self.engine = Engine()` created *inside* the owner's `__init__` | The part cannot outlive the whole — a `Car`'s `Engine` is destroyed with the `Car` |
| Aggregation (has-a, references) | `self.driver = driver` where `driver` is passed *in* from outside | The part has independent lifetime — a `Driver` exists before and after any particular `Car` assignment |

Composition and aggregation look almost identical in Python (both are just an attribute
holding another object) — the distinguishing question is always "does this part get
constructed by its owner, or handed to its owner from outside," which is really asking
whether the part can exist independently. That question, asked explicitly for every pair of
related classes, is step 4 of the process in `OOD_FRAMEWORK.md`.

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Mechanism-first framing (good for explaining any single pattern precisely):** "Every
  pattern here reduces to the same move — take something that varies (an algorithm, a
  state, a step in a sequence) and put it behind an interface so the class that uses it
  doesn't have to change when the thing behind the interface does. Strategy varies the
  algorithm, State varies which state you're in, Template Method varies one step of a fixed
  sequence — same mechanism, different axis of variation."
- **Violation-then-fix framing (good for demonstrating SOLID isn't abstract theory):** "I'd
  rather show a principle by naming the violation first — 'this couples the coordinator to
  a concrete payment class' — and then the fix, than recite the principle's definition.
  Showing the failure mode is what proves I understand *why* the principle exists, not just
  its name."
- **Pillars-as-prerequisites framing (good for explaining why patterns work at all):**
  "Every pattern in this doc leans on polymorphism, and polymorphism only works because of
  abstraction — an interface to call through. If I'm asked to justify a pattern from
  scratch, I'd trace it back to the specific OOP pillar it's built on rather than treating
  the pattern as a memorized recipe."

### Vocabulary Builder

- **invariant** (n.) — a condition an object guarantees stays true across its own methods
  (a `BankAccount`'s balance never goes negative); encapsulation is the mechanism that lets
  a class enforce its own invariants.
- **contract** (n.) — the behavior an interface or parent class promises callers can rely
  on; Liskov Substitution is the test for whether a subclass honors its parent's contract.
  *"Bicycle broke the Vehicle contract, so it doesn't belong in that hierarchy."*
- **axis of variation** (n. phrase) — the specific thing a pattern is designed to let vary
  independently (the algorithm, for Strategy; the state, for State) — naming it precisely
  is what separates "I used a pattern" from "I solved the actual problem."
- **"…so the caller never has to change"** — a compact, reusable phrase for justifying any
  abstraction/interface choice without reciting Open/Closed by name.
- **check-then-act race** (n. phrase) — the general shape of a concurrency bug where
  "check a condition" and "act on it" aren't atomic (e.g. Singleton's `if cls._instance is
  None` check racing with construction); naming this shape, not just "there could be a
  race," signals precision.

---

**See also:** [`OOD_FRAMEWORK.md`](OOD_FRAMEWORK.md) for the step-by-step process this
primer's concepts feed into, and [`LLD_VS_HLD.md`](LLD_VS_HLD.md) for the compact checklist
and the full LLD/HLD question bank.
