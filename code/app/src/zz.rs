fn main() {
    println!("Hello, world!");

    //let agg = TheAggregate { state: None };

    let mut agg2: TheAggregate = TheAggregate::open(933);

    agg2.deposit(10);

    assert_eq!(agg2.state.balance, 10);
    assert_eq!(agg2.generation, 1);

    //---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    agg2.deposit(10);

    assert_eq!(agg2.state.balance, 20);
    assert_eq!(agg2.generation, 2);

    //---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    agg2.deposit(10);

    assert_eq!(agg2.state.balance, 30);
    assert_eq!(agg2.generation, 3);

    println!("Got money!{}", agg2.state.balance);
}

#[derive(Debug, Copy, Clone)]
struct InternalState {
    id: u64,
    balance: u64,
}

#[derive(Debug, Copy, Clone)]
struct TheAggregate {
    state: InternalState,
    generation: u64,
}

impl TheAggregate {
    fn open(id: u64) -> TheAggregate {
        TheAggregate {
            state: InternalState { id: id, balance: 0 },
            generation: 0,
        }
    }

    fn deposit(&mut self, amount: u64) {
        self.state.balance += amount;
        self.generation += 1;
    }
}
