# prime
rust *u64* prime library

- Minimalist
- Miller-Rabin
- Can generate all prime numbers smaller than 2 power 64
- Provides an iterator for all prime numbers

```rust
// find the first prime number greater or equal than 1000
assert_eq!(next_prime(1000), 1009);

// list the first 4 prime numbers
assert_eq!(primes(0).take(4).collect::<Vec<_>>(), vec![2, 3, 5, 7]);
```
