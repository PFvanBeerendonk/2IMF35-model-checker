#[cfg(test)]
mod test_new {
    use parity_game_solver::types::vertex::Vertex;

    #[test]
    fn test_get_all_states() {

        let v =  Vertex::new(
            1,
            0,
            1, 
            Vec::<i64>::from([1,2]),
        );

        assert_eq!(v.identifier, 1);
        assert_eq!(v.priority, 0);
        assert_eq!(v.owner, 0);
        assert_eq!(v.successors, Vec::<i64>::from([1,2]));
    }
}
