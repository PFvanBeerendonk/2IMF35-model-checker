use array_tool::vec::Union as ATUnion;

// Specify custom type `Ltl`
// follows `https://www.mcrl2.org/web/user_manual/tools/lts.html`

pub struct Ltl {
    pub first_state: i64,
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


    
    // Get S
    pub fn getAllStates() -> StateSet {
        return StateSet{all_states:true,  states:None};
    }

    pub fn henk() -> StateSet {
        if false {
            return StateSet{all_states:false, states:Some(vec![0, 1, 2])}
        } else {
            return StateSet{all_states:true,  states:None}
        }
    }
}

/**
 * Defines a set of states, if `all_states` is true then `states` is None
 * This will save us returning a HUGE vector
 * 
 * #returns 
 * - `bool`: all_states
 * - `states`: `bool` is false -> Vec<i64>
 */
pub struct StateSet {
    pub all_states: bool, 
    pub states: Option<Vec<i64>>
}

pub trait Union {
    fn union(&self, other: Self) -> Self;
}
impl Union for StateSet { 
    fn union(&self, r:StateSet) -> StateSet {
        if self.all_states || r.all_states {
            return StateSet{all_states:true,  states:None} 
        }
        if self.states == None || r.states == None {
            panic!("If all_states is false, states cannot be None")
        }

        let return_value = self.states.clone().unwrap().union(r.states.unwrap());

        return StateSet{all_states:false,  states:Some(return_value)} 
    }

    
}