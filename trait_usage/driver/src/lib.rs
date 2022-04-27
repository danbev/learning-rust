use sometrait::Something;

pub struct Driver {}

impl Driver {
    pub fn process(s: &dyn Something) -> () {
        s.doit();
    }
}
