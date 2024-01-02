use std::collections::HashSet;
use std::collections::HashMap;

// Specify custom type `Game`
// follows `https://www.mcrl2.org/web/user_manual/tools/lts.html`

pub struct Game {
    pub first_state: i64,
    pub transitions: HashMap< i64, HashMap< String, HashSet<i64> > >, 
    // state -> state_map where state_map: label -> [state]

    pub nr_of_states: i64,
}

/* NOTE: The Game datatype should be able to do the following things efficiently:
 * - build from a .gm file
 * 
 * - build, store and operate on the progress mater
 *  */ 
impl Game{
    /**
     * Initialize 
     */
    pub fn new(first_state: i64, _: i64, nr_of_states: i64) -> Self{
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
     * 
     */
    pub fn add_transition(&mut self, start_state: i64, label: &str, end_state: i64, debug: bool) -> &mut Self {
        
        return self
    }

}
