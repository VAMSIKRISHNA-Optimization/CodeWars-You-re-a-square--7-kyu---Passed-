fn is_square(n: i64) -> bool 
{
    if n < 0 { return false; }
    else if n == 0 { return true; }
    else
    {
        let mut num: i64 = 1;
        
        while num * num <= n
        {
            if num * num == n { return true; }
            num += 1;
        }
        
        return false;
    }
}
