pub trait TraitA {
    fn banana();
}

pub trait TraitB {
    fn banana();
}

pub fn teste(_item: &(impl TraitA + TraitB)) {}
