use seed::{prelude::*, *};
use std::marker::PhantomData;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        budget: Budget {
            transactions: vec![
                Transaction::new(1234.0),
                Transaction::new(-50.),
                Transaction::new(8.),
            ]
        }
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    budget: Budget<Euro>,
}

// ------ Budget -------

struct Budget<C: Currency> {
    transactions: Vec<Transaction<C>>
}

// ------ Transaction -------

struct Transaction<C: Currency> {
    amount: f64,
    _currency: PhantomData<C>,
}

impl<C: Currency> Transaction<C> {
    fn new(amount: f64) -> Self {
        Self {
            amount,
            _currency: PhantomData
        }
    }
}

// ------ Currencies -------

trait Currency {}

struct Euro;
impl Currency for Euro {}

// ------ ------
//    Update
// ------ ------

enum Msg {
}

fn update(_: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    table![C!["transaction-table"],
        caption![C!["transaction-caption"],
            "Your Transactions"
        ],
        tr![C!["transaction-row"],
            th![C!["transaction-header"],
                "Amount"
            ],
        ],
        model.budget.transactions.iter().map(view_transaction),
    ]
}

fn view_transaction<C: Currency>(transaction: &Transaction<C>) -> Node<Msg> {
    tr![class!["transaction-row"],
        td![
            &transaction.amount,
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
