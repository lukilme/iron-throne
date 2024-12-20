use serde::{Serialize, Deserialize};
use serde_json::Value;


pub trait DAO: Serialize + for<'de> Deserialize<'de> + Default {
    fn create() {
        let mut db = SINGLETON_INSTANCE.lock().unwrap();
        let _ = db.execute_query(&Self::create_sql_query());
    }

    fn shine(&self) {
        let json = serde_json::to_value(self).unwrap();
        if let Value::Object(mapa) = json {
            for (campo, valor) in mapa {
                println!("Campo: {}, Valor: {}", campo, valor);
            }
        }
    }

    fn save(&self) {
        let json = serde_json::to_value(self).unwrap();
        if let Value::Object(fields) = json {
            let table_name = std::any::type_name::<Self>()
                .rsplit("::")
                .next()
                .unwrap();

            let columns: Vec<String> = fields.keys().cloned().collect();
            let placeholders: Vec<String> = columns.iter().map(|_| "?".to_string()).collect();
            let values: Vec<String> = fields
                .values()
                .map(|v| match v {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    _ => "".to_string(),
                })
                .collect();

            let query = format!(
                "INSERT OR REPLACE INTO {} ({}) VALUES ({});",
                table_name,
                columns.join(", "),
                placeholders.join(", ")
            );

            let mut db = SINGLETON_INSTANCE.lock().unwrap();
            //db.execute_query_with_params(&query, values);
        }
    }

    fn delete(&self) {
        let primary_key_field = Self::primary_key_field();
        let json = serde_json::to_value(self).unwrap();

        if let Value::Object(fields) = json {
            if let Some(primary_key_value) = fields.get(&primary_key_field) {
                let table_name = std::any::type_name::<Self>()
                    .rsplit("::")
                    .next()
                    .unwrap();

                let query = format!(
                    "DELETE FROM {} WHERE {} = ?;",
                    table_name, primary_key_field
                );

                let mut db = SINGLETON_INSTANCE.lock().unwrap();
                //db.execute_query_with_params(&query, vec![primary_key_value.to_string()]);
            } else {
                eprintln!("Primary key field '{}' not found in the struct.", primary_key_field);
            }
        }
    }

    fn create_sql_query() -> String {
        let example = Self::default();
        let json_example = serde_json::to_value(example).unwrap();

        if let Value::Object(mapa) = json_example {
            let mut colunas: Vec<String> = Vec::new();
            let primary_field = Self::primary_key_field();
            let mut has_primary_key = false;

            for (campo, valor) in mapa {
                let tipo_sql = match valor {
                    Value::String(_) => "TEXT",
                    Value::Number(_) => "INTEGER",
                    Value::Bool(_) => "BOOLEAN",
                    _ => "TEXT",
                };

                if campo == primary_field {
                    colunas.push(format!("{} {} PRIMARY KEY", campo, tipo_sql));
                    has_primary_key = true;
                } else {
                    colunas.push(format!("{} {}", campo, tipo_sql));
                }
            }

            if !has_primary_key {
                colunas.push("id INTEGER PRIMARY KEY".to_string());
            }

            let nome_tabela = std::any::type_name::<Self>()
                .rsplit("::")
                .next()
                .unwrap();

            let query = format!(
                "CREATE TABLE IF NOT EXISTS {} (\n    {}\n);",
                nome_tabela,
                colunas.join(",\n    ")
            );
            return query;
        }

        String::new()
    }

    fn primary_key_field() -> String;
}
