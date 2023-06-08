/// Cursed version that only allocates one HashMap and mem::translates the u64 counts into f64 probabilities
/// because why not
/// Should be safe because f64 and u64 take up the same space in memory
/// I would do this because incrementing with ints is always accurate. I could also just use floats in the occurance count 
/// but that's lame
#[allow(dead_code)]
fn from_symbols_iter_unsafe<T>(symbols: impl IntoIterator<Item = T>) -> SymbolDistribution<T>
where 
    T: Eq + Copy + Hash
{
    let mut count = 0;
    let mut occurances: HashMap<T, u64> = HashMap::new();
    for symbol in symbols.into_iter() {
        *occurances.entry(symbol).or_insert(0) += 1;
        count += 1;
    }
    for v in occurances.values_mut() {
        let occurs_float = *v as f64 / count as f64;
        *v = unsafe { std::mem::transmute(occurs_float) };
    }
    unsafe { std::mem::transmute(occurances) }
}

#[allow(dead_code)]
pub fn map_hashmap(map: HashMap<usize,u64>) -> HashMap<usize, f64> {
    unsafe { std::mem::transmute(map) }
}