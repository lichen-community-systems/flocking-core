use merge::Merge;

pub fn merge_options<T>(defaults: &T, options: Option<&T>) -> T
    where T: Merge + Clone {

    // Clone the defaults so that we don't have
    // to move ownership of them.
    let right = defaults.clone();

    match options {
        Some(o) => {
            let mut left = o.clone();
            left.merge(right);
            return left
        },

        None => {
            return right
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Merge, Clone, Debug, PartialEq)]
    struct Basement {
        cat: Option<String>,
        beers: Option<i8>,
        guitar: Option<f32>,
        synthesizers: Option<Vec<String>>
    }

    #[test]
    fn test_merge() {
        let default_basement = Basement {
            cat: Some("A cat".to_string()),
            guitar: Some(2.0),
            beers: None,
            synthesizers: Some(vec!["RS-09".to_string()])
        };

        let colins_basement = Basement {
            cat: Some("Griffin".to_string()),
            guitar: None,
            beers: Some(1),
            synthesizers: Some(
                vec!["Eurorack".to_string(), "MS-20".to_string()]
            )
        };


        assert_eq!(default_basement,
            merge_options(&default_basement, None),
            "No options specified");

        let expected = Basement {
            cat: colins_basement.cat.clone(),
            guitar: default_basement.guitar,
            beers: colins_basement.beers,
            synthesizers: colins_basement.synthesizers.clone()
        };

        let actual = merge_options(&default_basement,
            Some(&colins_basement));
        assert_eq!(expected, actual,
            "Specified options replace defaults");
    }
}
