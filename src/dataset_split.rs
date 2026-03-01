use rand::seq::SliceRandom;

pub fn split<T: Clone>(data: Vec<T>, ratio: f32) -> (Vec<T>, Vec<T>) {
    let mut d = data.clone();
    let mut rng = rand::thread_rng();
    d.shuffle(&mut rng);

    let split = (d.len() as f32 * ratio) as usize;
    (d[..split].to_vec(), d[split..].to_vec())
}