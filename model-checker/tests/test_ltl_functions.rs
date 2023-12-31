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

        simple_ltl.add_transition(0, "a", 1, false); // 0 -a-> 1
        simple_ltl.add_transition(0, "a", 0, false); // 0 -a-> 0
        assert_eq!(simple_ltl.transitions, test_map );

        // Adding same transition twice does not change the data
        simple_ltl.add_transition(0, "a", 0, false); // 0 -a-> 0
        assert_eq!(simple_ltl.transitions, test_map );
    }

    #[test]
    #[should_panic(expected = "start_state '4' not correct")]
    fn test_panic_start_state_too_big() {
        let mut simple_ltl =  Ltl::new(
            0,
            2,
            4,
        );
        simple_ltl.add_transition(4, "a", 1, false); // 4 -a-> 1, but 4 is too big
    }
    
    #[test]
    #[should_panic(expected = "end_state '3' not correct")]
    fn test_panic_end_state_too_big() {
        let mut simple_ltl: Ltl = Ltl::new(0, 2, 3);
        simple_ltl.add_transition(1, "a", 3, false); // 1 -a-> 3, but 3 is too big
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
        simple_ltl.add_transition(0, "a", 1, false); // 0 -a-> 1
        simple_ltl.add_transition(0, "a", 0, false); // 0 -a-> 0
        simple_ltl.add_transition(1, "a", 2, false); // 1 -a-> 2
        
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
        let transitions: [(i64, &str, i64); 5] = [
            (0, "a", 1),
            (0, "a", 0),
            (1, "a", 1),
            (2, "a", 1),
            (3, "b", 4),
        ];
        for (s, a, t) in transitions.iter() {
            simple_ltl.add_transition(s.clone(), a, t.clone(), false);
        }
        
        let out_states = HashSet::from([1, 2]);

        // [a]{1,2} i.e. get the states where all a-transitions go into 1 or 2
        let boxmod = simple_ltl.get_box_modality(String::from("a") , out_states);
    
        // State 1 a self loop, so should be included
        // State 2 has all outgoing a-transitions going into [1,2]
        // State 3 has no (outgoing) a-transitions, 
        // 4 has no (outgoing) transitions at all
        assert_eq!(boxmod, HashSet::from([1, 2, 3, 4]))
    }

    #[test]
    fn test_box_modality_emptyset() {
        let mut simple_ltl =  Ltl::new(
            0,
            3,
            4,
        );
        
        let transitions: [(i64, &str, i64); 4] = [
            (0, "a", 1),
            (0, "a", 0),
            (1, "a", 2),
            (2, "b", 3),
        ];
        for (s, a, t) in transitions.iter() {
            simple_ltl.add_transition(s.clone(), a, t.clone(), false);
        }
        
        let out_states = HashSet::from([]);

        // [a]{} i.e. get the states where all a-transitions go into \tempyset
        let boxmod = simple_ltl.get_box_modality(String::from("a") , out_states);

        // 2 has only outgoing b
        // 3 has no outgoing
        assert_eq!(boxmod, HashSet::from([2, 3]))
    }

    #[test]
    fn test_box_modality_emptyset_modal_operators_form3() {
        let mut simple_ltl =  Ltl::new(
            0,
            14,
            8,
        );
        let transitions: [(i64, &str, i64); 14] = [
            (0, "tau", 1),
            (0, "tau", 2),
            (1, "tau", 3),
            (1, "tau", 4),
            (2, "tau", 5),
            (2, "tau", 4),
            (3, "b", 0),
            (3, "a", 6),
            (4, "tau", 7),
            (4, "tau", 6),
            (5, "a", 0),
            (5, "a", 7),
            (6, "tau", 2),
            (7, "b", 1)
        ];
        for (s, a, t) in transitions.iter() {
            simple_ltl.add_transition(s.clone(), a, t.clone(), false);
        }
        
        let out_states = HashSet::from([]);

        // [tau]{} i.e. get the states where all a-transitions go into \tempyset
        let boxmod = simple_ltl.get_box_modality(String::from("tau") , out_states);

        assert_eq!(boxmod, HashSet::from([3, 5, 7]))
    }
}


#[cfg(test)]
mod test_get_diamond_modality {
    use model_checker::types::ltl::Ltl;
    use std::collections::HashSet;

    #[test]
    fn test_diamond_modality_regular() {
        let mut simple_ltl =  Ltl::new(
            0,
            3,
            3,
        );
        simple_ltl.add_transition(0, "a", 1, false); // 0 -a-> 1
        simple_ltl.add_transition(0, "a", 0, false); // 0 -a-> 0
        simple_ltl.add_transition(1, "a", 2, false); // 1 -a-> 2
        
        let out_states = HashSet::from([1]);

        // [a]{1} i.e. get the states where all a-transitions go into 1
        let diamod = simple_ltl.get_diamond_modality(String::from("a") , out_states);

        assert_eq!(diamod, HashSet::from([0]))
    }

    #[test]
    fn test_diamond_modality_emptyset() {
        let mut simple_ltl =  Ltl::new(
            0,
            3,
            4,
        );
        simple_ltl.add_transition(0, "a", 1, false); // 0 -a-> 1
        simple_ltl.add_transition(0, "a", 0, false); // 0 -a-> 0
        simple_ltl.add_transition(1, "a", 2, false); // 1 -a-> 2
        simple_ltl.add_transition(2, "b", 3, false); // 1 -a-> 2
        
        let out_states = HashSet::from([]);

        // [a]{} i.e. get the states where all a-transitions go into 1
        let diamod = simple_ltl.get_diamond_modality(String::from("a") , out_states);

        assert_eq!(diamod, HashSet::from([]))
    }
}
