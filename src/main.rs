
#[derive(Debug)]
pub struct Block{
    pub id:usize,
    pub hash:String,
    pub previous_hash:String,
    pub timestamp:i64,
    pub txn:Vec<String>
}


fn main() {
    let test= Block{
       id:1,
       hash:"dada".to_string(),
       previous_hash:"dada".to_string(),
       timestamp:31,
       txn:vec!["dada".to_string()]
    };
    println!("{:?}",test);
}
