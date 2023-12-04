use crate::types::ltl::Ltl;
use crate::types::formula::Formula;


// Given a formula `f`, evaluate it over `ltl`
pub fn execute(f: Formula, instance:Ltl) {
    println!("Initialing");
    init(f, instance)
}

pub fn execute_extended(f: Formula, instance:Ltl) {

}


// Uses functions:
fn init(f:Formula, instance:Ltl) {

}

fn eval(f:Formula, instance:Ltl) {
}