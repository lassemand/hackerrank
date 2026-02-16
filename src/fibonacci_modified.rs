fn from_u32(mut x: u32) -> Vec<u8> {
    let mut v = Vec::new();
    while x > 0 {
        v.push((x & 0xFF) as u8);
        x >>= 8;
    }
    if v.is_empty() {
        v.push(0);
    }
    v
}

fn trim(v: &mut Vec<u8>) {
    while v.len() > 1 && *v.last().unwrap() == 0 {
        v.pop();
    }
}

fn add(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(a.len().max(b.len()) + 1);
    let mut carry = 0u16;

    for i in 0..a.len().max(b.len()) {
        let av = *a.get(i).unwrap_or(&0) as u16;
        let bv = *b.get(i).unwrap_or(&0) as u16;
        let sum = av + bv + carry;
        res.push((sum & 0xFF) as u8);
        carry = sum >> 8;
    }

    if carry > 0 {
        res.push(carry as u8);
    }

    res
}

fn square(a: &[u8]) -> Vec<u8> {
    let mut res = vec![0u8; a.len() * 2];

    for i in 0..a.len() {
        let mut carry = 0u32;

        for j in 0..a.len() {
            let idx = i + j;
            let cur =
                res[idx] as u32 +
                (a[i] as u32 * a[j] as u32) +
                carry;

            res[idx] = (cur & 0xFF) as u8;
            carry = cur >> 8;
        }

        let mut k = i + a.len();
        while carry > 0 {
            let cur = res[k] as u32 + carry;
            res[k] = (cur & 0xFF) as u8;
            carry = cur >> 8;
            k += 1;
        }
    }

    trim(&mut res);
    res
}

fn to_decimal(bytes: Vec<u8>) -> String {
    const BASE: u64 = 1_000_000_000;

    let mut dec: Vec<u64> = vec![0];

    for &byte in bytes.iter().rev() {
        let mut carry = byte as u64;

        for d in dec.iter_mut() {
            let cur = (*d << 8) + carry;
            *d = cur % BASE;
            carry = cur / BASE;
        }

        if carry > 0 {
            dec.push(carry);
        }
    }

    let mut s = String::new();
    for (i, &d) in dec.iter().rev().enumerate() {
        if i == 0 {
            s.push_str(&d.to_string());
        } else {
            s.push_str(&format!("{:09}", d));
        }
    }

    s
}

pub fn fibonacci_modified(t1: u32, t2: u32, n: i32) -> String {
    if n == 1 {
        return t1.to_string();
    }
    if n == 2 {
        return t2.to_string();
    }

    let mut a = from_u32(t1);
    let mut b = from_u32(t2);

    for _ in 0..(n - 2) {
        let b2 = square(&b);
        let next = add(&a, &b2);
        a = b;
        b = next;
    }

    to_decimal(b)
}

#[cfg(test)]
mod tests {
    use crate::fibonacci_modified::fibonacci_modified;

    #[test]
    fn single_node() {
        assert_eq!(fibonacci_modified(0, 1, 6), "27");
    }
}

