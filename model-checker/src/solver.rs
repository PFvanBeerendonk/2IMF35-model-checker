use crate::types::ltl::Ltl;
use crate::types::formula::Formula;
use crate::types::formula::Operator;
use crate::types::formula::Node;

use std::collections::HashMap;
use std::collections::HashSet;


// Given a formula `f`, evaluate it over `ltl`
pub fn execute(f: Formula, instance:Ltl) -> HashSet<i64> {
    let mut variable_map: HashMap<String,HashSet<i64>> = HashMap::new();
    return eval(f.root_node, &instance, &mut variable_map);
    // let mut set = init(f, instance);
    
}

fn eval(node: Node, instance:&Ltl, variable_map: &mut HashMap<String,HashSet<i64>>) -> HashSet<i64> {
    match node {
        Node::Variable(string) => {
            // X_i, TODO not sure about array yet.
            if (*variable_map).contains_key(&string.clone()) {
                return retrieve_element(variable_map.get(&string.clone())).clone();
            } else {
                // TODO: Look at, do we actually want to insert this element? 
                (*variable_map).insert(string.clone(), HashSet::new());
                return retrieve_element(variable_map.get(&string.clone())).clone();
            }
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
                let node = *lhs;
                match node {
                    Node::Variable(string) => {
                        (*variable_map).insert(string.clone(), HashSet::new());
                        return calculate_fixpoint(string, *rhs, instance, variable_map)
                    }
                    Node::Action(_) => unreachable!(),
                    Node::UnaryExpr { op: _ } => unreachable!(),
                    Node::BinaryExpr { op: _, lhs: _, rhs: _ } => unreachable!(),
                }
            } else if op == Operator::GreatestFixpoint {
                let node = *lhs;
                match node {
                    Node::Variable(string) => {
                        (*variable_map).insert(string.clone(), instance.get_all_states());
                        return calculate_fixpoint(string, *rhs, instance, variable_map)
                    }
                    Node::Action(_) => unreachable!(),
                    Node::UnaryExpr { op: _ } => unreachable!(),
                    Node::BinaryExpr { op: _, lhs: _, rhs: _ } => unreachable!(),
                }
            } else if op == Operator::DiamondModality {
                let node = *lhs;
                match node {
                    Node::Action(string) => {
                        let states_rhs: HashSet<i64> = eval(*rhs, instance, variable_map);
                        return instance.get_diamond_modality(string, states_rhs)
                    }
                    Node::Variable(_) => unreachable!(),
                    Node::UnaryExpr { op: _ } => unreachable!(),
                    Node::BinaryExpr { op: _, lhs: _, rhs: _ } => unreachable!(),
                }
            } else if op == Operator::BoxModality {
                let node = *lhs;
                match node {
                    Node::Action(string) => {
                        let states_rhs: HashSet<i64> = eval(*rhs, instance, variable_map);
                        return instance.get_diamond_modality(string, states_rhs)
                    }
                    Node::Variable(_) => unreachable!(),
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
        Node::Action(_) => {
            unreachable!("Should not happen");
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

fn calculate_fixpoint(string: String, g: Node, instance:&Ltl, variable_map: &mut HashMap<String,HashSet<i64>>) -> HashSet<i64> {
    let mut x_prime: HashSet<i64> = HashSet::new(); 
    let mut a = retrieve_element(variable_map.get(&string.clone())).clone();
    while (x_prime != a){
        x_prime = a.clone();
        let temp = eval(g.clone(), instance, variable_map);
        (*variable_map).insert(string.clone(), temp);
        a = retrieve_element(variable_map.get(&string.clone())).clone();
    }
    return retrieve_element(variable_map.get(&string.clone())).clone();
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