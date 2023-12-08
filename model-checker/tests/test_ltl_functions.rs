
#[cfg(test)]
mod test_get_all_states {
    use model_checker::types::ltl::Ltl;
    use std::collections::HashSet;

    #[test]
    fn test_get_all_states() {
        let simple_ltl =  Ltl::new(
            0,
            0,
            10,
        );
        
        let result = simple_ltl.get_all_states();

        assert_eq!(result, HashSet::from([0,1,2,3,4,5,6,7,8,9]));
    }
}

#[cfg(test)]
mod test_insert_transition {
    use model_checker::types::ltl::Ltl;
    use std::collections::HashMap;
    use std::collections::HashSet;

    #[test]
    fn test_add_transition() {
        let mut simple_ltl =  Ltl::new(
            0,
            2,
            4,
        );
        // No transitions yet, but should have made an empty transistion map for each state
        let empty_map: HashMap<i64, HashMap< String, HashSet<i64>>> =  HashMap::from([
            (0, HashMap::new()),
            (1, HashMap::new()),
            (2, HashMap::new()),
            (3, HashMap::new()),
        ]);

        assert_eq!(simple_ltl.transitions, empty_map);
        
        // create example transition data 
        // {0: {a: [1, 0]}}
        let test_map: HashMap<i64, HashMap<String, HashSet<i64>>> = HashMap::from([
            (0, HashMap::from(
                [(String::from("a"), HashSet::from([1,0]))]
            )),
            (1, HashMap::new()),
            (2, HashMap::new()),
            (3, HashMap::new()),
        ]);

        simple_ltl.add_transition(0, "a", 1); // 0 -a-> 1
        simple_ltl.add_transition(0, "a", 0); // 0 -a-> 0
        assert_eq!(simple_ltl.transitions, test_map );

        // Adding same transition twice does not change the data
        simple_ltl.add_transition(0, "a", 0); // 0 -a-> 0
        assert_eq!(simple_ltl.transitions, test_map );
    }
}


#[cfg(test)]
mod test_get_box_modality {
    use model_checker::types::ltl::Ltl;
    use std::collections::HashSet;

    #[test]
    fn test_box_modality_regular() {
        let mut simple_ltl =  Ltl::new(
            0,
            3,
            3,
        );
        simple_ltl.add_transition(0, "a", 1); // 0 -a-> 1
        simple_ltl.add_transition(0, "a", 0); // 0 -a-> 0
        simple_ltl.add_transition(1, "a", 2); // 1 -a-> 2
        
        let out_states = HashSet::from([1]);

        // [a]{1} i.e. get the states where all a-transitions go into 1
        let boxmod = simple_ltl.get_box_modality(String::from("a") , out_states);

        assert_eq!(boxmod, HashSet::from([2]))
    }

    #[test]
    fn test_box_mod_state_with_no_a() {
        // Box modality should also work for states that have NO a-transitions, or NO transitions at all

        let mut simple_ltl =  Ltl::new(
            0,
            3,
            5,
        );
        simple_ltl.add_transition(0, "a", 1); // 0 -a-> 1
        simple_ltl.add_transition(0, "a", 0); // 0 -a-> 0
        simple_ltl.add_transition(1, "a", 1); // 1 -a-> 1
        simple_ltl.add_transition(2, "a", 1); // 2 -a-> 1
        simple_ltl.add_transition(3, "b", 4); // 3 -b-> 4
        
        let out_states = HashSet::from([1, 2]);

        // [a]{1,2} i.e. get the states where all a-transitions go into 1 or 2
        let boxmod = simple_ltl.get_box_modality(String::from("a") , out_states);
    
        // State 1 a self loop, so should be included
        // State 2 has all outgoing a-transitions going into [1,2]
        // State 3 has no (outgoing) a-transitions, 
        // 4 has no (outgoing) transitions at all
        assert_eq!(boxmod, HashSet::from([1, 2, 3, 4]))
    }

}


// NOTE TO SELF: if add ALL states to hashmap (so noexistant states also work in boxMod)