use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
struct Transaction {
    amount: f64,
    description: String,
}

type Summary = (f64, usize);

impl FromIterator<Transaction> for Summary {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Transaction>,
    {
        let mut total_amount = 0.0;
        let mut total_transactions = 0;

        for transaction in iter {
            total_amount += transaction.amount;
            total_transactions += 1;
        }

        return (total_amount, total_transactions);
    }
}

impl FromIterator<Transaction> for HashMap<String, f64> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Transaction>,
    {
        let mut result = HashMap::new();

        for transaction in iter {
            result.insert(transaction.description, transaction.amount);
        }

        result
    }
}

fn main() {
    println!("\n\n---------------------------------\n\n");
    let transactions = vec![
        Transaction {
            amount: 100.50,
            description: "Groceries".to_string(),
        },
        Transaction {
            amount: 40.90,
            description: "Gas".to_string(),
        },
        Transaction {
            amount: 60.99,
            description: "Internet Bill".to_string(),
        },
    ];

    let summary: Summary = transactions.into_iter().collect();

    println!("the summary {:.2?}", summary);

    println!("\n\n---------------------------------\n\n");

    let transactions2 = vec![
        Transaction {
            amount: 100.50,
            description: "Groceries".to_string(),
        },
        Transaction {
            amount: 40.90,
            description: "Gas".to_string(),
        },
        Transaction {
            amount: 60.99,
            description: "Internet Bill".to_string(),
        },
    ];

    let transaction2_map: HashMap<String, f64> = transactions2.into_iter().collect();

    println!("transaction2_map {:.2?}", transaction2_map)
}
