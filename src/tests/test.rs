use std::{sync::Arc, time};

use base64::Engine;
use rand::RngCore;

#[test]
fn arc_clone_test() {
    let arr = String::from("Hello");
    let arr = Arc::new(arr);
    let brr = arr.clone();

    let arc_p_arr = format!("{:p}", &arr);
    let arc_p_brr = format!("{:p}", &brr);

    println!("arc arr: {}", arc_p_arr);
    println!("arc brr: {}", arc_p_brr);

    assert_ne!(arc_p_arr, arc_p_brr);

    let p_arr = format!("{:p}", std::ops::Deref::deref(&arr).as_ptr());
    let p_brr = format!("{:p}", std::ops::Deref::deref(&brr).as_ptr());

    println!("p_arr: {}", p_arr);
    println!("p_brr: {}", p_brr);

    assert_eq!(arr, brr);
    assert_eq!(p_arr, p_brr);
}

#[test]
fn fermat_primes() {
    let mut res = Vec::<u32>::new();
    for i in 1..10u32 {
        let m = 2u32.pow(i);
        let n = 2u64.pow(m);
        let x = u32::MAX as u64;
        if n > x {
            if n + 1 > x {
                break;
            }
        }
        res.push((n + 1) as u32);
    }
    println!("{:?}", res);
    assert_eq!(res, vec![5, 17, 257, 65537])
}

#[test]
fn rand_req_id() {
    let a: String = gen_req_id();
    let b: String = gen_req_id();
    println!("{} - {}", a, b);
    assert_ne!(a, b);
}

fn gen_req_id() -> String {
    let mut rng = rand::thread_rng();
    let mut b = [0u8; 3];
    rng.fill_bytes(&mut b);

    hex::encode(b)
}

#[test]
fn milli_base_64() {
    let engine = base64::engine::GeneralPurpose::new(
        &base64::alphabet::URL_SAFE,
        base64::engine::general_purpose::NO_PAD,
    );

    let milli = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
    let milli = format!("{}", milli);
    println!("{}", milli);

    let mut id = String::new();
    engine.encode_string(milli.as_bytes(), &mut id);

    println!("{}", id);
    assert!(id.len() >= 18);
}

#[test]
fn sha256() {
    let data = "Hello world !";
    let mut sha = openssl::sha::Sha256::new();
    sha.update(data.as_bytes());
    let hash = sha.finish();

    let hash = hex::encode(hash);
    let expected = String::from("2f951d3adf29ab254d734286755e2131c397b6fc1894e6ffe5b236ea5e099ecf");
    assert_eq!(hash, expected);
}

#[test]
fn hex_str_bench() {
    let start = time::Instant::now();
    for _ in 0..100_000 {
        let data = "Hello world !";
        let mut sha = openssl::sha::Sha256::new();
        sha.update(data.as_bytes());
        let hash = sha.finish();

        let _ = hash
            .iter()
            .map(|i| format!("{:x}", i))
            .fold(String::new(), |mut op, i| {
                op += i.as_str();
                op
            });
    }
    let s = start.elapsed().as_millis();
    println!("fold_str: {} ms", s);

    let start = time::Instant::now();
    for _ in 0..100_000 {
        let data = "Hello world !";
        let mut sha = openssl::sha::Sha256::new();
        sha.update(data.as_bytes());
        let hash = sha.finish();

        let hash = hash
            .iter()
            .map(|b| format!("{:x}", b))
            .fold(Vec::<u8>::new(), |mut vec, b| {
                for i in b.as_bytes() {
                    vec.push(*i)
                }
                vec
            });
        let _ = String::from_utf8(hash).unwrap();
    }
    let v = start.elapsed().as_millis();
    println!("fold_vec: {} ms", v);

    let start = time::Instant::now();
    for _ in 0..100_000 {
        let data = "Hello world !";
        let mut sha = openssl::sha::Sha256::new();
        sha.update(data.as_bytes());
        let hash = sha.finish();

        let _ = hex::encode(hash);
    }
    let h = start.elapsed().as_millis();
    println!("fold_hex: {} ms", h);

    assert!(h < v);
    assert!(h < s);
}
