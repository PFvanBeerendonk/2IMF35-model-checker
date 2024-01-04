// Specify custom type `ProgressMeasure` to hide implementation details
pub struct ProgressMeasure {
    pub data: Vec<
        Option<Vec<i64>>
    >,
}
// Here Option will turn into NULL or Vec<i64>, in case of NULL we will have T

/* NOTE: The ProgressMeasure datatype should be able to do the following things efficiently:
 * - build, store and operate on the progress mater
 * 
 * More precisely
 * - find v \in V such that ϱ1 < Liftv (ϱ)
 * - update ϱ(v)
 *  */ 
impl ProgressMeasure{
    /**
     * Initialize ϱ(v) = d-tuples of element 0 FOREACH v \in V
     * 
     * id: maximal identifier (i.e. nr_of_states)
     * d: maximal priority + 1
     */
    pub fn new(id: i64, d: i64) -> Self{
        // note that all identifiers of vertices are between 0 and id (inclusive)

        let mut data = (0..id).map(|_| Some(vec![0; d as usize])).collect::<Vec<Option<Vec<i64>>>>();

        return Self{
            data: data
        }
    }

    /**
     * Should set ϱ(v) to new_value
     */
    pub fn set(self, vertex: i64, new_value: &str) -> Self {
        
        return self
    }


    /**
     * Prog (ϱ, v , w ), for v , w ∈ V , is the least m ∈ M⊤, such that:
     * • if p(v ) is even, then m ≥p(v ) ϱ(w )
     * • if p(v ) is odd, then either m >p(v ) ϱ(w ), or, if ϱ(w ) = ⊤, also m = ⊤
     */
    pub fn prog(self, v: i64, w: i64) -> Self {
        return self
    }
}
