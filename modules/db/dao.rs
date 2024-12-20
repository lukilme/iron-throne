use serde::{Serialize, Deserialize};
use serde_json::Value;


pub trait DAO: Serialize + for<'de> Deserialize<'de> + Default {

    fn create(){
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

    fn create_sql_query() -> String {
        let example = Self::default();
        let json_example = serde_json::to_value(example).unwrap();

        if let Value::Object(mapa) = json_example {
            let mut colunas: Vec<String> = Vec::new();
            for (campo, valor) in mapa {
                let tipo_sql = match valor {
                    Value::String(_) => "TEXT",
                    Value::Number(_) => "INTEGER", 
                    Value::Bool(_) => "BOOLEAN",
                    _ => "TEXT", 
                };
                colunas.push(format!("{} {}", campo, tipo_sql));
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
}
