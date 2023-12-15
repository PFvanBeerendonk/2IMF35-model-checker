use crate::types::ltl::Ltl;
use crate::types::formula::Formula;
use crate::types::formula::Operator;
use crate::types::formula::Node;

use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;


/**
 * Given a Formula f and an LTL instance, evaluate f on the instance.
 * Uses the eval functions and also returns the number of iterations.
 */
pub fn execute(f: Formula, instance:Ltl) -> (HashSet<i64>, i64) {
    // Make a new map and iterations variable and call the eval function.
    let mut variable_map: HashMap<String,HashSet<i64>> = HashMap::new();
    let mut iterations: i64 = 0;
    (eval(f.root_node, &instance, &mut variable_map, &mut iterations), iterations)    
}

/**
 * Given a Node node and an LTL instance evaluate the set of stats in instance satisfing satisfying the formula represented by node.
 * also requires variable_map, found fixed point variabels and their associated value and iterations, 
 * to keep track of the number of fixed point iterations.
 */
fn eval(node: Node, instance:&Ltl, variable_map: &mut HashMap<String,HashSet<i64>>, iterations: &mut i64) -> HashSet<i64> {
    match node {
        Node::Variable(string) => {
            // The first case, we need to return the value associated with the variable string in the variable_map.
            // If this value exists, return the associated value:
            if (*variable_map).contains_key(&string.clone()) {
                return variable_map.get(&string.clone()).unwrap().clone();
            } else {
                // Otherwise we need to insert an empty set into the map, and return this empty set.
                (*variable_map).insert(string.clone(), HashSet::new());
                return variable_map.get(&string.clone()).unwrap().clone();
            }
        }
        Node::BinaryExpr { op, lhs, rhs } => {
            // We have a binary expression, that is an expression over 2 variables (conjunction/disjunction/boxmodality/diamondmodality):
            if op == Operator::Conjunction {
                // We have a conjunction, we return the intersection of the evaluation of the left and right hand side:
                let eval_lhs = eval(*lhs, instance, variable_map, iterations);
                let eval_rhs = eval(*rhs, instance, variable_map, iterations);
                return eval_lhs.intersection(&eval_rhs).map(|x| *x).collect::<HashSet<i64>>();
            } else if op == Operator::Disjunction {
                // We have a disjunction, we return the union of the evaluation of the left and right hand side:
                let eval_lhs = eval(*lhs, instance, variable_map, iterations);
                let eval_rhs = eval(*rhs, instance, variable_map, iterations);
                return eval_lhs.union(&eval_rhs).map(|x| *x).collect::<HashSet<i64>>();
            } else if op == Operator::DiamondModality {
                // We have a diamondmodality, we get the action label and call the get_diamond_modality function 
                // on the LTL instance to get the diamond modality.
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
                // We have a boxmodality, we get the action label and call the get_box_modality function 
                // on the LTL instance to get the diamond modality.
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
            // We have a unary expression, that is either a simple true or a simple false statement.
            if op == Operator::SimpleTrue {
                // If we have a simple true, we simple return all states from instance using instance.get_all_states().
                let all_states = instance.get_all_states();
                return all_states;
            } else if op == Operator::SimpleFalse {
                // We have a simple false, we simply return an empty set.
                return HashSet::new();
            } else {
                panic!("This should not reach any statement except true or false");
            }
        }
        Node::FixPointExpr { op, variable, rhs, surrounding_binder: _ } => {
            // We have a fixed point expression, we first check wheter we have a least fixed point or greatest fixed point.
            // Then in the case of a greatest fixed point, we set variable_map[operator] to the set of all states in the instance.
            // In the case of a least fixed point, we set variable_map[operator] to the empty set. Then we use calculate_fixpoint
            // to calculate the fixed point.
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

/**
 * Given a variable string, a Node g, an LTL instance, a variable_map and an iterations variable,
 * calculate the fixed point of g for variable string and return the resulting set.
 */
fn calculate_fixpoint(variable: String, g: Node, instance:&Ltl, variable_map: &mut HashMap<String,HashSet<i64>>, iterations: &mut i64) -> HashSet<i64> {
    // Set x' to something that is both not the full set and the empty set, to make sure we do not quit immediately:
    let mut x_prime: HashSet<i64> = HashSet::from([1]); 
    // Retrieve the a value from the map:
    let mut a = variable_map.get(&variable.clone()).unwrap().clone();
    // Until x' is equal to a, we keep calculating the fixed point using the eval function and insert the result into the map.
    while x_prime != a {
        x_prime = a.clone();
        let temp = eval(g.clone(), instance, variable_map, iterations);
        (*variable_map).insert(variable.clone(), temp);
        a = variable_map.get(&variable.clone()).unwrap().clone();
        // Each iteration of the while loop, we increment iterations by 1:
        *iterations += 1;
    }
    // Return the value at variable_map[variable].
    return variable_map.get(&variable.clone()).unwrap().clone();
}

/**
 * Given a Formula f and an LTL instance, evaluate f on the instance.
 * Uses the eval_improved functions using the Emerson_Lei algorithm and also returns the number of iterations.
 */
pub fn execute_improved(f: Formula, instance:Ltl) -> (HashSet<i64>, i64) {
    // First we find the open variables, and initialize the variable map as required for the given mu en nu variables:
    let (variables_open_map, variables_nu, variables_mu) = find_open_variables(&f.root_node);
    let mut variables_map = HashMap::new();
    initialize_variable_map(&instance, &mut variables_map, &variables_nu, &variables_mu);
    let mut iterations = 0;
    // Then we call the eval_improved function.
    (eval_improved(f.root_node, &instance, &mut variables_map, &variables_open_map, &mut iterations), iterations)
}

/**
 * Initialize the map variables_map setting the values empty set for the mu variabels and the set of all states from the Ltl instance for the nu variables.
 */
fn initialize_variable_map(instance:&Ltl, variable_map: &mut HashMap<String,HashSet<i64>>, variables_nu: &HashSet<String>, variables_mu: &HashSet<String>) {
    // If we have a nu variable, insert all states from instance into the map.
    for var in variables_nu {
        (*variable_map).insert(var.clone(), instance.get_all_states());
    }
    // If we have a mu variable, insert an empty set into the map.
    for var in variables_mu {
        (*variable_map).insert(var.clone(), HashSet::new());
    }
}

/**
 * Given a Node node and an LTL instance evaluate the set of stats in instance satisfing satisfying the formula represented by node.
 * also requires variable_map, found fixed point variabels and their associated value, iterations, to keep track of the number of fixed 
 * point iterations and variables_open_map to find for each variable, their open subvariables of the same fixpoint. So for a mu variable
 * only containing mu variables, and for a nu variable only containing nu variables.
 */
fn eval_improved(node: Node, instance:&Ltl, variable_map: &mut HashMap<String,HashSet<i64>>, variables_open_map: &HashMap<String
    , HashSet<String>>, iterations: &mut i64) -> HashSet<i64> {
    match node {
        Node::Variable(string) => {
            // The first case, we need to return the value associated with the variable string in the variable_map.
            // If this value exists, return the associated value:
            if (*variable_map).contains_key(&string.clone()) {
                return variable_map.get(&string.clone()).unwrap().clone();
            } else {
                // Otherwise we need to insert an empty set into the map, and return this empty set.
                (*variable_map).insert(string.clone(), HashSet::new());
                return variable_map.get(&string.clone()).unwrap().clone();
            }
        }
        Node::BinaryExpr { op, lhs, rhs } => {
             // We have a binary expression, that is an expression over 2 variables (conjunction/disjunction/boxmodality/diamondmodality):
            if op == Operator::Conjunction {
                // We have a conjunction, we return the intersection of the evaluation of the left and right hand side:
                let eval_lhs = eval_improved(*lhs, instance, variable_map, variables_open_map, iterations);
                let eval_rhs = eval_improved(*rhs, instance, variable_map, variables_open_map, iterations);
                return eval_lhs.intersection(&eval_rhs).map(|x| *x).collect::<HashSet<i64>>();
            } else if op == Operator::Disjunction {
                // We have a disjunction, we return the union of the evaluation of the left and right hand side:
                let eval_lhs = eval_improved(*lhs, instance, variable_map, variables_open_map, iterations);
                let eval_rhs = eval_improved(*rhs, instance, variable_map, variables_open_map, iterations);
                return eval_lhs.union(&eval_rhs).map(|x| *x).collect::<HashSet<i64>>();
            } else if op == Operator::DiamondModality {
                // We have a diamondmodality, we get the action label and call the get_diamond_modality function 
                // on the LTL instance to get the diamond modality.
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
                // We have a boxmodality, we get the action label and call the get_box_modality function 
                // on the LTL instance to get the diamond modality.
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
            // We have a unary expression, that is either a simple true or a simple false statement.
            if op == Operator::SimpleTrue {
                // If we have a simple true, we simple return all states from instance using instance.get_all_states().
                let all_states = instance.get_all_states(); 
                return all_states;
            } else if op == Operator::SimpleFalse {
                // We have a simple false, we simply return an empty set.
                return HashSet::new();
            } else {
                panic!("This should not reach any statement except true or false");
            }
        }
        Node::FixPointExpr { op, variable, rhs, surrounding_binder } => {
            // We have a fixed point expression, we first check wheter we have a least fixed point or greatest fixed point.
            // Then in the case of a greatest fixed point, and the surrounding binder of the current fixed point is a least
            // fixed point, we reset variable_map for all variables in the variables_open_map[variable]
            // to the set of all states in the instance.
            if op == Operator::GreatestFixpoint {
                if surrounding_binder == Operator::LeastFixpoint {
                    if let Some(value) = variables_open_map.get(&variable) {
                        for var in value {
                            (*variable_map).insert(var.clone(), instance.get_all_states());
                        }
                    }
                }
            } else if op == Operator::LeastFixpoint {
            // in the case of a least fixed point, and the surrounding binder of the current fixed point is a greatest
            // fixed point, we reset variable_map for all variables in the variables_open_map[variable]
            // to the empty set.
                if surrounding_binder == Operator::GreatestFixpoint {
                    if let Some(value) = variables_open_map.get(&variable) {
                        for var in value {
                            (*variable_map).insert(var.clone(), HashSet::new());
                        }
                    }
                }
                (*variable_map).insert(variable.clone(), HashSet::new());
                
            } else {
                panic!("This should not happen");
            }
            // We call the calculate_fixpoint_improved function to calculate the fixed point.
            return calculate_fixpoint_improved(variable, *rhs, instance, variable_map, variables_open_map, iterations)
        }
        Node::Action(_) => {
            unreachable!("Should not happen");
        }
    }
}

/**
 * Given a variable string, a Node g, an LTL instance, a variable_map, variables_open_map and an iterations variable,
 * calculate the fixed point of g for variable string and return the resulting set.
 */
fn calculate_fixpoint_improved(string: String, g: Node, instance:&Ltl, variable_map: &mut HashMap<String,HashSet<i64>>, 
    variables_open_map: &HashMap<String, HashSet<String>>, iterations: &mut i64) -> HashSet<i64> {
    // Set x' to something that is both not the full set and the empty set, to make sure we do not quit immediately:
    let mut x_prime: HashSet<i64> = HashSet::from([1]); 
    // Retrieve the a value from the map:
    let mut a = variable_map.get(&string.clone()).unwrap().clone();
    // Until x' is equal to a, we keep calculating the fixed point using the eval function and insert the result into the map.
    while x_prime != a {
        x_prime = a.clone();
        let temp = eval_improved(g.clone(), instance, variable_map, variables_open_map, iterations);
        (*variable_map).insert(string.clone(), temp);
        a = variable_map.get(&string.clone()).unwrap().clone();
        // Each iteration of the while loop, we increment iterations by 1:
        *iterations += 1;
    }
    // Return the value at variable_map[variable].
    return variable_map.get(&string.clone()).unwrap().clone();
}

/**
 * Given a Node node, find all open variables, and return a map from each variable to the set of open variables in its subformula 
 * of the same fixed point. That is, for a greatest fixed point all open subvariables should be open variables for a greatest 
 * fixed point, and for a least fixed point all open subvariables should be open variables for a least fixed point.
 * Also return the set of nu variables and the set of mu variables.
 */
fn find_open_variables(node: &Node) -> (HashMap<String, HashSet<String>>, HashSet<String>, HashSet<String>) {
    // Initialize some sets and maps:
    let mut variables_mu: HashSet<String> = HashSet::new();
    let mut variables_nu: HashSet<String> = HashSet::new();
    let mut variables_sub_found_map: HashMap<String,HashSet<String>> = HashMap::new();
    let mut variables_sub_map: HashMap<String,HashSet<String>> = HashMap::new();
    let mut variables_open_set: HashSet<String> = HashSet::new();
    let mut variables_open_map: HashMap<String,HashSet<String>> = HashMap::new();
    let mut variables_visited: HashSet<String> = HashSet::new();
    // Call the find_variables function to find all greatset fixed point and least fixed point variables and find all variables visited up to a variable (variables_sub_map)
    // and all variables found up to a variable (variables_sub_found_map).
    find_variables(node, &mut variables_mu, &mut variables_nu, &mut variables_sub_found_map, &mut variables_sub_map, &mut variables_visited);
    // We create the set of open variables for each node by taking the difference of variables_sub_found_map, that is all found variables and all visited variables (variables_sub_map).
    for (key, value) in variables_sub_found_map {
        let temp_set = variables_sub_map.get(&key).unwrap().clone();
        if (value).difference(&temp_set).map(|x| x.to_string()).collect::<HashSet<String>>().len() > 0 {
            variables_open_set.insert(key);
        }
    }
    // Remove all variables of the other sort (greatest/least fixed point) and insert these to variables_open_map.
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


/**
 * Find variables for a given formula, insert least fixed point variables into the variables_mu set, insert greateste fixed point variables into the variables_nu set.
 * Furthermore, insert variables found as variable with the least/greatest fixed point operator in variables_sub_map and variables found as separate variable
 * up to a variable in variables_sub_found_map. We use variables_visited to keep track of all variables visited up to a variable 
 * (so we can separate variables between two or or and operators).
*/
fn find_variables(node: &Node, variables_mu: &mut HashSet<String>, variables_nu: &mut HashSet<String>, 
    variables_sub_found_map: &mut HashMap<String,HashSet<String>>, variables_sub_map: &mut HashMap<String,HashSet<String>>, variables_visited: &mut HashSet<String>) {
    match node {
        Node::Variable(var) => {
            // If we find a variable, add these to all variables_visisted for the variables_subs_found_map:
            for variable in & *variables_visited {
                let var_set = variables_sub_found_map.get_mut(variable).unwrap();
                (*var_set).insert(var.clone());
            }
        }
        Node::BinaryExpr { lhs, rhs, .. } => {
            // Recursively call the function on both sides with the variables_visisted cloned so we have no overlap here:
            find_variables(lhs, variables_mu, variables_nu, variables_sub_found_map, variables_sub_map, &mut variables_visited.clone());
            find_variables(rhs, variables_mu, variables_nu, variables_sub_found_map, variables_sub_map, &mut variables_visited.clone());
        }
        Node::FixPointExpr { op, variable, rhs , surrounding_binder: _} => {
            // Add variables to variables_nu or variables_mu as required:
            if *op == Operator::GreatestFixpoint {
                variables_nu.insert(variable.clone());
            } else if *op == Operator::LeastFixpoint {
                variables_mu.insert(variable.clone());
            }
            // Insert the current variable to variables_visited and insert an empty set to variables_sub_found_map and variables_sub_map:
            (*variables_visited).insert(variable.clone());
            (*variables_sub_found_map).insert(variable.clone(), HashSet::new());
            (*variables_sub_map).insert(variable.clone(), HashSet::new());
            // Add the variables to variables_sub_map:
            for var in & *variables_visited {
                let var_set = variables_sub_map.get_mut(var).unwrap();
                (*var_set).insert(variable.clone());
            }
            // Recursively call the function on the rhs of the fixed point expression:
            find_variables(rhs, variables_mu, variables_nu, variables_sub_found_map, variables_sub_map, variables_visited);
        }
        Node::UnaryExpr { .. } | Node::Action(_) => {}
    }
}

/**
 * Method to find the nesting depth, alteration depth and dependent alteration depth of a formula given by Node node. Uses nesting_depth, alteration_depth and dependent_alteration_depth
 * to keep track of the current values. And returns a tupple with these 3 values. Variables_map is used to keep track of the variables visited from that variable.
 */
fn find_formula_depths(node: &Node, nesting_depth: i64, alteration_depth: i64, dependent_alteration_depth: i64, variables_map: &mut HashMap<String, HashSet<String>>) -> (i64, i64, i64) {
    match node {
        Node::Variable(_) => {
            // For a variable the value is equal to the values when we entered this node.
            (nesting_depth, alteration_depth, dependent_alteration_depth)
        }
        Node::BinaryExpr { op: _, lhs, rhs } => {
            // In the case of a binary expression the value of the nesting depth, alteration depth and dependent alteration depth is equal to the maximum of the values of either side.
            let (nesting_depth_lhs, alteration_depth_lhs, dependent_alteration_depth_lhs) = find_formula_depths(lhs, nesting_depth, alteration_depth, dependent_alteration_depth, variables_map);
            let (nesting_depth_rhs, alteration_depth_rhs, dependent_alteration_depth_rhs) = find_formula_depths(rhs, nesting_depth, alteration_depth, dependent_alteration_depth, variables_map);
            let nesting_depth = max(nesting_depth_lhs, nesting_depth_rhs);
            let alteration_depth = max(alteration_depth_lhs, alteration_depth_rhs);
            let dependent_alteration_depth = max(dependent_alteration_depth_lhs, dependent_alteration_depth_rhs);
            (nesting_depth, alteration_depth, dependent_alteration_depth)
        }
        Node::FixPointExpr { op, variable, rhs, surrounding_binder } => {
            // For a fixed point expression, we first need to find the nesting depth, alteration depth and dependent alteration depths of the right hand side.
            let (nesting_depth_rhs, alteration_depth_rhs, dependent_alteration_depth_rhs) = find_formula_depths(rhs, nesting_depth, alteration_depth, dependent_alteration_depth, variables_map);
            // If the surrounding binder is the same as the current operator, we simply increment the nesting depth by 1.
            if surrounding_binder == op {
                (nesting_depth_rhs + 1, alteration_depth_rhs, dependent_alteration_depth_rhs)
            } else {
                // If the surrounding binder is not the same as the current operator, we need to check whether the variable is in the variables_map.
                // If the variable is in the variables map we increment all 3 of nesting depth, alteration depth and dependent alteration depth.
                if ((*variables_map).get(&variable.clone()).unwrap()).contains(&variable.clone()) {
                    (nesting_depth_rhs + 1, alteration_depth_rhs + 1, dependent_alteration_depth_rhs + 1)
                } else {
                    // Otherwise only increment the nesting depth and alteration depth.
                    (nesting_depth_rhs + 1, alteration_depth_rhs + 1, dependent_alteration_depth_rhs)
                }
            } 
        }
        Node::UnaryExpr { op: _ } => {
            // For a unary expression the value is equal to the values when we entered this node.
            (nesting_depth, alteration_depth, dependent_alteration_depth)
        }
        Node::Action(_) => {
            // For an action the value is equal to the values when we entered this node.
            (nesting_depth, alteration_depth, dependent_alteration_depth)
        }
    }
}

/**
 * Method to find all variables we visit from a given node. Uses variables_map to keep track of the variables visited from that variable.
 * Uses visited_variables to keep track of the variables visited up to a variable (so we can deal with binary expressions).
 */
fn find_visited_variables(node: &Node, variables_map: &mut HashMap<String, HashSet<String>>, visited_variables: &mut HashSet<String>) {
    match node {
        Node::Variable(variable) => {
            // For a variable, we add this to all values in the visisted_variables set.
            for var in & *visited_variables {
                let var_set = variables_map.get_mut(var).unwrap();
                (*var_set).insert(variable.clone());
            }
        }
        Node::BinaryExpr { op: _, lhs, rhs } => {
            // For a binary expression, we recursively call the function on both sides with the visited_variables cloned so we have no overlap here.
            find_visited_variables(lhs, variables_map, &mut visited_variables.clone());
            find_visited_variables(rhs, variables_map, &mut visited_variables.clone());
        }
        Node::FixPointExpr { op: _, variable, rhs, surrounding_binder: _ } => {
            // For a fixed point expression, we recursively call the function for the rhs, insert an empty hashset for the current value 
            // and add the variable to the visited_variables set.
            (*variables_map).insert(variable.clone(), HashSet::new());
            (*visited_variables).insert(variable.clone());
            find_visited_variables(rhs, variables_map, visited_variables);
        }
        // For the unary expression or action we do nothing.
        Node::UnaryExpr { op: _} |  Node::Action(_) => {
        }
    }
}

/**
 * Method to find the nesting depth, alteration depth and dependent alteration depth of a formula given by Node node. 
 * Returns these 3 values in a tupple.
 */
pub fn find_formula_statistics(node: &Node) -> (i64, i64, i64) {
    // initialize some variables and call the find_visited_variables and find_formula_depths functions.
    let mut variables_map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut visited_variables: HashSet<String> = HashSet::new();
    find_visited_variables(node, &mut variables_map, &mut visited_variables);
    find_formula_depths(node, 0, 0, 0, &mut variables_map)
}