use uuid::Uuid;

pub struct Node<T> {
    id: Uuid,
    value: T,
    parent: Option<Uuid>,
    children: Vec<Uuid>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            value,
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn get_parent(&self) -> Option<Uuid> {
        self.parent
    }

    pub fn get_children(&self) -> &Vec<Uuid> {
        &self.children
    }
}
