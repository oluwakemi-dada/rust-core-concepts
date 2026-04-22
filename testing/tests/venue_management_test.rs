use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};

use testing::attractions::{MovieTheatre, Museum};
use testing::management::VenueManagement;

#[fixture]
fn museum_with_three_paintings() -> Museum {
    let mut museum = Museum::new();
    museum.buy_painting("Mona Lisa");
    museum.buy_painting("The Starry Night");
    museum.buy_painting("Girl with a Pearl Earring");
    museum
}

#[fixture]
fn museum_management(museum_with_three_paintings: Museum) -> VenueManagement<Museum> {
    VenueManagement::new(museum_with_three_paintings)
}

#[fixture]
fn movie_theatre_with_one_movie() -> MovieTheatre {
    let mut movie_theatre = MovieTheatre::new();
    movie_theatre.add_movie("Titanic");
    movie_theatre
}

#[fixture]
fn movie_theatre_management(
    movie_theatre_with_one_movie: MovieTheatre,
) -> VenueManagement<MovieTheatre> {
    VenueManagement::new(movie_theatre_with_one_movie)
}

#[rstest]
fn venue_management_interacts_with_museum_venue(museum_with_three_paintings: Museum) {
    let mut venue_mgmt = VenueManagement::new(museum_with_three_paintings);
    venue_mgmt.make_money();

    assert_eq!(venue_mgmt.venue.paintings.len(), 3);
    assert_eq!(venue_mgmt.venue.revenue, 25);
}

#[rstest]
fn venue_management_interacts_with_movie_theatre_venue(mut movie_theatre_management: VenueManagement<MovieTheatre>){
movie_theatre_management.make_money();
assert_eq!(movie_theatre_management.venue.sales, 15);
}
