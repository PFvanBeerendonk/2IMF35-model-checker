// Specify custom type `Ltl`

pub struct Ltl {
    pub temp: i64, // NOTE: temp is temprary
}

/* NOTE: The ltl datatype should be able to do the following things efficiently:
 * - build from a .aut file
 * 
 * - S:             Return all states
 * - [[ p ]]:       for some label p, return all states where p holds
 * - [[ [a]f ]]:    Get all states that have an a-transition into a state in set F
 *  */ 

impl Ltl{
    /**
     * Initial initial node, and all nodes
     * 'des (' first_state ',' nr_of_transitions ',' nr_of_states ')'
     */
    pub fn new(first_state: i64, nr_of_transitions: i64, nr_of_states: i64) -> Self{
        println!("uwuwuuw {} {} {}",first_state, nr_of_transitions, nr_of_states);
        
        return Self{
            temp: 12,
        }
    }

    
}



struct State {
    number: i64, //Number given to the state
}




// TODO: define further using `impl`
