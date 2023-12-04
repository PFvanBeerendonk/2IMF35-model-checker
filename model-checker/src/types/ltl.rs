// Specify custom type `Ltl`
// follows `https://www.mcrl2.org/web/user_manual/tools/lts.html`

pub struct Ltl {
    pub first_state: i64, // NOTE: temp is temprary
    nr_of_transitions: i64, 
    nr_of_states: i64
}

/* NOTE: The ltl datatype should be able to do the following things efficiently:
 * - build from a .aut file
 * 
 * - S:             Return all states
 * - [[ [a]f ]]:    Get all states that have all a-transition into a state in set F
 * - [[ <a>f ]]:    Get all states that have some a-transition into a state in set F
 *  */ 

impl Ltl{
    /**
     * Initialize initial node, and all nodes
     * 
     */
    pub fn new(first_state: i64, nr_of_transitions: i64, nr_of_states: i64) -> Self{
        println!("uwuwuuw {} {} {}", first_state, nr_of_transitions, nr_of_states);
        
        return Self{
            first_state: first_state,
            nr_of_transitions: nr_of_transitions, 
            nr_of_states: nr_of_states
        }
    }

    /**
     * add an edge
     */
    pub fn add_transition(self, start_state: i64, label: &str, end_state: i64) -> Self {
        if start_state < 0 || start_state > self.nr_of_states {
            panic!("start_state '{}' not correct", start_state)
        }
        if end_state < 0 || end_state > self.nr_of_states {
            panic!("end_state '{}' not correct", end_state)
        }

        println!("TODO: add line ({},{},{})", start_state, label, end_state);

        return self
    }


    /**
     * EXAMPLE of how to return all states, will save us returning a HUGE vector
     * 
     * #returns 
     * - `bool`: allStates
     * - `states`: `bool` is false -> Vec<i64>
     */
    pub fn getAllStates() -> (bool, Option<Vec<i64>>) {
        if false {
            return (true, Some(vec![0, 1, 2]))
        } else {
            return (false, None)
        }
    }
}



struct State {
    number: i64, //Number given to the state
}




// TODO: define further using `impl`
