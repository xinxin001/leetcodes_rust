fn main() {}

pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    match gas
        .into_iter()
        .zip(cost.into_iter())
        .map(|(g, c)| g - c)
        .enumerate()
        .fold((0, (0, 0)), |(s, pass @ (_, vm)), (i, v)| match s + v {
            s if s < vm => (s, (i as i32 + 1, s)),
            s => (s, pass),
        }) {
        (s, _) if s < 0 => -1,
        (_, (im, _)) => im,
    }
}

pub fn alt_can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut fuel = 0;
    let mut deficit = 0;
    let mut start = 0;

    for idx in 0..gas.len() {
        fuel += gas[idx] - cost[idx];

        if fuel < 0 {
            deficit += fuel;
            fuel = 0;
            start = idx + 1;
        }
    }

    if (fuel + deficit) >= 0 {
        return start as i32;
    }
    -1
}
