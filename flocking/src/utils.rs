use merge::Merge;

pub fn merge<T>(defaults: &T, options: Option<&T>) -> T
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
