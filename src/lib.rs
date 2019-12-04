fn run<F: Send>(_: F) {}

trait Object: Send {}

pub trait Trait {
    type Type;
}

impl Trait for Box<dyn Object> {
    type Type = ();
}

trait Contains<T> {}

struct IsTrait<T: Trait>(Box<dyn Contains<T::Type> + Send>);

impl<T: Trait> IsTrait<T> {
    pub fn new(item: T) -> Self {
        IsTrait(panic!())
    }
}

fn fails() {
    run(async {
        let a: Box<dyn Object> = panic!();
        let collection = IsTrait::new(a);
        async {}.await;
    })
}
