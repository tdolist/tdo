extern crate tdo_core;


#[cfg(test)]
mod storage {
    use tdo_core::{tdo, list, todo};

    #[test]
    fn save() {
        let mut tdo = tdo::Tdo::new();
        tdo.add_list(list::TodoList::new("test")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(0, "First Entry", None)).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(1, "Second Entry", None)).unwrap();

        let res = tdo.save("foo.json").unwrap();
        assert_eq!(res, ());
    }

    #[test]
    fn load() {
        let mut tdo = tdo::Tdo::new();
        tdo.add_list(list::TodoList::new("test")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(0, "First Entry", None)).unwrap();

        tdo.save("foo2.json").unwrap();

        let new_tdo = tdo::Tdo::load("foo2.json").unwrap();
        assert_eq!(new_tdo.lists[1].list[0].name, "First Entry");
    }

}
