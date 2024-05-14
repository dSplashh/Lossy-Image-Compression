mod array2;
pub use crate::array2::Array2;

#[cfg(test)]
mod tests {
    use crate::Array2;
    #[test]
    fn create_and_access() {
        let a = Array2::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        assert_eq!(*a.get(2, 0).unwrap(), 3);
    }

    #[test]
    fn modify_and_access() {
        let mut a = Array2::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let i = a.get_mut(2, 0).unwrap();
        *i = 99;
        assert_eq!(*a.get(2, 0).unwrap(), 99);
    }

    #[test]
    fn access_out_of_bounds() {
        let a = Array2::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let v = a.get(3, 0);
        assert_eq!(v, None);
    }
    #[test]
    fn access_in_bounds() {
        let a = Array2::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let v = a.get(2, 0);
        assert_eq!(v, Some(&3));
    }

    #[test]
    fn modify_out_of_bounds() {
        let mut a = Array2::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let v = a.get_mut(3, 0);
        assert_eq!(v, None);
    }

    #[test]
    fn sum_via_fold() {
        let a = Array2::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let v: i32 = a.iter_row_major().fold(0_i32, |acc, (_, _, x)| acc + x);
        assert_eq!(v, 21);
    }

    #[test]
    fn iter_row_major() {
        let a = Array2::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let mut b = a.iter_row_major();

        assert_eq!(b.next(), Some((0, 0, &1)));
        assert_eq!(b.next(), Some((1, 0, &2)));
        assert_eq!(b.next(), Some((2, 0, &3)));

        assert_eq!(b.next(), Some((0, 1, &4)));
        assert_eq!(b.next(), Some((1, 1, &5)));
        assert_eq!(b.next(), Some((2, 1, &6)));

        assert_eq!(b.next(), None);
    }

    #[test]
    fn iter_col_major() {
        let a = Array2::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let mut b = a.iter_col_major();

        assert_eq!(b.next(), Some((0, 0, &1)));
        assert_eq!(b.next(), Some((0, 1, &4)));

        assert_eq!(b.next(), Some((1, 0, &2)));
        assert_eq!(b.next(), Some((1, 1, &5)));

        assert_eq!(b.next(), Some((2, 0, &3)));
        assert_eq!(b.next(), Some((2, 1, &6)));

        assert_eq!(b.next(), None);
    }
}
