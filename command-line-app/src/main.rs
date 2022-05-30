use std::collections::HashMap;
fn main() {
    
   let mut args = std::env::args().skip(1);
   let key = args.next().expect("No Key was passed as an argument");
   let value = args.next().expect("No Value was passed as an argument");

   let mut database = Database::new("cl.db").expect("Databas::new() crashed.");
   
   database.insert(key.to_uppercase(), value.clone());
   database.insert(key, value);
   database.flush().expect("Couldn't dump values into database file.");
}

struct Database {
   map : HashMap<String, String>,
   filename : String,
}

impl Database {
      fn new(_filename : &str) -> Result<Database, std::io::Error> {
         if !std::path::Path::new(_filename).exists(){
            std::fs::File::create(_filename).expect("Couldn't create file");
         }
         
         let contents = std::fs::read_to_string(_filename)?;

         let mut map = HashMap::new();
         //Parse the String
         for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt Database");
            
            //insert the keys-value pair read (copy the key and creaate and and owned value).
            map.insert(key.to_owned(), value.to_owned());
         }
         

         Ok(Database {
            map : map,
            filename : _filename.to_owned()
         })
      }

      fn insert(&mut self, _key : String, _val : String){
         //Create a file to store the values.

         self.map.insert(_key, _val);

      }

      //API design decission: We don't want the user to insert values once they have flush the values into the database.
      //This is why we use self instead of &self so we take the ownership of database.
      fn flush(self) -> Result<(), std::io::Error>{
         let mut contents = String::new();

         for pairs in self.map {
            let pair = format!("{}\t{}\n", pairs.0, pairs.1);
            contents.push_str(&pair);
         }

         std::fs::write(&self.filename, contents)
      }

}