use app::mybank::prelude::*;

#[test]
fn withdrawing_money_emits_debited_event() {
    // Arrange
    let initial_events = vec![
        BankAccountEvent::acc_opened(100, 20),
        BankAccountEvent::credited(100, 49),
    ];
    let withdraw = BankAccountCommand::withdraw(100, 9);
    let expected = Ok(vec![BankAccountEvent::debited(100, 9)]);

    // Act
    let state = BankAccountAggregate::apply_events(initial_events).unwrap();
    let result = BankAccountAggregate::handle(Some(state), withdraw);

    // Assert
    assert_eq!(expected, result);
}

#[test]
fn account_debited_event_will_set_state_correctly() {
    // Arrange
    let events = vec![
        BankAccountEvent::acc_opened(100, 20),
        BankAccountEvent::credited(100, 49),
        BankAccountEvent::debited(100, 9),
    ];
    let expected = Ok(BankAccountState {
        id: 100,
        customer_id: 20,
        balance: 40,
        generation: 3,
    });

    // Act
    let result = BankAccountAggregate::apply_events(events);

    // Assert
    assert_eq!(expected, result);
}
