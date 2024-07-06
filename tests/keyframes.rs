#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::collections::HashSet;
    
    use cssoptiadvisor::parse;

    #[test]
    fn keyframe_to() {
        let path = Path::new("tests/datasets/keyframes/to.css");
        let mut results: HashSet<String> = HashSet::new();
        let _ = parse(path, &mut results);

        let mut expected_results: HashSet<String> = HashSet::new();
        expected_results.insert("@keyframes \"to_test\" : the value \"100%\" can be replaced by \"to\" in file \"tests/datasets/keyframes/to.css\".".to_string());
        assert_eq!(expected_results, results);
    }

    #[test]
    fn keyframe_from() {
        let path = Path::new("tests/datasets/keyframes/from.css");
        let mut results: HashSet<String> = HashSet::new();
        let _ = parse(path, &mut results);

        let mut expected_results: HashSet<String> = HashSet::new();
        expected_results.insert("@keyframes \"from_test\" : the value \"from\" can be replaced by \"0%\" in file \"tests/datasets/keyframes/from.css\".".to_string());
        assert_eq!(expected_results, results);
    }

    #[test]
    fn keyframe_duplicates() {
        let path = Path::new("tests/datasets/keyframes/duplicates.css");
        let mut results: HashSet<String> = HashSet::new();
        let _ = parse(path, &mut results);

        let mut expected_results: HashSet<String> = HashSet::new();
        expected_results.insert("@keyframes \"duplicates_test\" : the value \"20%\" already exist in file \"tests/datasets/keyframes/duplicates.css\".".to_string());
        assert_eq!(expected_results, results);
    }
}