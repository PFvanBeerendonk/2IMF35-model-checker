

#[cfg(test)]
mod tests {
    use model_checker::types::ltl::StateSet;
    use model_checker::types::ltl::Union;

    #[test]
    fn test_union_all_states() {
        let a = StateSet{all_states:true,  states:None} ;
        let b = StateSet{all_states:false,  states:Some(vec![0, 1, 2])};
        
        let result = a.union(b) ;
        assert_eq!(result.all_states, true);
        assert_eq!(result.states, None);
    }
}

