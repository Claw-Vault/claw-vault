use rand::RngCore;

#[test]
fn arc_clone_test() {
    let arr = String::from("Hello");
    let arr = std::sync::Arc::new(arr);
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
    let a = generate_id();
    let b = generate_id();
    println!("{} - {}", a, b);
    assert_ne!(a, b);
}

fn generate_id() -> String {
    let mut rng = rand::thread_rng();
    let mut b = [0u8; 3];
    rng.fill_bytes(&mut b);

    b.iter().map(|i| format!("{:x}", i)).collect()
}
