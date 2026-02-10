#[derive(Copy, Clone)]
pub enum ComponentFlag {
    Focused,
}

pub enum Component {
    Root {},
    Text {},
    List {},
    Request {},
    Response {},
}

impl Component {}
