// Structures allow for managing incoming and out going data, a place to store data
// for later use it also allows us to guard against malformed data.
//

#[derive(Debug)]
pub struct Data {
   pub name: String,
   pub age: u8,
}


// Enumerators or Enums allow to classify data into meaningful tokens
//
#[derive(Debug)]
pub enum Direction {
        North,
        East,
        South,
        West,
    }

