pub fn convert(array: &[u8]) -> Vec<String> {
    let mut arr: Vec<String> = Vec::new();
    for x in array {
        arr.push(format!("{:#04X?}", x));
    }
    return arr;
}
