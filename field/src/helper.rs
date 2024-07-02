use crate::field::Field;

pub fn naive_poly_mul<F:Field>(a:&[F],b:&[F]) ->Vec<F>{
    //计算两个多项式相乘
    let mut product = vec![F::ZERO;a.len()+b.len()-1];
    for (i,c1) in a.iter().enumerate()  {
        for (j,c2) in b.iter().enumerate() {
            product[i+j] += c1.clone()*c2.clone();
        }
    }
    product
}