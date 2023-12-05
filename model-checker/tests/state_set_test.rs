

#[cfg(test)]
mod test_union {
    use model_checker::types::state_set::StateSet;
    use model_checker::types::state_set::Union;

    #[test]
    fn test_union_all_states() {
        let a = StateSet{all_states:true,  states:None} ;
        let b = StateSet{all_states:false,  states:Some(vec![0, 1, 2])};
        
        let result = a.union(b) ;
        assert_eq!(result.all_states, true);
        assert_eq!(result.states, None);
    }

    #[test]
    fn test_union_some_states() {
        let a = StateSet{all_states:false,  states:Some(vec![0, 1, 2])} ;
        let b = StateSet{all_states:false,  states:Some(vec![0, 3, 4])};
        
        let result = a.union(b) ;
        assert_eq!(result.all_states, false);
        assert_eq!(result.states, Some(vec![0,1,2,3,4]));
    }
}

mod test_intersect {
    use model_checker::types::state_set::StateSet;
    use model_checker::types::state_set::Intersect;

    #[test]
    fn test_intersect_all_states() {
        let a = StateSet{all_states:true,  states:None} ;
        let b = StateSet{all_states:false,  states:Some(vec![0, 1, 2])};
        
        let result = a.intersect(b) ;
        assert_eq!(result.all_states, false);
        assert_eq!(result.states, Some(vec![0, 1, 2]));
    }

    #[test]
    fn test_intersect_some_states() {
        let a = StateSet{all_states:false,  states:Some(vec![0, 1, 2])} ;
        let b = StateSet{all_states:false,  states:Some(vec![0, 3, 4])};
        
        let result = a.intersect(b) ;
        assert_eq!(result.all_states, false);
        assert_eq!(result.states, Some(vec![0]));
    }
}
