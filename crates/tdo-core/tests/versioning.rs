extern crate tdo_core;


#[cfg(test)]
mod versions {
    use tdo_core::{tdo, list, todo};

    #[test]
    fn load01() {
        let mut tdo = tdo::Tdo::new();
        tdo.add_list(list::TodoList::new("test")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(0, "First Entry", None)).unwrap();
        let new_tdo = tdo::Tdo::load("tests/versions/version01.json").unwrap();
        new_tdo.save("upgrade_version01.json").unwrap();
    }

}
