pub struct Movie {
    pub title: String,
}

impl Movie {
    pub fn new(title: &str) -> Self {
        Movie {
            title: title.to_string(),
        }
    }

    pub fn reconstruct(title: &str) -> Self {
        Movie {
            title: title.to_string(),
        }
    }
}

#[cfg(test)]
mod movie_tests {
    use super::*;

    #[test]
    fn test_new_movie() {
        let movie = Movie::new("Movie Title1");
        assert_eq!(movie.title, "Movie Title1");
    }

    #[test]
    fn test_reconstruct_movie() {
        let movie = Movie::new("Movie Title2");
        assert_eq!(movie.title, "Movie Title2");
    }
}
