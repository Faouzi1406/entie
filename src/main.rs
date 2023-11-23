use entie::{query::Query, world::World};

struct Health;
struct Name(String);

pub fn main() {
    let mut world = World::new();

    let mut ent = world.new_entity();
    ent.add_component(Health);
    ent.add_component(Name("Hello".into()));

    let mut other = world.new_entity();
    other.add_component(Name("Hello".into()));

    let q = Query((Name("Hello".into()), Name("Goodbye".into())));
}
