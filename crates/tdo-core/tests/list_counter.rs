extern crate tdo_core;


#[cfg(test)]
mod counter {
    use tdo_core::{tdo, list, todo};

    #[test]
    fn highest_id_single_list() {
        let mut tdo = tdo::Tdo::new();
        tdo.add_list(list::TodoList::new("test")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(0, "First Entry", None))
            .unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(1, "Second Entry", None))
            .unwrap();

        assert_eq!(tdo.get_highest_id(), 1);
    }

    #[test]
    fn highest_id_multiple_list() {
        let mut tdo = tdo::Tdo::new();
        tdo.add_list(list::TodoList::new("test")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(0, "First Entry", None))
            .unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(1, "Second Entry", None))
            .unwrap();

        tdo.add_todo(None, todo::Todo::new(2, "Random third Entry", None))
            .unwrap();
        tdo.add_todo(None, todo::Todo::new(3, "Funny fourth Entry", None))
            .unwrap();

        assert_eq!(tdo.get_highest_id(), 3);
    }

    #[test]
    fn remove_number_between() {
        let mut tdo = tdo::Tdo::new();
        tdo.add_list(list::TodoList::new("test")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(0, "First Entry", None))
            .unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(1, "Second Entry", None))
            .unwrap();

        tdo.add_todo(None, todo::Todo::new(2, "Random third Entry", None))
            .unwrap();
        tdo.add_todo(None, todo::Todo::new(3, "Funny fourth Entry", None))
            .unwrap();

        tdo.remove_id(2).unwrap();
        assert_eq!(tdo.get_highest_id(), 3);
    }

    #[test]
    fn remove_highest_id() {
        let mut tdo = tdo::Tdo::new();
        tdo.add_list(list::TodoList::new("test")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(0, "First Entry", None))
            .unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(1, "Second Entry", None))
            .unwrap();

        tdo.add_todo(None, todo::Todo::new(2, "Random third Entry", None))
            .unwrap();
        tdo.add_todo(None, todo::Todo::new(3, "Funny fourth Entry", None))
            .unwrap();

        tdo.remove_id(3).unwrap();
        assert_eq!(tdo.get_highest_id(), 2);
    }

    #[test]
    fn move_item_between_lists() {
        let mut tdo = tdo::Tdo::new();
        tdo.add_list(list::TodoList::new("test")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(0, "First Entry", None))
            .unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(1, "Second Entry", None))
            .unwrap();

        tdo.add_todo(None, todo::Todo::new(2, "Random third Entry", None))
            .unwrap();
        tdo.add_todo(None, todo::Todo::new(3, "Funny fourth Entry", None))
            .unwrap();

        assert_eq!(tdo.move_todo(3, "test").unwrap(),
                   (),
                   "Moving the todo didn't work!");
        let list = tdo.find_id(3).unwrap();
        assert_eq!(tdo.lists[list].list.len(), 3);

        // make sure the new element is at the last position
        assert_eq!(tdo.lists[list].list[2].id, 3);

        tdo.add_todo(None, todo::Todo::new(4, "But can you pass this test?", None))
            .unwrap();
        tdo.move_todo(3, "default").unwrap();

        // make sure the element moved out of the list again
        assert_eq!(tdo.lists[list].list.len(), 2);
        let new_list = tdo.find_id(3).unwrap();
        assert_eq!(tdo.lists[new_list].list.len(), 3);
        // make sure the new element is at the  position
        assert_eq!(tdo.lists[new_list].list[1].id, 3);
    }
}
