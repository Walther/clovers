use clovers::{
    random::{
        random_cosine_direction, random_in_unit_disk, random_on_hemisphere, random_unit_vector,
    },
    Vec2, Vec3,
};
use plotly::{
    common::{Marker, Mode},
    Layout, Plot, Scatter, Scatter3D,
};
use rand::{rngs::SmallRng, SeedableRng};

fn main() {
    let mut rng: SmallRng = SmallRng::from_os_rng();
    plot_random_in_unit_disk(&mut rng);
    plot_random_unit_vector(&mut rng);
    plot_random_cosine_direction(&mut rng);
    plot_random_on_hemisphere(&mut rng);
}

fn plot_random_in_unit_disk(rng: &mut SmallRng) {
    let mut plot = Plot::new();
    plot.set_layout(
        Layout::new()
            .width(1000)
            .height(1000)
            .title("random in unit disk"),
    );
    let mut coordinates: Vec<Vec2> = Vec::new();

    for _ in 0..10_000 {
        let coordinate: Vec2 = random_in_unit_disk(rng);
        coordinates.push(coordinate);
    }
    let scatter = Scatter::new(
        coordinates.iter().map(|c| c.x).collect(),
        coordinates.iter().map(|c| c.y).collect(),
    )
    .mode(Mode::Markers)
    .marker(Marker::new().size(4));
    plot.add_trace(scatter);
    plot.write_html("plots/random_in_unit_disk.html");
}

fn plot_random_unit_vector(rng: &mut SmallRng) {
    let mut plot = Plot::new();
    plot.set_layout(
        Layout::new()
            .width(1000)
            .height(1000)
            .title("random unit vector"),
    );
    let mut coordinates: Vec<Vec3> = Vec::new();

    for _ in 0..10_000 {
        let coordinate: Vec3 = *random_unit_vector(rng);
        coordinates.push(coordinate);
    }
    let scatter = Scatter3D::new(
        coordinates.iter().map(|c| c.x).collect(),
        coordinates.iter().map(|c| c.y).collect(),
        coordinates.iter().map(|c| c.z).collect(),
    )
    .mode(Mode::Markers)
    .marker(Marker::new().size(2));
    plot.add_trace(scatter);
    plot.write_html("plots/random_unit_vector.html");
}

fn plot_random_cosine_direction(rng: &mut SmallRng) {
    let mut plot = Plot::new();
    plot.set_layout(
        Layout::new()
            .width(1000)
            .height(1000)
            .title("random cosine direction"),
    );
    let mut coordinates: Vec<Vec3> = Vec::new();

    for _ in 0..10_000 {
        let coordinate: Vec3 = *random_cosine_direction(rng);
        coordinates.push(coordinate);
    }
    let scatter = Scatter3D::new(
        coordinates.iter().map(|c| c.x).collect(),
        coordinates.iter().map(|c| c.y).collect(),
        coordinates.iter().map(|c| c.z).collect(),
    )
    .mode(Mode::Markers)
    .marker(Marker::new().size(2));
    plot.add_trace(scatter);
    plot.write_html("plots/random_cosine_direction.html");
}

fn plot_random_on_hemisphere(rng: &mut SmallRng) {
    let mut plot: Plot = Plot::new();
    plot.set_layout(
        Layout::new()
            .width(1000)
            .height(1000)
            .title("random on hemisphere"),
    );
    let mut coordinates: Vec<Vec3> = Vec::new();
    let normal = Vec3::new(0.0, 0.0, -1.0);

    for _ in 0..10_000 {
        let coordinate: Vec3 = *random_on_hemisphere(normal, rng);
        coordinates.push(coordinate);
    }
    let scatter = Scatter3D::new(
        coordinates.iter().map(|c| c.x).collect(),
        coordinates.iter().map(|c| c.y).collect(),
        coordinates.iter().map(|c| c.z).collect(),
    )
    .mode(Mode::Markers)
    .marker(Marker::new().size(2));
    plot.add_trace(scatter);
    plot.write_html("plots/random_on_hemisphere.html");
}
