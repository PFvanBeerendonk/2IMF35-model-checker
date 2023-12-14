use crate::types::ltl::Ltl;
use crate::types::formula::Formula;
use crate::types::formula::Operator;
use crate::types::formula::Node;

use std::collections::HashMap;
use std::collections::HashSet;


// Given a formula `f`, evaluate it over `ltl`
pub fn execute(f: Formula, instance:Ltl) -> (HashSet<i64>, i64) {
    let mut variable_map: HashMap<String,HashSet<i64>> = HashMap::new();
    let mut iterations: i64 = 0;
    (eval(f.root_node, &instance, &mut variable_map, &mut iterations), iterations)
    // let mut set = init(f, instance);
    
}

fn eval(node: Node, instance:&Ltl, variable_map: &mut HashMap<String,HashSet<i64>>, iterations: &mut i64) -> HashSet<i64> {
    match node {
        Node::Variable(string) => {
            // X_i, TODO not sure about array yet.
            if (*variable_map).contains_key(&string.clone()) {
                return variable_map.get(&string.clone()).unwrap().clone();
            } else {
                // TODO: Look at, do we actually want to insert this element? 
                (*variable_map).insert(string.clone(), HashSet::new());
                return variable_map.get(&string.clone()).unwrap().clone();
            }
        }
        Node::BinaryExpr { op, lhs, rhs } => {
            if op == Operator::Conjunction {
                // Not sure if this can be done cleaner tbh, maybe hashset intersection isn't so nice after all...
                let eval_lhs = eval(*lhs, instance, variable_map, iterations);
                let eval_rhs = eval(*rhs, instance, variable_map, iterations);
                return eval_lhs.intersection(&eval_rhs).map(|x| *x).collect::<HashSet<i64>>();
            } else if op == Operator::Disjunction {
                let eval_lhs = eval(*lhs, instance, variable_map, iterations);
                let eval_rhs = eval(*rhs, instance, variable_map, iterations);
                return eval_lhs.union(&eval_rhs).map(|x| *x).collect::<HashSet<i64>>();
            } else if op == Operator::DiamondModality {
                let node = *lhs;
                match node {
                    Node::Action(string) => {
                        let states_rhs: HashSet<i64> = eval(*rhs, instance, variable_map, iterations);
                        return instance.get_diamond_modality(string, states_rhs)
                    }
                    Node::Variable(_) | Node::UnaryExpr { op: _ } | Node::BinaryExpr { op: _, lhs: _, rhs: _ } | 
                    Node::FixPointExpr { op: _, variable: _, rhs: _, surrounding_binder: _ } => unreachable!(),
                }
            } else if op == Operator::BoxModality {
                let node = *lhs;
                match node {
                    Node::Action(string) => {
                        let states_rhs: HashSet<i64> = eval(*rhs, instance, variable_map, iterations);
                        return instance.get_box_modality(string, states_rhs)
                    }
                    Node::Variable(_) | Node::UnaryExpr { op: _ } | Node::BinaryExpr { op: _, lhs: _, rhs: _ } | 
                    Node::FixPointExpr { op: _, variable: _, rhs: _, surrounding_binder: _ } => unreachable!(),
                }
            } else {
                panic!("We shouldn't reach this!");
            }
        }
        Node::UnaryExpr { op } => {
            if op == Operator::SimpleTrue {
                let all_states = instance.get_all_states(); // TODO
                return all_states;
            } else if op == Operator::SimpleFalse {
                // Empty set.
                return HashSet::new();
            } else {
                panic!("This should not reach any statement except true or false");
            }
        }
        Node::FixPointExpr { op, variable, rhs, surrounding_binder: _ } => {
            if op == Operator::GreatestFixpoint {
                (*variable_map).insert(variable.clone(), instance.get_all_states());
                return calculate_fixpoint(variable, *rhs, instance, variable_map, iterations)
            } else if op == Operator::LeastFixpoint {
                (*variable_map).insert(variable.clone(), HashSet::new());
                return calculate_fixpoint(variable, *rhs, instance, variable_map, iterations)
            } else {
                panic!("This should not happen");
            }
        }
        Node::Action(_) => {
            unreachable!("Should not happen");
        }
    }
}

fn calculate_fixpoint(string: String, g: Node, instance:&Ltl, variable_map: &mut HashMap<String,HashSet<i64>>, iterations: &mut i64) -> HashSet<i64> {
    // Set this to something that is both not the full set and the empty set, to make sure we do not quit immediately:
    let mut x_prime: HashSet<i64> = HashSet::from([1]); 
    let mut a = variable_map.get(&string.clone()).unwrap().clone();
    while x_prime != a {
        x_prime = a.clone();
        let temp = eval(g.clone(), instance, variable_map, iterations);
        (*variable_map).insert(string.clone(), temp);
        a = variable_map.get(&string.clone()).unwrap().clone();
        *iterations += 1;
    }
    return variable_map.get(&string.clone()).unwrap().clone();
}

pub fn execute_improved(f: Formula, instance:Ltl) -> (HashSet<i64>, i64) {
    let (variables_open_map, variables_nu, variables_mu) = find_open_variables(&f.root_node);
    let mut variables_map = HashMap::new();
    initialize_variable_map(&instance, &mut variables_map, &variables_nu, &variables_mu);
    let mut iterations = 0;
    (eval_improved(f.root_node, &instance, &mut variables_map, &variables_open_map, &mut iterations), iterations)
}

fn initialize_variable_map(instance:&Ltl, variable_map: &mut HashMap<String,HashSet<i64>>, variables_nu: &HashSet<String>, variables_mu: &HashSet<String>) {
    for var in variables_nu {
        (*variable_map).insert(var.clone(), instance.get_all_states());
    }
    for var in variables_mu {
        (*variable_map).insert(var.clone(), HashSet::new());
    }
}

fn eval_improved(node: Node, instance:&Ltl, variable_map: &mut HashMap<String,HashSet<i64>>, variables_open_map: &HashMap<String
    , HashSet<String>>, iterations: &mut i64) -> HashSet<i64> {
    match node {
        Node::Variable(string) => {
            // X_i, TODO not sure about array yet.
            if (*variable_map).contains_key(&string.clone()) {
                return variable_map.get(&string.clone()).unwrap().clone();
            } else {
                // TODO: Look at, do we actually want to insert this element? 
                (*variable_map).insert(string.clone(), HashSet::new());
                return variable_map.get(&string.clone()).unwrap().clone();
            }
        }
        Node::BinaryExpr { op, lhs, rhs } => {
            if op == Operator::Conjunction {
                // Not sure if this can be done cleaner tbh, maybe hashset intersection isn't so nice after all...
                let eval_lhs = eval_improved(*lhs, instance, variable_map, variables_open_map, iterations);
                let eval_rhs = eval_improved(*rhs, instance, variable_map, variables_open_map, iterations);
                return eval_lhs.intersection(&eval_rhs).map(|x| *x).collect::<HashSet<i64>>();
            } else if op == Operator::Disjunction {
                let eval_lhs = eval_improved(*lhs, instance, variable_map, variables_open_map, iterations);
                let eval_rhs = eval_improved(*rhs, instance, variable_map, variables_open_map, iterations);
                return eval_lhs.union(&eval_rhs).map(|x| *x).collect::<HashSet<i64>>();
            } else if op == Operator::DiamondModality {
                let node = *lhs;
                match node {
                    Node::Action(string) => {
                        let states_rhs: HashSet<i64> = eval_improved(*rhs, instance, variable_map, variables_open_map, iterations);
                        return instance.get_diamond_modality(string, states_rhs)
                    }
                    Node::Variable(_) | Node::UnaryExpr { op: _ } | Node::BinaryExpr { op: _, lhs: _, rhs: _ } | 
                    Node::FixPointExpr { op: _, variable: _, rhs: _, surrounding_binder: _ } => unreachable!(),
                }
            } else if op == Operator::BoxModality {
                let node = *lhs;
                match node {
                    Node::Action(string) => {
                        let states_rhs: HashSet<i64> = eval_improved(*rhs, instance, variable_map, variables_open_map, iterations);
                        return instance.get_box_modality(string, states_rhs)
                    }
                    Node::Variable(_) | Node::UnaryExpr { op: _ } | Node::BinaryExpr { op: _, lhs: _, rhs: _ } | 
                    Node::FixPointExpr { op: _, variable: _, rhs: _, surrounding_binder: _ } => unreachable!(),
                }
            } else {
                panic!("We shouldn't reach this statement!");
            }
        }
        Node::UnaryExpr { op } => {
            if op == Operator::SimpleTrue {
                let all_states = instance.get_all_states(); // TODO
                return all_states;
            } else if op == Operator::SimpleFalse {
                // Empty set.
                return HashSet::new();
            } else {
                panic!("This should not reach any statement except true or false");
            }
        }
        Node::FixPointExpr { op, variable, rhs, surrounding_binder } => {
            if op == Operator::GreatestFixpoint {
                if surrounding_binder == Operator::LeastFixpoint {
                    if let Some(value) = variables_open_map.get(&variable) {
                        for var in value {
                            (*variable_map).insert(var.clone(), instance.get_all_states());
                        }
                    }
                }
                return calculate_fixpoint_improved(variable, *rhs, instance, variable_map, variables_open_map, iterations)
            } else if op == Operator::LeastFixpoint {
                if surrounding_binder == Operator::GreatestFixpoint {
                    if let Some(value) = variables_open_map.get(&variable) {
                        for var in value {
                            (*variable_map).insert(var.clone(), HashSet::new());
                        }
                    }
                }
                (*variable_map).insert(variable.clone(), HashSet::new());
                return calculate_fixpoint_improved(variable, *rhs, instance, variable_map, variables_open_map, iterations)
            } else {
                panic!("This should not happen");
            }
        }
        Node::Action(_) => {
            unreachable!("Should not happen");
        }
    }
}

fn find_open_variables(node: &Node) -> (HashMap<String, HashSet<String>>, HashSet<String>, HashSet<String>) {
    let mut variables_mu: HashSet<String> = HashSet::new();
    let mut variables_nu: HashSet<String> = HashSet::new();
    let mut variables_sub_found_map: HashMap<String,HashSet<String>> = HashMap::new();
    let mut variables_sub_map: HashMap<String,HashSet<String>> = HashMap::new();
    let mut variables_open_set: HashSet<String> = HashSet::new();
    let mut variables_open_map: HashMap<String,HashSet<String>> = HashMap::new();
    find_variables(node, &mut variables_mu, &mut variables_nu, &mut variables_sub_found_map, &mut variables_sub_map);
    for (key, value) in variables_sub_found_map {
        let temp_set = variables_sub_map.get(&key).unwrap().clone();
        if (value).difference(&temp_set).map(|x| x.to_string()).collect::<HashSet<String>>().len() > 0 {
            variables_open_set.insert(key);
        }
    }
    for (key, value) in variables_sub_map {
        if variables_mu.contains(&key) {
            let temp = (value).difference(&variables_nu).map(|x| x.to_string()).collect::<HashSet<String>>(); 
            variables_open_map.insert(key, (temp).intersection(&variables_open_set).map(|x| x.to_string()).collect::<HashSet<String>>());
        } else if variables_nu.contains(&key) {
            let temp = (value).difference(&variables_mu).map(|x| x.to_string()).collect::<HashSet<String>>();
            variables_open_map.insert(key, (temp).intersection(&variables_open_set).map(|x| x.to_string()).collect::<HashSet<String>>());
        }
    }
    (variables_open_map, variables_nu, variables_mu)
}

fn find_variables(node: &Node, variables_mu: &mut HashSet<String>, variables_nu: &mut HashSet<String>, 
    variables_sub_found_map: &mut HashMap<String,HashSet<String>>, variables_sub_map: &mut HashMap<String,HashSet<String>>) {
    match node {
        Node::Variable(var) => {
            for (_, var_set) in variables_sub_found_map {
                (*var_set).insert(var.clone());
            }
        }
        Node::BinaryExpr { lhs, rhs, .. } => {
            find_variables(lhs, variables_mu, variables_nu, variables_sub_found_map, variables_sub_map);
            find_variables(rhs, variables_mu, variables_nu, variables_sub_found_map, variables_sub_map);
        }
        Node::FixPointExpr { op, variable, rhs , surrounding_binder: _} => {
            if *op == Operator::GreatestFixpoint {
                variables_nu.insert(variable.clone());
            } else if *op == Operator::LeastFixpoint {
                variables_mu.insert(variable.clone());
            }
            (*variables_sub_found_map).insert(variable.clone(), HashSet::new());
            (*variables_sub_map).insert(variable.clone(), HashSet::new());
            for (_, var_set) in &mut *variables_sub_map {
                (*var_set).insert(variable.clone());
            }
            find_variables(rhs, variables_mu, variables_nu, variables_sub_found_map, variables_sub_map);
        }
        Node::UnaryExpr { .. } | Node::Action(_) => {}
    }
}

fn calculate_fixpoint_improved(string: String, g: Node, instance:&Ltl, variable_map: &mut HashMap<String,HashSet<i64>>, 
    variables_open_map: &HashMap<String, HashSet<String>>, iterations: &mut i64) -> HashSet<i64> {
    // Set this to something that is both not the full set and the empty set, to make sure we do not quit immediately:
    let mut x_prime: HashSet<i64> = HashSet::from([1]); 
    let mut a = variable_map.get(&string.clone()).unwrap().clone();
    while x_prime != a {
        x_prime = a.clone();
        let temp = eval_improved(g.clone(), instance, variable_map, variables_open_map, iterations);
        (*variable_map).insert(string.clone(), temp);
        a = variable_map.get(&string.clone()).unwrap().clone();
        *iterations += 1;
    }
    return variable_map.get(&string.clone()).unwrap().clone();
}


