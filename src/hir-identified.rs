#[prelude_import]
use ::std::prelude::v1::*; /* node_id: 3 hir local_id: 0 */
#[macro_use]
extern crate std; /* node_id: 9 hir local_id: 0 */
fn main() ({
               ((<Person>::new /* node_id: 15 hir local_id: 5
                    */)(("not_bind" /* node_id: 16 hir local_id: 6 */),
                        (18 /* node_id: 17 hir local_id: 7 */)) /*
                   node_id: 18 hir local_id: 8 */);
               let kiske /* pat node_id: 20 hir local_id: 10 */ =
                   ((<Person>::new /* node_id: 23 hir local_id: 14
                        */)(("kiske" /* node_id: 24 hir local_id: 15 */),
                            (18 /* node_id: 25 hir local_id: 16 */)) /*
                       node_id: 26 hir local_id: 17 */);
           } /* block node_id: 12 hir local_id: 19 */ /*
              node_id: 67 hir local_id: 20 */) /* node_id: 10 hir local_id: 0
*/

struct Person {
    pub name: String,
    pub age: u32,
} /* node_id: 27 hir local_id: 0 */

impl Person {
    fn new(name /* pat node_id: 43 hir local_id: 14 */: &str,
           age /* pat node_id: 48 hir local_id: 16 */: u32)
     ->
         Person ({
                     (Person{name:
                                 ((name /* node_id: 57 hir local_id: 5
                                      */).to_string() /*
                                     node_id: 58 hir local_id: 6 */),
                             (age /* node_id: 60 hir local_id: 9 */),} /*
                         node_id: 61 hir local_id: 10 */)
                 } /* block node_id: 53 hir local_id: 11 */ /*
                    node_id: 70 hir local_id: 12 */)
    /*
    40
    */
} /* node_id: 36 hir local_id: 0 */
