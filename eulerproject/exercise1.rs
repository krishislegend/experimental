fn main() 
{
    let mut x:u64 = 1;
    let mut sum :u64 = 0; 
    let mut v = Vec::new();
    while x < 1000
    {
        if (x % 3 == 0) || (x % 5 == 0)
        {
            v.push(x);
        }
        x =  x + 1; 
    }
    for i in &v 
    {
        sum = sum +i ;
    }
    println!("The answer is {}", sum);
}
