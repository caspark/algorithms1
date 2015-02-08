use test::Bencher;
use percolation;


#[bench]
fn simulate_percolation_of_size_10(b: &mut Bencher) {
    b.iter(|| percolation::simulate(100));
}

#[bench]
fn simulate_percolation_of_size_100(b: &mut Bencher) {
    b.iter(|| percolation::simulate(100));
}

#[bench]
fn simulate_percolation_of_size_200(b: &mut Bencher) {
    b.iter(|| percolation::simulate(200));
}

#[bench]
fn simulate_10_percolations_of_size_200_in_1_job(b: &mut Bencher) {
    b.iter(|| percolation::simulate_multiple(200, 10, 1));
}

#[bench]
fn simulate_10_percolations_of_size_200_in_4_jobs(b: &mut Bencher) {
    b.iter(|| percolation::simulate_multiple(200, 10, 4));
}

#[bench]
fn simulate_10_percolations_of_size_200_in_8_jobs(b: &mut Bencher) {
    b.iter(|| percolation::simulate_multiple(200, 10, 8));
}
