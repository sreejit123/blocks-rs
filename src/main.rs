use crate::bloom_filter::BloomFilter;

mod bloom_filter;

fn main() {
    let mut bloom_filter : BloomFilter = BloomFilter::new(3, 10).unwrap();
    bloom_filter.insert("Hello");
    assert_eq!("Maybe", bloom_filter.search("Hello"));
    assert_eq!("Not present", &bloom_filter.search("World"));
    println!("Demo complete !!");

}
