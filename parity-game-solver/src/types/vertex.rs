
// Specify custom type `Vertex`
// follows `https://www.mcrl2.org/web/user_manual/tools/lts.html`

pub type Vertices = Vec<Option<Vertex>>;

#[derive(Clone)]
pub struct Vertex {
    pub identifier: i64,
    pub priority: i64,
    pub owner: i64, // here owner 0 == V<> and owner 1 == V□
    pub successors: Vec<i64>,
}

/* NOTE: The NOde datatype should be able to do the following things efficiently:
 * - build from a .gm file
 * 
 * - return priority
 * - return successor identifiers
 */ 
impl Vertex{
    /**
     * Initialize node
     */
    pub fn new(identifier: i64, priority: i64, owner:i64, successors: Vec<i64>) -> Self{

        return Self {
            identifier: identifier,
            priority: priority, 
            owner: owner,
            successors: successors,
        }
    }

}
