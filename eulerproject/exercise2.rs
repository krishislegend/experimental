fn main() 
{
    let mut a:u64 = 1;
    let mut b:u64 = 2;
    let mut c:u64 = 0;
    let mut x:u64 = 0; 
    let mut sum:u64 =0; 
    let mut v = Vec::new();
    v.push(a);
    v.push(b);
    while c < 4000000
    {
        c = a + b; 
        a = b; 
        b = c; 
        v.push(c);
    }
    for i in &v 
    {
        if (i % 2 == 0)
        {
            sum = sum + i; 
        }
    }
    println!("The answer is {}", sum );
}
