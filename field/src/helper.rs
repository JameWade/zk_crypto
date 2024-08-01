use crate::field::Field;

// pub fn naive_poly_mul<F:Field>(a:&[F],b:&[F]) ->Vec<F>{
//     //计算两个多项式相乘
//     let mut product = vec![F::ZERO;a.len()+b.len()-1];
//     for (i,c1) in a.iter().enumerate()  {
//         for (j,c2) in b.iter().enumerate() {
//             product[i+j] += c1.clone()*c2.clone();
//         }
//     }
//     product
// }

fn xgcd(a:i64,b:i64) ->(i64,i64,i64){
    if b==0 {
        (a,1,0)
    }else {
        let (g,x,y) = xgcd(b,a%b);
        (g,y,x-(a/b)*y)
    }
}

#[cfg(test)]
#[test]
fn test_xgcd(){
    let a = 48;
    let b = 18;
    let (g, x, y) = xgcd(a, b);
    println!("For a = {} and b = {}:", a, b);
    println!("GCD: {}", g);
    println!("Coefficients: x = {}, y = {}", x, y);
    println!("Verification: {}*{} + {}*{} = {}", a, x, b, y, g);
}