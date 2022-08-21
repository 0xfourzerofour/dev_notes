/// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum InsertType {
    /// Terminal tick.
    Project,
    /// Key press.
    Note,
}

/// Terminal event handler.
#[derive(Debug)]
pub struct InsertHandler<'a> {
    insert_type: InsertType,
    insert_text: &'a mut str,
}

// impl<'a> InsertHandler<'a> {
//     pub fn new() -> Self {
//         Self {
//             insert_type: InsertType::Project,
//             insert_text: "",
//         }
//     }

//     pub fn write(&self, input: &'a mut str) -> () {
//         self.insert_text = input;
//     }
// }
