use serde::{Deserialize, Serialize};
use serde_json::Value;

pub trait DAO: Serialize + for<'de> Deserialize<'de> + Default {
    fn create(&self) {
        let mut db = SINGLETON_INSTANCE.lock().unwrap();
        let retorno = db.execute_query(&Self::create_sql_query());
        println!("{}", retorno);
    }

    fn delete(&self) {
        println!("Deletando registro: Primary Key {}", self.get_primary_key());
    }

    fn persist(&self) {
        println!(
            "Persistindo registro: Primary Key {}",
            self.get_primary_key()
        );
        let json = serde_json::to_value(self).unwrap();
        if let Value::Object(fields) = json {
            let table_name = std::any::type_name::<Self>().rsplit("::").next().unwrap();

            let columns: Vec<String> = fields.keys().cloned().collect();
            let placeholders: Vec<String> = columns.iter().map(|_| "?".to_string()).collect();
            let _values: Vec<String> = fields
                .values()
                .map(|v| match v {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    _ => "".to_string(),
                })
                .collect();

            let _query = format!(
                "INSERT OR REPLACE INTO {} ({}) VALUES ({});",
                table_name,
                columns.join(", "),
                placeholders.join(", ")
            );

            let mut _db = SINGLETON_INSTANCE.lock().unwrap();
            //db.execute_query_with_params(&query, values);
        }
    }

    fn unpersist(&self) {
        println!(
            "Removendo persistência do registro: Primary Key {}",
            self.get_primary_key()
        );
    }
    fn create_sql_query() -> String {
        let example = Self::default();
        let json_example = serde_json::to_value(example).unwrap();

        if let Value::Object(mapa) = json_example {
            let mut colunas: Vec<String> = Vec::new();
            let primary_field = "Empty".to_string();
            let mut _has_primary_key = false;

            for (campo, valor) in mapa {
                let tipo_sql = match valor {
                    Value::String(_) => "TEXT",
                    Value::Number(_) => "INTEGER",
                    Value::Bool(_) => "BOOLEAN",
                    _ => "TEXT",
                };

                if campo == primary_field {
                    colunas.push(format!("{} {} PRIMARY KEY", campo, tipo_sql));
                    _has_primary_key = true;
                } else {
                    colunas.push(format!("{} {}", campo, tipo_sql));
                }
            }
            let nome_tabela = std::any::type_name::<Self>().rsplit("::").next().unwrap();

            let query = format!(
                "CREATE TABLE IF NOT EXISTS {} (\n    {}\n);",
                nome_tabela,
                colunas.join(",\n    ")
            );
            println!("{}", query);
            return query;
        }
        String::new()
    }

    fn read(&self) {
        println!("Lendo registro: Primary Key {}", self.get_primary_key());
    }

    fn update(&self) {
        println!(
            "Atualizando registro: Primary Key {}",
            self.get_primary_key()
        );
    }

    fn primary_key_field(&self) -> String {
        "Empty".to_string()
    }

    fn get_primary_key(&self) -> String {
        "Empty".to_string()
    }
}
