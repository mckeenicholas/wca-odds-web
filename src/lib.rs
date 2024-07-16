use wasm_bindgen::prelude::*;
use rand::Rng;
use std::f64::consts::PI;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

fn random_normals() -> (f64, f64) {
    let mut rng = rand::thread_rng();
    let (mut u1, mut u2): (f64, f64) = (0.0, 0.0);

    while u1 == 0.0 {
        u1 = rng.gen();
    }
    while u2 == 0.0 {
        u2 = rng.gen();
    }

    let r = (-2.0 * u1.ln()).sqrt();
    let o = 2.0 * PI * u2;
    (r * o.cos(), r * o.sin())
}

fn random_skew_normal(xi: i32, omega: i32, alpha: f64, delta: f64) -> i32 {
    let (u0, v) = random_normals();
    if alpha == 0.0 {
        return xi + omega * u0.round() as i32;
    }
    let u1 = (delta * u0 + (1.0 - delta.powi(2)).sqrt() * v) as i32;
    let z = if u1 >= 0 { u1 } else { -u1 };
    xi + omega * z
}

fn simulate() {
    let data: Vec<i32> = vec![
        665, 572, 527, 698, 526, 550, 580, 540, 486, 649, 556, 535, 526, 475, 516, 540, 552, 582,
        531, 415, 507, 708, 514, 618, 539, 729, 553, 518, 534, 532, 555, 588, 595, 483, 721, 661,
        476, 500, 454, 649, 669, 505, 671, 466, 501, 673, 634, 562, 575, 505, 517, 505, 557, 694,
        516, 504, 675, 665, 510, 554, 486, 767, 588, 537, 558, 501, 462, 463, 523, 562, 530, 515,
        512, 572, 576, 571, 485, 506, 555, 829, 664, 413, 458, 723, 686, 592, 584, 459, 540, 493,
        786, 526, 583, 965, 551, 640, 741, 564, 651, 655, 705, 521, 512, 573, 590, 550, 548, 554,
        856, 532, 564, 591, 519, 526, 511, 543, 540, 494, 423, 568, 524, 731, 770, 668, 567, 597,
        559, 538, 646, 598, 565, 487, 593, 602, 633, 592, 611, 585, 538, 404, 589, 484, 378, 596,
        612, 574, 698, 539, 442, 579, 722, 582, 570, 624, 458, 466, 779, 706, 493, 748, 630, 637,
        620, 714, 637, 560, 633, 519, 564, 450, 519, 446, 567, 643, 847, 616, 481, 638, 464, 614,
        730, 599, 620, 511, 504, 604, 392, 535, 421, 574, 571, 501, 472, 485, 544, 593, 581, 648,
        566, 526, 429, 487, 489, 693, 728, 510, 483, 570, 624, 510, 539, 584, 776, 537, 532, 539,
        471, 627, 863, 488, 367, 463, 561, 689, 559, 626, 651, 545, 547, 488, 546, 534, 603, 593,
        697, 439, 652, 760, 531, 366, 544, 630, 686, 557, 693, 725, 508, 580, 565, 505, 600, 572,
        482, 561, 461, 577, 534, 551, 878, 672, 550, 596, 585, 683, 488, 472, 382, 543, 579, 533,
        545, 710, 630, 528, 498, 455, 581, 662, 668, 426, 572, 642, 490, 450, 503, 413, 518, 417,
        516, 527, 458, 544, 480, 503, 550, 414, 516, 494, 536, 599, 525, 581, 478, 599, 517, 535,
        596, 526, 531, 532, 587, 549, 581, 510, 462, 661, 568, 511, 561, 543, 562, 499, 495, 667,
        494, 499, 655, 447, 523, 441, 508, 581, 701, 428, 519, 602, 586, 549, 530, 542, 590, 563,
        512, 511, 567, 379, 437, 485, 533, 501, 645, 602, 775, 625, 489, 643, 652, 578, 619, 492,
        570, 488, 590, 574, 540, 678, 447, 594, 400, 575, 571, 636, 549, 610, 481, 548, 625, 572,
        528, 538, 590, 522, 480, 585, 479
    ];

    let attempts = 5;
    let simulations = 100000;

    let mu: f64 = data.iter().copied().sum::<i32>() as f64 / data.len() as f64;
    let sigma: f64 = (data.iter().map(|&x| (x as f64 - mu).powi(2)).sum::<f64>() / data.len() as f64).sqrt();

    let a: f64 = data.iter().map(|&x| ((x as f64 - mu) / sigma).powi(3)).sum::<f64>() / data.len() as f64;

    let gamma = a.abs().min(0.99);

    let delta = (gamma.powf(2.0 / 3.0) / (gamma.powf(2.0 / 3.0) + ((4.0 - PI) / 2.0).powf(2.0 / 3.0) * (PI / 2.0))).sqrt();

    let alpha = delta / (1.0 - delta.powi(2)).sqrt();
    let omega = (sigma / (1.0 - (2.0 * delta.powi(2) / PI)).sqrt()).round() as i32;
    let xi = (mu - omega as f64 * delta * (2.0 / PI).sqrt()).round() as i32;

    let dist: Vec<i32> = (0..attempts * simulations)
        .map(|_| random_skew_normal(xi, omega, alpha, delta))
        .collect();

    let mut dist_np: Vec<Vec<i32>> = Vec::with_capacity(simulations);
    for i in 0..simulations {
        dist_np.push(dist[i * attempts..(i + 1) * attempts].to_vec());
    }

    let dist_sort: Vec<Vec<i32>> = dist_np
        .iter()
        .map(|row| {
            let mut sorted_row = row.clone();
            sorted_row.sort();
            sorted_row
        })
        .collect();

    let dist_trimmed: Vec<Vec<i32>> = dist_sort
        .iter()
        .map(|row| row[1..4].to_vec())
        .collect();

    let dist_avg: Vec<f64> = dist_trimmed
        .iter()
        .map(|row| row.iter().map(|&x| x as f64).sum::<f64>() / row.len() as f64)
        .collect();

}

#[wasm_bindgen]
pub fn generate() {
    simulate();
}
