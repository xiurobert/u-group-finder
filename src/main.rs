use std::env;
/// Finds the gcd of `a` and `b`.
/// Uses recursion
/// Time complexity: O(log(min(a, b)))
fn find_gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        // this magically works.
        find_gcd(b, a % b)
    }
}

/// Attempts to discover a group that is multiplication modulo `n`
/// Time complexity: idk man 
/// Proof that this works: trivial and left as an exercise to the reader
fn discover_u_group(n: i32) -> Vec<i32> {
    let mut u_group = Vec::new();
    for i in 1..n {
        if find_gcd(i, n) == 1 {
            u_group.push(i);
        }
    }
    u_group.sort();
    u_group
}

// Discovers U_k(n) given U(n)
fn find_u_k_group(u_grp: &[i32], k: i32) -> Vec<i32> {
    let mut u_k_group = Vec::new();
    for i in u_grp {
        if i % k == 1 {
            u_k_group.push(*i);
        }
    }
    u_k_group.sort();
    u_k_group
}

// uses a generator to generate the powers of a number
fn discover_group_generator(mod_n: i32, generator: i32) -> Vec<i32> {
    let mut group = Vec::new();
    let mut current = generator;
    while !group.contains(&current) {
        group.push(current);
        current = (current * generator) % mod_n;
    }
    group.sort();
    group
}

// usage: <command> <modulo n> <optional: generator>
fn main() {
    // todo: use a proper argument parser, this is such a mess
    let modulo_n = env::args().nth(1).expect("Please provide the n modulo").parse::<i32>().expect("Please provide a valid integer for MOD_N");
    let generator = env::args().nth(2);
    
    // discover the group
    let u_group = discover_u_group(modulo_n);
    println!("U({}) is: {:?} | length: {}", modulo_n, u_group, u_group.len());
    
    if let Some(gen) = generator {
        let gen_num = gen.parse::<i32>().expect("Please provide a valid integer for GENERATOR");

        let group = discover_group_generator(modulo_n, gen_num);
        println!("<{}> in U({}) is: {:?} | length: {}", gen_num, modulo_n, group, group.len());
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_find_gcd() {
        assert_eq!(find_gcd(10, 5), 5);
        assert_eq!(find_gcd(10, 0), 10);
        assert_eq!(find_gcd(10, 1), 1);
        assert_eq!(find_gcd(10, 2), 2);
        assert_eq!(find_gcd(10, 3), 1);
        assert_eq!(find_gcd(10, 4), 2);
        assert_eq!(find_gcd(10, 5), 5);
        assert_eq!(find_gcd(10, 6), 2);
        assert_eq!(find_gcd(10, 7), 1);
        assert_eq!(find_gcd(10, 8), 2);
        assert_eq!(find_gcd(10, 9), 1);
    }

    #[test]
    fn test_discover_u_group() {
        assert_eq!(discover_u_group(10), vec![1, 3, 7, 9]);
        assert_eq!(discover_u_group(11), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(discover_u_group(12), vec![1, 5, 7, 11]);
    }
}
