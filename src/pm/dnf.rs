#![doc = doc_self!()]

macro_rules! doc_self {
    () => {
        "The [Dandified YUM](https://github.com/rpm-software-management/dnf)."
    };
}
use doc_self;

#[doc = doc_self!()]
pub struct Dnf {}
