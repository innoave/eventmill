use bigdecimal::BigDecimal;
use eventmill::{
    Aggregate, AggregateType, DomainEvent, EventType, HandleCommand, NewEvent, WithAggregateId,
};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

const EVENT_NAMESPACE: &str = "event://github.com/innoave/eventmill/examples/banking";

//
// Domain Events
//

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct AccountCreated {
    account_code: String,
    owner: String,
    credit_limit: BigDecimal,
}

impl EventType for AccountCreated {
    fn event_type_version(&self) -> &str {
        "V1"
    }

    fn event_type(&self) -> &str {
        "AccountCreated"
    }

    fn event_source(&self) -> &str {
        EVENT_NAMESPACE
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct MoneyDeposited {
    account_code: String,
    amount: BigDecimal,
}

impl EventType for MoneyDeposited {
    fn event_type_version(&self) -> &str {
        "V1"
    }

    fn event_type(&self) -> &str {
        "CashDeposited"
    }

    fn event_source(&self) -> &str {
        EVENT_NAMESPACE
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct MoneyWithdrawn {
    account_code: String,
    amount: BigDecimal,
}

impl EventType for MoneyWithdrawn {
    fn event_type_version(&self) -> &str {
        "V1"
    }

    fn event_type(&self) -> &str {
        "CashWithdrawn"
    }

    fn event_source(&self) -> &str {
        EVENT_NAMESPACE
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum MoneyTransferred {
    Credit(MoneyWithdrawn),
    Debit(MoneyDeposited),
}

impl EventType for MoneyTransferred {
    fn event_type_version(&self) -> &str {
        "V1"
    }

    fn event_type(&self) -> &str {
        "MoneyTransferred"
    }

    fn event_source(&self) -> &str {
        EVENT_NAMESPACE
    }
}

//
// Aggregates
//

#[derive(Debug, Clone)]
struct BankAccount {
    account_code: String,
    credit_limit: BigDecimal,
    balance: BigDecimal,
}

impl AggregateType for BankAccount {}

impl WithAggregateId for BankAccount {
    type Id = String;

    fn aggregate_id(&self) -> &Self::Id {
        &self.account_code
    }
}

impl Aggregate<MoneyDeposited> for BankAccount {
    fn apply_event(mut self, event: &DomainEvent<MoneyDeposited, Self>) -> Self {
        self.balance += event.payload.amount.clone();
        self
    }
}

impl Aggregate<MoneyWithdrawn> for BankAccount {
    fn apply_event(mut self, event: &DomainEvent<MoneyWithdrawn, Self>) -> Self {
        self.balance -= event.payload.amount.clone();
        self
    }
}

impl Aggregate<MoneyTransferred> for BankAccount {
    fn apply_event(self, event: &DomainEvent<MoneyTransferred, Self>) -> Self {
        match &event.payload {
            MoneyTransferred::Credit(money_withdrawn) => {
                self.apply_event(&event.transmute(money_withdrawn.clone()))
            }
            MoneyTransferred::Debit(money_deposited) => {
                self.apply_event(&event.transmute(money_deposited.clone()))
            }
        }
    }
}

//
// Commands
//

enum BankAccountError {
    BalanceBelowLimit,
}

#[derive(Debug)]
struct CreateAccount {
    account_code: String,
    owner: String,
    credit_limit: BigDecimal,
}

impl HandleCommand<CreateAccount, BankAccount> for () {
    type Event = AccountCreated;
    type Error = BankAccountError;

    fn handle_command(
        &self,
        CreateAccount {
            account_code,
            owner,
            credit_limit,
        }: CreateAccount,
    ) -> Result<Vec<NewEvent<Self::Event, BankAccount>>, Self::Error> {
        let account_created = AccountCreated {
            account_code: account_code.clone(),
            owner,
            credit_limit,
        };
        Ok(vec![(account_code, account_created).into()])
    }
}

#[derive(Debug)]
struct DepositCash {
    account_code: String,
    amount: BigDecimal,
}

impl HandleCommand<DepositCash, BankAccount> for BankAccount {
    type Event = MoneyDeposited;
    type Error = Infallible;

    fn handle_command(
        &self,
        DepositCash {
            account_code,
            amount,
        }: DepositCash,
    ) -> Result<Vec<NewEvent<Self::Event, BankAccount>>, Self::Error> {
        let money_deposited = MoneyDeposited {
            account_code: account_code.clone(),
            amount,
        };
        Ok(vec![(account_code, money_deposited).into()])
    }
}

#[derive(Debug)]
struct WithdrawCash {
    account_code: String,
    amount: BigDecimal,
}

impl HandleCommand<WithdrawCash, BankAccount> for BankAccount {
    type Event = MoneyWithdrawn;
    type Error = BankAccountError;

    fn handle_command(
        &self,
        WithdrawCash {
            account_code,
            amount,
        }: WithdrawCash,
    ) -> Result<Vec<NewEvent<Self::Event, BankAccount>>, Self::Error> {
        if &self.balance - &amount < self.credit_limit {
            Err(BankAccountError::BalanceBelowLimit)
        } else {
            let money_withdrawn = MoneyWithdrawn {
                account_code: account_code.clone(),
                amount,
            };
            Ok(vec![(account_code, money_withdrawn).into()])
        }
    }
}

#[derive(Debug)]
struct TransferMoney {
    creditor_account_code: String,
    debitor_account_code: String,
    amount: BigDecimal,
}

impl HandleCommand<TransferMoney, BankAccount> for (BankAccount, BankAccount) {
    type Event = MoneyTransferred;
    type Error = BankAccountError;

    fn handle_command(
        &self,
        TransferMoney {
            creditor_account_code,
            debitor_account_code,
            amount,
        }: TransferMoney,
    ) -> Result<Vec<NewEvent<Self::Event, BankAccount>>, Self::Error> {
        let (creditor_account, _debitor_account) = if self.0.account_code == creditor_account_code {
            (&self.0, &self.1)
        } else {
            (&self.1, &self.0)
        };

        if &creditor_account.balance - &amount < creditor_account.credit_limit {
            Err(BankAccountError::BalanceBelowLimit)
        } else {
            let credit_event = MoneyTransferred::Credit(MoneyWithdrawn {
                account_code: creditor_account_code.clone(),
                amount: amount.clone(),
            });
            let debit_event = MoneyTransferred::Debit(MoneyDeposited {
                account_code: debitor_account_code.clone(),
                amount,
            });

            Ok(vec![
                (creditor_account_code, credit_event).into(),
                (debitor_account_code, debit_event).into(),
            ])
        }
    }
}

fn main() {}
