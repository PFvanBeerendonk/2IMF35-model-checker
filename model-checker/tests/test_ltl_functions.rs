
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
            10,
        );
        // No transitions yet
        let empty_map: HashMap<i64, HashMap< String, HashSet<i64>>> = HashMap::new();

        assert_eq!(simple_ltl.transitions, empty_map);
        
        // create example transition data 
        // {0: {a: [1, 0]}}
        let mut test_map : HashMap<i64, HashMap< String, HashSet<i64>>> = HashMap::new();
        let mut sub_map: HashMap< String, HashSet<i64>> = HashMap::new();
        sub_map.insert(String::from("a"), HashSet::from([1,0])); //{a: [1, 0]}
        test_map.insert(0, sub_map);

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
        simple_ltl.add_transition(2, "a", 1); // 2 -a-> 1
        
        let out_states = HashSet::from([1, 2]);

        // [a]1 i.e. get the states where all a-transitions go into 1 or 2
        let boxmod = simple_ltl.get_box_modality(String::from("a") , out_states);
        
        let mut expected_boxmod = HashSet::new();
        expected_boxmod.insert(2);

        // TODO: testing
        assert_eq!(boxmod, expected_boxmod)
    }
}


// NOTE TO SELF: if add ALL states to hashmap (so noexistant states also work in boxMod)