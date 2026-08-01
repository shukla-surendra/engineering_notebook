//! 3. Vending Machine
//! Textbook State pattern: VendingMachine (context) delegates to its current state.
//!
//! Same self-referential-mutation problem as the elevator's motion state (a state
//! transition needs `&mut VendingMachine`, which conflicts with the trait-object state
//! living inside the very struct being mutated) — see elevator_rusty's comment for the
//! full explanation. Translated the same way: a closed `enum` plus `match`, one arm per
//! state, instead of a trait-object state hierarchy.
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Product {
    #[allow(dead_code)]
    code: String,
    name: String,
    price: i64, // cents
    quantity: i64,
}

struct Inventory {
    products: HashMap<String, Product>,
}

impl Inventory {
    fn new(products: HashMap<String, Product>) -> Self {
        Inventory { products }
    }

    fn get(&self, code: &str) -> Option<&Product> {
        self.products.get(code)
    }

    fn decrement(&mut self, code: &str) {
        if let Some(p) = self.products.get_mut(code) {
            p.quantity -= 1;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MachineState {
    Idle,
    HasSelection,
    HasEnoughMoney,
    SoldOut,
}

struct VendingMachine {
    inventory: Inventory,
    balance: i64,
    selected_code: Option<String>,
    state: MachineState,
}

impl VendingMachine {
    fn new(inventory: Inventory) -> Self {
        VendingMachine {
            inventory,
            balance: 0,
            selected_code: None,
            state: MachineState::Idle,
        }
    }

    fn refund(&mut self) -> i64 {
        let amount = self.balance;
        self.balance = 0;
        amount
    }

    fn select_product(&mut self, code: &str) -> Result<(), String> {
        match self.state {
            MachineState::Idle => {
                let product = self
                    .inventory
                    .get(code)
                    .ok_or_else(|| format!("no such product: {code}"))?;
                if product.quantity <= 0 {
                    self.state = MachineState::SoldOut;
                    return Ok(()); // silently transitions, matching solution.py's IdleState
                }
                self.selected_code = Some(code.to_string());
                self.state = MachineState::HasSelection;
                Ok(())
            }
            MachineState::HasSelection => {
                // Matches solution.py exactly: no existence check here, just a change
                // of mind — "allow changing the mind before paying".
                self.selected_code = Some(code.to_string());
                Ok(())
            }
            MachineState::HasEnoughMoney => {
                Err("payment already sufficient; dispense or cancel".to_string())
            }
            MachineState::SoldOut => {
                let available = self.inventory.get(code).is_some_and(|p| p.quantity > 0);
                if available {
                    self.selected_code = Some(code.to_string());
                    self.state = MachineState::HasSelection;
                    Ok(())
                } else {
                    Err("product sold out".to_string())
                }
            }
        }
    }

    fn insert_coin(&mut self, cents: i64) -> Result<(), String> {
        match self.state {
            MachineState::Idle => Err("select a product before inserting coins".to_string()),
            MachineState::HasSelection => {
                self.balance += cents;
                let code = self
                    .selected_code
                    .clone()
                    .expect("HasSelection state always has a selected code");
                let price = self
                    .inventory
                    .get(&code)
                    .expect("selected product must exist")
                    .price;
                if self.balance >= price {
                    self.state = MachineState::HasEnoughMoney;
                }
                Ok(())
            }
            MachineState::HasEnoughMoney => {
                self.balance += cents; // overpaying is allowed; refunded on dispense
                Ok(())
            }
            MachineState::SoldOut => Err("product sold out".to_string()),
        }
    }

    fn dispense(&mut self) -> Result<String, String> {
        match self.state {
            MachineState::HasEnoughMoney => {
                let code = self
                    .selected_code
                    .clone()
                    .expect("HasEnoughMoney state always has a selected code");
                let (name, price) = {
                    let product = self
                        .inventory
                        .get(&code)
                        .expect("selected product must exist");
                    (product.name.clone(), product.price)
                };
                self.inventory.decrement(&code);
                let change = self.balance - price;
                self.balance = 0;
                self.selected_code = None;

                // Mirrors solution.py's exact check: `product.quantity` there already
                // reflects the decrement (Python passes the same object by reference),
                // so this reads the post-decrement quantity and subtracts one more.
                let remaining = self.inventory.get(&code).unwrap().quantity;
                self.state = if remaining - 1 <= 0 {
                    MachineState::SoldOut
                } else {
                    MachineState::Idle
                };
                Ok(format!("Dispensed {name}, change: {change} cents"))
            }
            MachineState::Idle => Err("no product selected".to_string()),
            MachineState::HasSelection => Err("insert more coins first".to_string()),
            MachineState::SoldOut => Err("product sold out".to_string()),
        }
    }

    fn cancel(&mut self) -> i64 {
        // solution.py captures the balance before delegating to the state's cancel(),
        // so Idle/SoldOut (no-ops) report whatever balance existed rather than 0.
        let refunded = self.balance;
        match self.state {
            MachineState::HasSelection | MachineState::HasEnoughMoney => {
                self.refund();
                self.state = MachineState::Idle;
            }
            MachineState::Idle | MachineState::SoldOut => {}
        }
        refunded
    }
}

fn main() {
    let inventory = Inventory::new(HashMap::from([(
        "A1".to_string(),
        Product {
            code: "A1".to_string(),
            name: "Chips".to_string(),
            price: 150,
            quantity: 1,
        },
    )]));
    let mut machine = VendingMachine::new(inventory);
    machine.select_product("A1").unwrap();
    machine.insert_coin(150).unwrap();
    println!("{}", machine.dispense().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_python_reference_behavior() {
        let inventory = Inventory::new(HashMap::from([
            (
                "A1".to_string(),
                Product {
                    code: "A1".to_string(),
                    name: "Chips".to_string(),
                    price: 150,
                    quantity: 1,
                },
            ),
            (
                "B2".to_string(),
                Product {
                    code: "B2".to_string(),
                    name: "Soda".to_string(),
                    price: 125,
                    quantity: 0,
                },
            ),
        ]));
        let mut machine = VendingMachine::new(inventory);

        machine.select_product("A1").unwrap();
        machine.insert_coin(100).unwrap();
        assert_eq!(machine.state, MachineState::HasSelection);
        machine.insert_coin(100).unwrap();
        assert_eq!(machine.state, MachineState::HasEnoughMoney);
        let result = machine.dispense().unwrap();
        assert!(result.contains("Chips") && result.contains("50"));
        assert_eq!(machine.state, MachineState::SoldOut); // last unit consumed

        assert!(machine.select_product("A1").is_err());

        let inventory2 = Inventory::new(HashMap::from([(
            "A1".to_string(),
            Product {
                code: "A1".to_string(),
                name: "Chips".to_string(),
                price: 150,
                quantity: 3,
            },
        )]));
        let mut machine2 = VendingMachine::new(inventory2);
        machine2.select_product("A1").unwrap();
        machine2.insert_coin(50).unwrap();
        let refunded = machine2.cancel();
        assert_eq!(refunded, 50);
        assert_eq!(machine2.state, MachineState::Idle);
    }
}
