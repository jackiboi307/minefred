type Amount = u8;

pub struct InventoryItem {
    pub key: &'static str,
    pub amount: Amount,
}

pub struct Inventory {
    pub items: Box<[Option<InventoryItem>]>,
}

impl Inventory {
    pub fn new(slots: usize) -> Self {
        let mut items = Vec::with_capacity(slots);

        for _ in 0..slots {
            items.push(None);
        }

        Self {
            items: items.into_boxed_slice(),
        }
    }

    pub fn try_receive(&mut self, key: &'static str, amount: Amount) {
        if let Some(Some(item)) = self.items.iter_mut().find(|i|
                    if let Some(i) = i {
                        i.key == key
                    } else {
                        false
                    }
                ) {

            item.amount += amount;
            // TODO check for threshold reached

        } else if let Some(first_empty_index)
                = self.items.iter().position(|i| i.is_none()) {

            self.items[first_empty_index] = Some(InventoryItem {
                key,
                amount,
            });
        }
    }
}
