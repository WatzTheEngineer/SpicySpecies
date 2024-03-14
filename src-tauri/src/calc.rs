/// Calculate the Shannon entropy and Pielou's evenness index for the given statistics vector.
///
/// # Arguments
///
/// * `stats_vector` - A vector of vectors containing the statistics data.
///
/// # Returns
///
/// A vector containing two strings: Shannon entropy and Pielou's evenness index.
pub fn shannon(stats_vector: Vec<Vec<u32>>) -> Vec<String> {
    let n = totalcount(stats_vector.clone()) as f32;

    let mut aggregated_values = vec![0u32; stats_vector[0].len()];
    for row in &stats_vector {
        for (i, &number) in row.iter().enumerate() {
            aggregated_values[i] += number;
        }
    }

    let h = aggregated_values.iter().filter(|&&sum| sum != 0).map(|&sum| {
        let pi = sum as f32 / n;
        pi * f32::log2(pi)
    }).sum::<f32>().abs();

    let p = (h / f32::log2(n) * 100.0).abs();

    vec![format!("{:.4}", h), format!("{:.2}", p)]
}

/// Calculate the Jaccard similarity coefficient for the given statistics vector.
///
/// # Arguments
///
/// * `stats_vector` - A vector of vectors containing the statistics data.
///
/// # Returns
///
/// A vector of strings containing Jaccard similarity coefficients.
pub fn jaccard(stats_vector: Vec<Vec<u32>>) -> Vec<String> {
    let mut coefs = Vec::new();

    for i in 0..(stats_vector.len() - 1) {
        for j in i + 1..=(stats_vector.len() - 1) {
            coefs.push(format!("{:.2}", presence_counter(&stats_vector[i], &stats_vector[j])));
        }
    }

    coefs
}

/// Calculate the Dice coefficient for the given statistics vector.
///
/// # Arguments
///
/// * `stats_vector` - A vector of vectors containing the statistics data.
///
/// # Returns
///
/// A vector of strings containing Dice coefficients.
pub fn dice(stats_vector: Vec<Vec<u32>>) -> Vec<String> {
    let mut coefs = Vec::new();

    for i in 0..(stats_vector.len() - 1) {
        for j in i + 1..=(stats_vector.len() - 1) {
            let abc = sor_abc(&stats_vector[i], &stats_vector[j]);
            let q = 2.0 * abc[2] as f32 / (abc[0] + abc[1]) as f32;
            coefs.push(format!("{:.4}", q));
        }
    }

    coefs
}

/// Count the presence of elements between two vectors.
///
/// # Arguments
///
/// * `firstvec` - The first vector.
/// * `secondvec` - The second vector.
///
/// # Returns
///
/// The presence count as a percentage.
pub fn presence_counter(firstvec: &[u32], secondvec: &[u32]) -> f32 {
    let (mut a, mut b, mut c) = (0, 0, 0);

    for i in 0..firstvec.len() {
        if firstvec[i] > 0 { a += 1 }
        if secondvec[i] > 0 { b += 1 }
        if firstvec[i] > 0 && secondvec[i] > 0 { c += 1 }
    }

    (c as f32 / (a + b - c) as f32) * 100.0
}

/// Calculate A, B, and C values for the Sørensen–Dice coefficient.
///
/// # Arguments
///
/// * `firstvec` - The first vector.
/// * `secondvec` - The second vector.
///
/// # Returns
///
/// A vector containing A, B, and C values.
pub fn sor_abc(firstvec: &[u32], secondvec: &[u32]) -> Vec<u32> {
    let (mut a, mut b, mut c) = (0, 0, 0);

    for i in 0..firstvec.len() {
        a += firstvec[i];
        b += secondvec[i];
        c += std::cmp::min(firstvec[i], secondvec[i]);
    }

    vec![a, b, c]
}

/// Calculate the total count of elements in the statistics vector.
///
/// # Arguments
///
/// * `stats_vector` - A vector of vectors containing the statistics data.
///
/// # Returns
///
/// The total count of elements in the vector.
pub fn totalcount(stats_vector: Vec<Vec<u32>>) -> u32 {
    stats_vector.iter().flat_map(|row| row.iter()).sum()
}

/// Calculate the Simpson's Diversity Index for the given statistics vector.
///
/// # Arguments
///
/// * `stats_vector` - A vector of vectors containing the statistics data.
///
/// # Returns
///
/// A vector containing Simpson's Diversity Index.
pub fn simpson(stats_vector: Vec<Vec<u32>>) -> Vec<String> {
    let n = totalcount(stats_vector.clone()) as f32;

    let lambda = stats_vector.iter().flat_map(|row| {
        row.iter().map(move |&number| {
            let pi = number as f32 / n;
            pi * pi
        })
    }).sum::<f32>() * 100.0;

    vec![format!("{:.2}", lambda)]
}



#[test]
fn sor_abc_test(){
    let firstvector = vec![5,0,10,5];
    let secondvector = vec![5,5,5,10];

    let abc = sor_abc(&firstvector, &secondvector);

    assert_eq!(abc[0],20);
    assert_eq!(abc[1],25);
    assert_eq!(abc[2],15);
}

#[test]
fn totalcount_test(){
    let stats_vector: Vec<Vec<u32>> = vec![vec![5,10,20,0],vec![5,12,8,5],vec![1,2,3,4]];

    assert_eq!((75),totalcount(stats_vector));
}

#[test]
fn dice_test(){
    let stats_vector: Vec<Vec<u32>> = vec![vec![5,10,20,0],vec![5,12,8,5],vec![1,2,3,4]];
    let expected: Vec<String> = vec![format!("0.7077"),format!("0.2667"),format!("0.5000")];

    let dice = dice(stats_vector);
    assert_eq!(expected[0],dice[0]);
    assert_eq!(expected[1],dice[1]);
    assert_eq!(expected[2],dice[2]);
}

#[test]
fn shannon_test(){
    let stats_vector: Vec<Vec<u32>> = vec![vec![5,10,20,0],vec![5,12,8,5],vec![1,2,3,4]];
    let expected_shannon: String = format!("1.8261");
    let expected_pielou: String = format!("29.32");

    assert_eq!(expected_shannon,shannon(stats_vector.clone())[0]);
    assert_eq!(expected_pielou,shannon(stats_vector)[1]);
}

#[test]
fn simpson_test(){
    let stats_vector: Vec<Vec<u32>> = vec![vec![5,10,20,0],vec![5,12,8,5],vec![1,2,3,4]];
    let expected: String = format!("14.45");

    assert_eq!(simpson(stats_vector)[0],expected);
}

#[test]
fn jaccard_test(){
    let stats_vector: Vec<Vec<u32>> = vec![vec![5,10,20,0],vec![5,12,8,5],vec![0,0,0,4]];
    let expected: Vec<String> = vec![format!("75.00"),format!("0.00"),format!("25.00")];

    assert_eq!(expected, jaccard(stats_vector))
}

#[test]
fn presence_counter_test(){
    let firstvec: Vec<u32> = vec![5,12,8,5];
    let secondvec: Vec<u32> = vec![0,0,0,4];

    assert_eq!(25,presence_counter(&firstvec, &secondvec) as u32)
}