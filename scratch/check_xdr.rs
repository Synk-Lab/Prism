use stellar_xdr::curr::{ScVal, WriteXdr, Limits};

fn main() {
    let val = ScVal::Void;
    let bytes = val.to_xdr(Limits::none()).unwrap();
    println!("{:?}", bytes);
    
    let val_false = ScVal::Bool(false);
    let bytes_false = val_false.to_xdr(Limits::none()).unwrap();
    println!("Bool(false): {:?}", bytes_false);
}
