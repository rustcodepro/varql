use std::error::Error;
/*
Gaurav Sablok
codeprog@icloud.com
*/

pub async fn paddna(inputvec: Vec<String>) -> Result<Vec<Vec<f32>>, Box<dyn Error>> {
    let inputvec = inputvec.clone();
    let mut convertdna: Vec<Vec<f32>> = Vec::new();
    for i in inputvec.iter() {
        let inputstring: Vec<char> = i.chars().collect::<Vec<_>>();
        let mut vecinput: Vec<Vec<f32>> = Vec::new();
        for i in inputstring.iter() {
            match i {
                'A' => vecinput.push(vec![1.0, 0.0, 0.0, 0.0]),
                'T' => vecinput.push(vec![0.0, 1.0, 0.0, 0.0]),
                'G' => vecinput.push(vec![0.0, 0.0, 1.0, 0.0]),
                'C' => vecinput.push(vec![0.0, 0.0, 0.0, 1.0]),
                'N' => vecinput.push(vec![0.0, 0.0, 0.0, 0.0]),
                _ => continue,
            }
        }
        convertdna.push(vecinput.iter().flatten().cloned().collect::<Vec<f32>>());
    }

    Ok(convertdna)
}

pub async fn paddnagc(inputvec: String) -> Result<f32, Box<dyn Error>> {
    let inputvecchars = inputvec.chars().collect::<Vec<char>>();
    let mut vecinput: Vec<f32> = Vec::new();
    for i in inputvecchars.iter() {
        let mut count_a = 0usize;
        let mut count_t: usize = 0usize;
        let mut count_g: usize = 0usize;
        let mut count_c: usize = 0usize;
        match i {
            'A' => count_a += 1usize,
            'T' => count_t += 1usize,
            'G' => count_g += 1usize,
            'C' => count_c += 1usize,
            _ => continue,
        }
        vecinput.push((count_g + count_c) as f32 / (count_a + count_c + count_g + count_t) as f32);
    }
    Ok(vecinput[0])
}
