use uuid::Uuid;

pub type ComponentIdentifier = Uuid;

pub trait Component {
    fn set_active(&mut self, state: bool);
}
