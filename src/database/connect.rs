use mysql::*;
use mysql::prelude::*;

pub fn main() {
    let database_result = Database::new();
    if let Ok(mut database) = database_result {
        database.insert();
    }
}

pub struct Database {
    conn: PooledConn,
}

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

impl Database {
    pub fn new() -> Result<Database> {
        let url = "mysql://root:yt66@localhost:3306/rust";
        let opts = Opts::from_url(url).unwrap();
        let pool = Pool::new(opts);
        match pool {
            Ok(pool) => {
                let conn = pool.get_conn().unwrap();
                Ok(Database { conn })
            }
            Err(error) => {
                println!("{}", error);
                Err(error)
            }
        }
    }

    pub fn insert(&mut self) {
        let result = self.exec_batch();
        if let Err(error) = result {
            println!("{}", error);

            //创建表
            let create_result = self.conn.query_drop(
                r"CREATE TEMPORARY TABLE payment (
                customer_id int not null,
                amount int not null,
                account_name text
            )");
            if let Err(error) = create_result {
                println!("{}", error);
            }


            if let Err(error) = self.exec_batch() {
                println!("----------------");
                println!("{}", error);
            }
        }


        println!("Yay!");
    }

    fn exec_batch(&mut self) -> Result<()> {
        // Now let's insert payments to the database
        let payments = vec![
            Payment { customer_id: 1, amount: 2, account_name: None },
            Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
            Payment { customer_id: 5, amount: 6, account_name: None },
            Payment { customer_id: 7, amount: 8, account_name: None },
            Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
        ];

        self.conn.exec_batch(
            r"INSERT INTO payment (customer_id, amount, account_name) VALUES (:customer_id, :amount, :account_name)",
            payments.iter().map(|p| params! {
        "customer_id" => p.customer_id,
        "amount" => p.amount,
        "account_name" => &p.account_name,
            }),
        )
    }
}

