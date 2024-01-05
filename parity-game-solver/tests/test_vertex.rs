#[cfg(test)]
mod test_new {
    use parity_game_solver::types::vertex::Vertex;

    #[test]
    fn test_get_all_states() {

        let v = Vertex::new(
            3,
            2,
            1, 
            Vec::<i64>::from([5,7]),
        );

        assert_eq!(v.identifier, 3);
        assert_eq!(v.priority, 2);
        assert_eq!(v.owner, 1);
        assert_eq!(v.successors, Vec::<i64>::from([5,7]));
    }
}
