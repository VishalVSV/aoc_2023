pub trait StringUtils {
    fn split_into(&self, pat: char) -> Result<(&str, &str), ()>;
}

impl StringUtils for String {
    fn split_into(&self, pat: char) -> Result<(&str, &str), ()> {
        let mut s = self.split(pat);
        if let Some(a) = s.next() {
            if let Some(b) = s.next() {
                Ok((a,b))
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

impl StringUtils for &str {
    fn split_into(&self, pat: char) -> Result<(&str, &str), ()> {
        let mut s = self.split(pat);
        if let Some(a) = s.next() {
            if let Some(b) = s.next() {
                Ok((a,b))
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}
