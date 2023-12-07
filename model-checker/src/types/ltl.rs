use std::collections::HashSet;
use std::collections::HashMap;

// Specify custom type `Ltl`
// follows `https://www.mcrl2.org/web/user_manual/tools/lts.html`

pub struct Ltl {
    pub first_state: i64,
    pub transitions: HashMap< i64, HashMap< String, Vec<i64> > >, 
    // state -> state_map where state_map: label -> [state]

    pub nr_of_states: i64,
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
        let mut transition_dict: HashMap<i64, HashMap< String, Vec<i64>>> = HashMap::new();
        transition_dict.reserve(nr_of_transitions as usize);

        return Self{
            first_state: first_state,
            transitions: transition_dict, 
            nr_of_states: nr_of_states,
        }
    }

    /**
     * add an edge
     */
    pub fn add_transition(&mut self, start_state: i64, label: &str, end_state: i64) -> &mut Self {
        if start_state < 0 || start_state > self.nr_of_states {
            panic!("start_state '{}' not correct", start_state)
        }
        if end_state < 0 || end_state > self.nr_of_states {
            panic!("end_state '{}' not correct", end_state)
        }

        println!("adding line ({},{},{})", start_state, label, end_state);
        
        let mut transition_dict = self.transitions.clone();
        if transition_dict.contains_key(&start_state) {
            // if start_state already in transition dict, add the given transition to its hashmap
            let mut state_map: HashMap< String, Vec<i64>> = transition_dict.get(&start_state)
                .expect("Won't happen, see contains_key() above").clone();
            if state_map.contains_key(&String::from(label)) {
                // State map already contains this transition, 
                let mut end_states: Vec<i64> = state_map.get(&String::from(label))
                .expect("Won't happen, see contains_key() above").clone();
                if !end_states.contains(&end_state) {
                    // this state is not yet in end_states, add it to the vector
                    end_states.push(end_state);
                }
                state_map.insert(String::from(label), end_states);
                
            } else {
                state_map.insert(String::from(label), vec![end_state]);
            }
            transition_dict.insert(start_state, state_map);

        } else {
            // else initialize the transition ==> insert (start_state, HashMap({label, [end_state]}))
            let mut state_map: HashMap< String, Vec<i64>> = HashMap::new();
            state_map.insert(String::from(label), vec![end_state]);
            transition_dict.insert(start_state, state_map);
        }

        self.transitions = transition_dict;
        return self
    }
    
    /**
     * Finish contruction, set data to immutable
     */
    pub fn finish_contruction(self) -> Self {
        // TODO: lock the data
        return self
    }

    /**
     * Get S, all states
     */
    pub fn get_all_states(self: Self) -> HashSet<i64> {
        // Create vector with numbers 0 through nr_of_states
        let states: Vec<i64> = (0..self.nr_of_states).collect();
        // Turn into hashset
        return HashSet::from_iter(states.iter().cloned());
    }

    /**
     * Get [[ [a]f ]] (BoxModality),
     *   Get all states that have all a-transition into a state in set F
     */
    pub fn get_box_modality(self: Self) -> HashSet<i64> {
        panic!("not implemented yet")
    }

    /**
     * Get [[ <a>f ]] (DiamondModality),
     *   Get all states that have some a-transition into a state in set F
     */
    pub fn get_diamond_modality(self: Self) -> HashSet<i64> {
        panic!("not implemented yet")
    }

}
