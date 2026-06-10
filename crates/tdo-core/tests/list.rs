extern crate tdo_core;

#[cfg(test)]
mod list_operation {
    use tdo_core::{tdo, list};

    #[test]
    #[should_panic]
    fn removing_default() {
        let mut tdo = tdo::Tdo::new();
        tdo.remove_list("default").unwrap();
    }

    #[test]
    fn removing_lists() {
        let mut tdo = tdo::Tdo::new();
        tdo.add_list(list::TodoList::new("deletion")).unwrap();
        tdo.remove_list("deletion").unwrap();
        assert_eq!(tdo::Tdo::new().lists.len(), tdo.lists.len())
    }

    #[test]
    #[should_panic]
    fn same_list_twice() {
        let mut tdo = tdo::Tdo::new();
        tdo.add_list(list::TodoList::new("default")).unwrap()
    }
}
