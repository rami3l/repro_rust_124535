#![doc = doc_self!()]

macro_rules! doc_self {
    () => {
        "The Dandified YUM."
    };
}
use doc_self;

#[doc = doc_self!()]
pub fn dnf() {}
