
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

    #[test]
    fn test_add_transition() {
        let mut simple_ltl =  Ltl::new(
            0,
            2,
            10,
        );
        // No transitions yet
        let empty_map: HashMap<i64, HashMap< String, Vec<i64>>> = HashMap::new();

        assert_eq!(simple_ltl.transitions, empty_map);
        
        // create example transition data 
        // {0: {a: [1, 0]}}
        let mut test_map : HashMap<i64, HashMap< String, Vec<i64>>> = HashMap::new();
        let mut sub_map: HashMap< String, Vec<i64>> = HashMap::new();
        sub_map.insert(String::from("a"), vec![1, 0]); //{a: [1, 0]}
        test_map.insert(0, sub_map);

        simple_ltl.add_transition(0, "a", 1); // 0 -a-> 1
        simple_ltl.add_transition(0, "a", 0); // 0 -a-> 0
        assert_eq!(simple_ltl.transitions, test_map );

        // Adding same transition twice does not change the data
        simple_ltl.add_transition(0, "a", 0); // 0 -a-> 0
        assert_eq!(simple_ltl.transitions, test_map );
    }
}
