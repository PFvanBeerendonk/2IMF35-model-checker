use super::vertex::Vertex;

pub type Measures = Option<Vec<i64>>;
// In lectures refered to as M^T OR \N^d \union \{T\} (the set of tuples of natural numbers of length d and element T)
// Here `Option` will be NULL or Vec<i64>, we will interpret NULL as T (see lecture 8, slide 6)

// Specify custom type `ProgressMeasure` to hide implementation details
pub struct ProgressMeasure {
    pub data: Vec<Measures>,
    // For some instance `pm`, pm.data contains the progressMeasure so we can use the impl trait below
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
        if _is_even(v.priority) {
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
                if ! changed && ! _is_even(i) {
                    if ew[i as usize] < d-1 {
                        ew[i as usize] = ew[i as usize] + 1;
                        changed = true;
                    }
                } else if ! _is_even(i) {
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

    
    /**
     * Lifts the progressMeasure with respect to v
     * 
     * Returns Lift_v(pm), did_update
     *      where did_update = (pm != Lift_v(pm))
     * 
     * NOTE that if did_update, we have that pm < Lift_v(pm)
     */
    pub fn lift_v(self, v: Vertex, vertices: Vec<Vertex>) -> (Self, bool) {
        // (lecture8, slide 19/43) Define Lift_v(ϱ) for v ∈ V as follows:
        // (
        //     ϱ[v := ϱ(v) max min{Prog (ϱ, v, w) | (v, w ) ∈ E }] if v ∈ V<>
        //     ϱ[v := ϱ(v) max max{Prog (ϱ, v, w) | (v, w ) ∈ E }] if v ∈ V□
        // )
        // NOTE: the first max means "maximally apply". The second means "take max/min over the set of successors"

        // check if v ∈ V<> (else v ∈ V□)
        if v.owner == 0 {
            // ϱ[v := ϱ(v) max min{Prog (ϱ, v, w) | (v, w ) ∈ E }]
            

        } else {
            // ϱ[v := ϱ(v) max max{Prog (ϱ, v, w) | (v, w ) ∈ E }]


        }

        return (self, false);
    }
}

/**
 * Given a list of measures of equal length or None, return the minimal measure
 * Assumes that list is not empty
 */
pub fn min_measures(list: Vec<Measures>) -> Measures {
    let mut filtered: Vec<Vec<i64>> = list.clone().into_iter().flatten().collect();

    // all values are none
    if filtered.is_empty() {
        return None
    }

    let mut out_measure: Vec<i64> = vec![0; filtered[0].len()];

    // go over each character
    for char_index in 0..filtered[0].len() {
        // find smallest char
        out_measure[char_index] = filtered.iter().map(|x| x[char_index]).min().unwrap();

        // remove too large measures from filtered
        filtered.retain(|x| x[char_index] == out_measure[char_index]);

        // terminate if filtered has length 1
        if filtered.len() == 1 {
            // return the only possible answer
            return Some(filtered[0].clone())
        }
    }

    // or terminate if we compared whole measures
    return Some(out_measure)
}

/**
 * Given a list of measures of equal length or None, return the maximal measure
 * Assumes that list is not empty
 */
pub fn max_measures(list: Vec<Measures>) -> Measures {
    let mut filtered: Vec<Vec<i64>> = list.clone().into_iter().flatten().collect();

    // some value is none
    if filtered.len() < list.len() {
        return None
    }

    let mut out_measure: Vec<i64> = vec![0; filtered[0].len()];

    // go over each character
    for char_index in 0..filtered[0].len() {
        // find smallest char
        out_measure[char_index] = filtered.iter().map(|x| x[char_index]).max().unwrap();

        // remove too large measures from filtered
        filtered.retain(|x| x[char_index] == out_measure[char_index]);

        // terminate if filtered has length 1
        if filtered.len() == 1 {
            // return the only possible answer
            return Some(filtered[0].clone())
        }
    }

    // or terminate if we compared whole measures
    return Some(out_measure)
}



/**
 * Replaces from tail_start to end of vector `v` with value 0
 * 
 e.g. v=[1,2,3,4,5] with tail_start=3 should be [1,2,3,0,0]
 */
#[doc(hidden)] //Not intended for public use, pub added for testing
pub fn _tail_zeros(mut v:Vec<i64>, tail_start: i64) -> Vec<i64> {
    for i in tail_start..(v.len() as i64) {
        v[i as usize] = 0;
    }
    return v

    // TODO: check if it is possible to speed this up
}

/**
 * check if `n` is even
 */
#[doc(hidden)] //Not intended for public use, pub added for testing
pub fn _is_even(n:i64) -> bool {
    return n % 2 == 0;
}
