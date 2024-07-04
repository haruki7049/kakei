use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DataTable {
    expenses: Vec<Price>,
    incomes: Vec<Price>,
}

impl std::default::Default for DataTable {
    fn default() -> Self {
        DataTable {
            expenses: Vec::new(),
            incomes: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Price {
    name: String,
    price: isize,
    unit: Unit,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Unit {
    Yen,
    Dollar,
}

#[cfg(test)]
mod tests {
    use super::DataTable;
    use super::Price;
    use super::Unit;

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

    #[test]
    fn test_data_table_initialize() {
        let data: DataTable = DataTable::default();

        assert_eq!(data.expenses.len(), 0);
        assert_eq!(data.incomes.len(), 0);
        assert_eq!(
            toml::to_string(&data).unwrap(),
            "expenses = []\nincomes = []\n"
        );
    }
}
