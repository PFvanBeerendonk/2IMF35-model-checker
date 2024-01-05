use super::vertex::Vertex;

pub type Measures = Option<Vec<i64>>;
// In lectures refered to as M^T OR \N^d \union \{T\} (the set of tuples of natural numbers of length d and T)
// Here `Option` will be NULL or Vec<i64>, we will interpret NULL as T (see lecture 8, slide 6)

// Specify custom type `ProgressMeasure` to hide implementation details
pub struct ProgressMeasure {
    pub data: Vec<Measures>,
}


/* NOTE: The ProgressMeasure datatype should be able to do the following things efficiently:
 * - build, store and operate on the progress mater
 * 
 * More precisely
 * - find v \in V such that ϱ1 < Liftv (ϱ)
 * - update ϱ(v)
 * - find Prog(ϱ, v, w)
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

        let data = (0..id).map(|_| Some(vec![0; d as usize])).collect::<Vec<Measures>>();

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
     * Params: Vertex v, w and d: max_priority
     * 
     * Prog (ϱ, v , w ), for v , w ∈ V , is the least m ∈ M⊤, such that:
     * • if p(v ) is even, then m ≥p(v ) ϱ(w )
     * • if p(v ) is odd, then either m >p(v ) ϱ(w ), or, if ϱ(w ) = ⊤, also m = ⊤
     * NOTE that >p(v) denotes lexicographical order up to index p(v) with T being largest
     * 
     * Assume that (v,w) \in E
     * 
     * @Returns m
     */
    pub fn prog(self, v: Vertex, w: Vertex, d: i64) -> Measures {
        // is even
        if v.priority % 2 == 0 {
            let mut m : Vec<i64> = vec![0; d as usize];
            // get least m with m >=_{p(v)} ϱ(w)
            if self.data[w.identifier as usize].is_none() {
                return None
            } else {
                // replace everything beyond index p(v) with 0s
                let mut ew = self.data[w.identifier as usize].clone().unwrap();
                ew = _tail_zeros(ew, v.priority+1);

                return Some(ew);
            }
        } else {
            // check if ϱ(w) = T
            if self.data[w.identifier as usize].is_none() {
                return None;
            }

            // get least m >_{p(v)} ϱ(w)
            let mut ew = self.data[w.identifier as usize].clone().unwrap();
            let mut changed = false;

            for i in 0..v.priority+1 {
                if ! changed && i % 2 == 1 {
                    if ew[i as usize] < d-1 {
                        ew[i as usize] = ew[i as usize] + 1;
                        changed = true;
                    }
                } else if i % 2 == 1 {
                    ew[i as usize] = 0;
                }
            }

            // from 0 through p(v) there is nothing we can increase, thus we return T
            if ! changed {
                return None
            }

            // tail with 0s
            ew = _tail_zeros(ew, v.priority+1);

            return Some(ew);
        }
    }
}

/**
 * Replaces from tail_start to end of vector `v` with value 0
 */
#[doc(hidden)] //Not intended for pubilc use, pub added for testing
pub fn _tail_zeros(mut v:Vec<i64>, tail_start: i64) -> Vec<i64> {
    for i in tail_start..(v.len() as i64) {
        v[i as usize] = 0;
    }
    return v

    // TODO: check if it is possible to speed this up
}
