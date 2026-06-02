use std::cell::RefCell;
use std::rc::Rc;

pub struct DropLogger {
    logs: Rc<RefCell<Vec<u32>>>,
    next_item_id: u32,
}

impl DropLogger {
    pub fn new() -> Self {
        Self {
            logs: Rc::new(RefCell::new(Vec::new())),
            next_item_id: 0,
        }
    }

    pub fn logs(&self) -> Vec<u32> {
        self.logs.borrow().iter().cloned().collect()
    }

    pub fn create_item(&mut self) -> DropItem {
        let logs = Some(self.logs.clone());
        let id = self.next_item_id;
        self.next_item_id += 1;
        DropItem { logs, id }
    }
}

#[derive(Default)]
pub struct DropItem {
    logs: Option<Rc<RefCell<Vec<u32>>>>,
    id: u32,
}

impl Drop for DropItem {
    fn drop(&mut self) {
        if let Some(logs) = self.logs.as_mut() {
            logs.borrow_mut().push(self.id);
        }
    }
}
