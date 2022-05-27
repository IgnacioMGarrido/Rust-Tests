use std::collections::HashMap;
fn main() {
    
   let mut args = std::env::args().skip(1);
   let key = args.next().expect("No Key was passed as an argument");
   let value = args.next().expect("No Value was passed as an argument");

   //Create a file to store the values.
   let contents = format!("{}\t{}\n", key, value);
   let write_result = std::fs::write("cl.db", contents);
   
   match write_result {
      Ok(()) => {}
      Err(e) => {
         panic!("Error: {}",e);
      }
   }

   let database = Database::new("cl.db").expect("Databas::new() crashed.");

   
   //read file

   print!("The key is {} = {}", key, value);
}

struct Database {
   map : HashMap<String, String>
}

impl Database {
      fn new(_filename : &str) -> Result<Database, std::io::Error> {
         
         //Read the file
         // let contents = match std::fs::read_to_string(_filename)
         // {
         //    Ok(c) => c,
         //    Err(error) => {
         //       return Err(error);
         //    }
         // };
         // Equivalent to commented code above. Pay attention to the '?'
         let contents = std::fs::read_to_string(_filename)?;

         let mut map = HashMap::new();
         //Parse the String
         for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt Database");
            
            //insert the keys-value pair read (copy the key and creaate and and owned value).
            map.insert(key.to_owned(), value.to_owned());
         }
         

         Ok(Database {
            map : map
         })
      }

      impl insert() {}
}