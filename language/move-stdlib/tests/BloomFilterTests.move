#[test_only]
module Std::BloomFilterTests {
    use Std::BloomFilter;

    #[test]
    fun filter_test() {
        let filter = BloomFilter::init(10);
        BloomFilter::add(&mut filter, b"hello");
        BloomFilter::add(&mut filter, b"world");

        assert!(BloomFilter::check(&filter, b"hello") == true, 0);
        assert!(BloomFilter::check(&filter, b"world") == true, 1);
        assert!(BloomFilter::check(&filter, b"mars") != true, 2);
    }
}
