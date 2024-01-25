mod vegetables;

/*
mod vegetables {
    //code
}
*/

/*
Declaring submodules: In any file other than the crate root, you can declare submodules. For example, you might declare mod vegetables; in src/garden.rs. The compiler will look for the submodule’s code within the directory named for the parent module in these places:
1. Inline, directly following mod vegetables, within curly brackets instead of the semicolon
2. In the file src/garden/vegetables.rs
3. In the file src/garden/vegetables/mod.rs
*/

/*backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
*/
/*
The "use" keyword: Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to "crate::garden::vegetables::Asparagus", you can create a shortcut with "use crate::garden::vegetables::Asparagus"; and from then on you only need to write "Asparagus" to make use of that type in the scope */

/*
 use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
 */

//  code within a module is private by default