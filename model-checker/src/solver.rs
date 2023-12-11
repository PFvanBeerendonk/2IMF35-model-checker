use crate::types::ltl::Ltl;
use crate::types::formula::Formula;
use crate::types::formula::Operator;
use crate::types::formula::Node;

use std::collections::HashMap;
use std::collections::HashSet;


// Given a formula `f`, evaluate it over `ltl`
pub fn execute(f: Formula, instance:Ltl) {
    println!("Initializing");
    // let mut set = init(f, instance);
    
}

fn eval(node: Node, instance:&Ltl, variable_map: &mut HashMap<String,HashSet<i64>>) -> HashSet<i64> {
    match node {
        Node::Variable(string) => {
            // X_i, TODO not sure about array yet.
            return HashSet::new();
        }
        Node::BinaryExpr { op, lhs, rhs } => {
            if op == Operator::Conjunction {
                // Not sure if this can be done cleaner tbh, maybe hashset intersection isn't so nice after all...
                let eval_lhs = eval(*lhs, instance, variable_map);
                let eval_rhs = eval(*rhs, instance, variable_map);
                return eval_lhs.intersection(&eval_rhs).map(|x| *x).collect::<HashSet<i64>>();
            } else if op == Operator::Disjunction {
                let eval_lhs = eval(*lhs, instance, variable_map);
                let eval_rhs = eval(*rhs, instance, variable_map);
                return eval_lhs.union(&eval_rhs).map(|x| *x).collect::<HashSet<i64>>();
            } else if op == Operator::LeastFixpoint {
                
            } else if op == Operator::GreatestFixpoint {
                let node = *lhs;
                match node {
                    Node::Variable(string) => {
                        (*variable_map).insert(string.clone(), instance.get_all_states());
                        let mut x_prime: HashSet<i64> = HashSet::new(); 
                        let mut a = retrieve_element(variable_map.get(&string.clone())).clone();
                        while (x_prime != a){
                            x_prime = a.clone();
                            let temp = eval(*rhs.clone(), instance, variable_map);
                            (*variable_map).insert(string.clone(), temp);
                            a = retrieve_element(variable_map.get(&string.clone())).clone();
                        }
                        return retrieve_element(variable_map.get(&string.clone())).clone();
                    }
                    Node::Action(_) => unreachable!(),
                    Node::UnaryExpr { op: _ } => unreachable!(),
                    Node::BinaryExpr { op: _, lhs: _, rhs: _ } => unreachable!(),
                }
            } else {
                panic!("Not implemented yet");
            }
            return HashSet::new();
        }
        Node::UnaryExpr { op } => {
            // if op == Operator::Negate {
            //     let eval_child = eval(*child);
            //     let all_states: HashSet<String> = HashSet::new(); // TODO
            //     return all_states.difference(&eval_child).map(|x| x.to_string()).collect::<HashSet<String>>();
            // } else
            if op == Operator::SimpleTrue {
                let all_states = instance.get_all_states(); // TODO
                return all_states;
            } else if op == Operator::SimpleFalse {
                // Empty set.
                return HashSet::new();
            } else {
                panic!("Not implemented yet");
            }
            return HashSet::new();
        }
        Node::Action(string) => {
            // TODO
            return HashSet::new();
        }
    }
}

fn retrieve_element(element: Option<&HashSet<i64>>) -> &HashSet<i64> {
    match element {
        Some(x) => {
            return x;
        }
        None => {
            panic!("Should not happen");
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