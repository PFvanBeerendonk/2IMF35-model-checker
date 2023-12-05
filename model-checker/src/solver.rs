use crate::types::ltl::Ltl;
use crate::types::formula::Formula;
use crate::types::formula::Operator;
use crate::types::formula::Node;

use std::collections::HashSet;



// Given a formula `f`, evaluate it over `ltl`
pub fn execute(f: Formula, instance:Ltl) {
    println!("Initializing");
    // let mut set = init(f, instance);
    
}

fn eval(node: Node) -> HashSet<String> {
    match node {
        Node::Variable(string) => {
            // X_i, TODO not sure about array yet.
            return HashSet::new();
        }
        Node::BinaryExpr { op, lhs, rhs } => {
            if op == Operator::Conjunction {
                // Not sure if this can be done cleaner tbh, maybe hashset intersection isn't so nice after all...
                let eval_lhs = eval(*lhs);
                let eval_rhs = eval(*rhs);
                return eval_lhs.intersection(&eval_rhs).map(|x| x.to_string()).collect::<HashSet<String>>();
            } else if op == Operator::Disjunction {
                let eval_lhs = eval(*lhs);
                let eval_rhs = eval(*rhs);
                return eval_lhs.union(&eval_rhs).map(|x| x.to_string()).collect::<HashSet<String>>();
            } else {
                panic!("Not implemented yet");
            }
            return HashSet::new();
        }
        Node::UnaryExpr { op, child } => {
            if op == Operator::Negate {
                let eval_child = eval(*child);
                let all_states: HashSet<String> = HashSet::new(); // TODO
                return all_states.difference(&eval_child).map(|x| x.to_string()).collect::<HashSet<String>>();
            } else if op == Operator::SimpleTrue {
                let all_states: HashSet<String> = HashSet::new(); // TODO
                return all_states;
            } else if op == Operator::SimpleFalse {
                // Empty set.
                return HashSet::new();
            } else {
                panic!("Not implemented yet");
            }
            return HashSet::new();
        }
    }
}

pub fn execute_improved(f: Formula, instance:Ltl) {

}


// Uses functions:
// fn init(f:Formula, instance:Ltl) -> (HashSet<>) {
//     let mut set = HashSet::new();

//     return (set);
// }

// fn eval(f:Formula, instance:Ltl) {

// }