use serde::{Deserialize, Serialize};

/// contains kakei's sub commands
pub mod commands;

#[derive(Debug, Serialize, Deserialize)]
struct DataTable {
    expenses: Vec<Price>,
    incomes: Vec<Price>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Price {
    name: String,
    price: isize,
    unit: Unit,
}

#[derive(Debug, Serialize, Deserialize)]
enum Unit {
    Yen,
    Dollar,
}

#[cfg(test)]
mod tests {
    use crate::DataTable;
    use crate::Price;
    use crate::Unit;

    #[test]
    fn test_data_table_serialize() {
        let data: DataTable = DataTable {
            expenses: vec![
                Price {
                    name: "food".to_string(),
                    price: 100,
                    unit: Unit::Yen,
                },
                Price {
                    name: "rent".to_string(),
                    price: 200,
                    unit: Unit::Dollar,
                },
            ],
            incomes: vec![
                Price {
                    name: "salary".to_string(),
                    price: 1000,
                    unit: Unit::Yen,
                },
                Price {
                    name: "bonus".to_string(),
                    price: 200,
                    unit: Unit::Dollar,
                },
            ],
        };

        assert_eq!(toml::to_string(&data).unwrap(), "[[expenses]]\nname = \"food\"\nprice = 100\nunit = \"Yen\"\n\n[[expenses]]\nname = \"rent\"\nprice = 200\nunit = \"Dollar\"\n\n[[incomes]]\nname = \"salary\"\nprice = 1000\nunit = \"Yen\"\n\n[[incomes]]\nname = \"bonus\"\nprice = 200\nunit = \"Dollar\"\n");
    }
}
