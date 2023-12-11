use std::collections::HashSet;
use std::collections::HashMap;

// Specify custom type `Ltl`
// follows `https://www.mcrl2.org/web/user_manual/tools/lts.html`

pub struct Ltl {
    pub first_state: i64,
    pub transitions: HashMap< i64, HashMap< String, HashSet<i64> > >, 
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
        // Build a hashMap with all states prefilled, pointing to empty (sub-)hashMaps
        let transition_dict: HashMap<i64, HashMap< String, HashSet<i64>>> = (0..nr_of_states) // range 0,1,2,...,nr_of_states
        .map(|chunk| (chunk, HashMap::new())) // map them to tuples of the right type
        .collect::<HashMap<i64, HashMap< String, HashSet<i64>>>>();

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
        if start_state < 0 || start_state >= self.nr_of_states {
            panic!("start_state '{}' not correct", start_state)
        }
        if end_state < 0 || end_state >= self.nr_of_states {
            panic!("end_state '{}' not correct", end_state)
        }

        println!("adding line ({},{},{})", start_state, label, end_state);
        
        let mut transition_dict = self.transitions.clone();
        if transition_dict.contains_key(&start_state) {
            // if start_state already in transition dict, add the given transition to its hashmap
            let mut state_map: HashMap< String, HashSet<i64>> = transition_dict.get(&start_state)
                .expect("Won't happen, see contains_key() above").clone();
            if state_map.contains_key(&String::from(label)) {
                // State map already contains this transition, 
                let mut end_states: HashSet<i64> = state_map.get(&String::from(label))
                .expect("Won't happen, see contains_key() above").clone();
                if !end_states.contains(&end_state) {
                    // this state is not yet in end_states, add it to the vector
                    end_states.insert(end_state);
                }
                state_map.insert(String::from(label), end_states);
                
            } else {
                state_map.insert(String::from(label), HashSet::from([end_state]));
            }
            transition_dict.insert(start_state, state_map);

        } else {
            // else initialize the transition ==> insert (start_state, HashMap({label, [end_state]}))
            let mut state_map: HashMap< String, HashSet<i64>> = HashMap::new();
            state_map.insert(String::from(label),  HashSet::from([end_state]));
            transition_dict.insert(start_state, state_map);
        }

        self.transitions = transition_dict;
        return self
    }

    /**
     * Get S, all states
     */
    pub fn get_all_states(self: &Self) -> HashSet<i64> {
        // Create vector with numbers 0 through nr_of_states
        let states: Vec<i64> = (0..self.nr_of_states).collect();
        // Turn into hashset
        return HashSet::from_iter(states.iter().cloned());
    }

    /**
     * Get [[ [a]f ]] (BoxModality),
     *   Get all states that have all a-transition into a state in set F
     */
    pub fn get_box_modality(self: &Self, label:String, out_states:HashSet<i64>) -> HashSet<i64> {
        let transition_dict = self.transitions.clone();
        let mut output = HashSet::new();

        for (state, state_map) in transition_dict {
            // For state
            if state_map.contains_key(&label) {
                // check if states are subset of out_states
                let target_states: &HashSet<i64> = state_map.get(&label)
                    .expect("Won't happen, see contains_key() above");
                if target_states.is_subset(&out_states) {
                    output.insert(state);
                }
            } else {
                // no "label" transitions, so add state
                output.insert(state);
            }
        } 

        return output;
    }

    /**
     * Get [[ <a>f ]] (DiamondModality),
     *   Get all states that have some a-transition into a state in set F
     */
    pub fn get_diamond_modality(self: &Self, label:String, out_states:HashSet<i64>) -> HashSet<i64> {
        let transition_dict = self.transitions.clone();
        let mut output = HashSet::new();

        for (state, state_map) in transition_dict {
            // For state
            if state_map.contains_key(&label) {
                let target_states: &HashSet<i64> = state_map.get(&label)
                    .expect("Won't happen, see contains_key() above");
                if target_states.intersection(&out_states).count() > 0 {
                    // at least one of the target_states is in out_states, i.e. some a-transition in F
                    output.insert(state);
                }
            }
        } 

        return output;
    }

}
