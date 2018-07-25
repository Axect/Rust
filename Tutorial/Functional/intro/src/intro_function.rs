pub fn f<T>(g: T, x: u32) -> u32 
where T: Fn(u32) -> u32 
{
    g(x+1) * g(x+2)    
}