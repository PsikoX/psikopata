pub const CSS: &str = concat!(
    include_str!("site.css"),
    "\n",
    include_str!("motion.css"),
    "\n",
    include_str!("fer.css"),
    "\n",
    include_str!("fer_artifacts.css")
);

/// A changed stylesheet gets a new browser cache key after publication.
pub fn css_href() -> String {
    use std::hash::{DefaultHasher, Hash, Hasher};
    let mut hash = DefaultHasher::new();
    CSS.hash(&mut hash);
    format!("/assets/site.css?v={:016x}", hash.finish())
}
