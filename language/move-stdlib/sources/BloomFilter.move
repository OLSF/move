module Std::BloomFilter {
        use Std::BitVector::{Self, BitVector};

        struct Filter has drop{
            bitvec: BitVector,
            k_num: u64,
            m: u64,
            n: u64,
            num_elem: u64,
        }
        
        public fun init(n: u64): Filter {
            let m = nbits(n);
            Filter{
                bitvec: BitVector::new(m),
                k_num: num_of_hashfuncs(n, m),
                m: m,
                n: n,
                num_elem: 0,
            }
        }
        public fun add(filter: &mut Filter, data: vector<u8>) {
            if (filter.num_elem < filter.n){
                let i = 0;
                while (i < filter.k_num) {
                    let index = get_index(filter, copy data, i);
                    BitVector::set(&mut filter.bitvec, index);
                    i = i+1;
                };
                filter.num_elem = filter.num_elem + 1;
            }
        }
        public fun check(filter: &Filter, data: vector<u8>): bool {
            let i = 0;
            while (i < filter.k_num) {
                let index = get_index(filter, copy data, i);
                let is_index = BitVector::is_index_set(&filter.bitvec, index);
                if(!is_index){
                    return false
                };
                i = i+1;
            };
            return true
        }
        fun get_index(filter: &Filter, data: vector<u8>, i: u64): u64 {
            let data_hash = hash(data, i);
            let index = data_hash % filter.m;
            index
        }
    native fun nbits(n: u64): u64;
    native fun num_of_hashfuncs(n: u64, m: u64): u64;
    native fun hash(data: vector<u8>, i: u64): u64;
}
