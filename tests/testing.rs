#[cfg(test)]
mod tests {
    use drs_cli::apis;

    #[test]
    fn test_get_service_info () {
        let config = apis::configuration::Configuration::default();
        let result = apis::service_info_api::get_service_info(&config);
        assert!(!result.unwrap().name.is_empty(),"name should not be empty");

    }

    #[test]
    fn test_get_access_url () {
        let config = apis::configuration::Configuration::default();
        let obj_id = String::from("1a570e4e-2489-4218-9333-f65549495872");
        let wrong_id = String::from("1234");
        let access_id = String::from("2878d5d3-b722-4fed-b573-a066d2d51d2a");
        let result = apis::objects_api::get_access_url(&config, &obj_id, &access_id);
        assert!(!result.unwrap().url.is_empty(),"url should be present");
        assert!(apis::objects_api::get_access_url(&config, &obj_id, &wrong_id).is_err());//wrong input so should recieve error

    }

    #[test]
    fn test_get_object () {
        let config = apis::configuration::Configuration::default();
        let obj_id = String::from("1a570e4e-2489-4218-9333-f65549495872");
        let wrong_id = String::from("-2489-4218-");
        let result = apis::objects_api::get_object(&config, &obj_id, Some(false));
        assert!(!result.unwrap().access_methods.expect("REASON").is_empty(),"access methods should not be empty");
        assert!(apis::objects_api::get_object(&config, &wrong_id, Some(false)).is_err());

    }

    #[test]
    fn test_options_object () {
        let config = apis::configuration::Configuration::default();
        let obj_id = String::from("1a570e4e-2489-4218-9333-f65549495872");
        let wrong_id = String::from("789");
        let result = apis::objects_api::options_object(&config, &obj_id);
        assert!(!result.is_ok());//Has none auth issuers
        assert!(apis::objects_api::options_object(&config, &wrong_id).is_err());

    }
}