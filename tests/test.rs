extern crate vlfeat;

#[test]
fn test_new() {
    unsafe {
        let vlsift = vlfeat::vl_sift_new(100, 100, 5, 5, 5);
        assert_eq!((*vlsift).width, 100);
    }
}
