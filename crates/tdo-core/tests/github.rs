extern crate tdo_core;


#[cfg(test)]
mod github {
    use tdo_core::tdo;

    #[test]
    fn handle_token() {
        let mut tdo = tdo::Tdo::new();
        let x = tdo.get_gh_token();
        assert_eq!(x, None);
        tdo.set_gh_token(Some("ThisIsNoWorkingAuthTokenJustForTestingTheMethod"));
        assert_eq!(tdo.get_gh_token().unwrap(),
                   "ThisIsNoWorkingAuthTokenJustForTestingTheMethod".to_string());
    }
}
