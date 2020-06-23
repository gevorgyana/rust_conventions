// Conventions for expressing errors in Rust with Result<T, E>

mod A {
    use super::B;

    // --- Rule 1
    // This error type is specific to the module A. Allows expressing
    // B's errors, as well as those relevant to A.
    pub enum Error { // no need for `A` in enum name, the module tells us where
        // it comes from
        Reason, // no need for `A` in the name of enum variants, the
        // module and enum tell us where they come from

        B(B::Error), // A wraps the functionality of
        // B, so define the variant that accepts its error types, and
        // name the variant accordingly
        // it __must__ be names `B`
    }

    // --- Rule 2
    pub trait CommonBehaviour {
        // Implementors will be restricted to the set of errors defined in
        // CommonError.
        //
        // If you need more,
        //   the implementors's behaviour is more complex and not really
        //   covered by CommonBehaviour,
        // If you need less,
        //   CommonBehaviour is too general.
        fn common() -> Result<CommonResult, CommonError>;
    }

    // There is no restriction on where the types any trait mentions must be
    // defined, here they are defined in the same module as the trait.
    pub struct CommonResult {}
    pub enum CommonError {
        Abc
    }

    fn wrapping() -> Result<u8, Error> {
        match B::work() {
            Err(e) => {
                // Choose if you want to reveal who originated the error.
                // Showing this information may be not relevant in some use
                // cases, e.g. the end user is not required to know the
                // the exact module that originated the error, but library code
                // may need this information for internal logic
                if (1 == 1) {
                    Err(Error::B(e))
                } else {
                    Err(Error::Reason)
                }
            },
            Ok(val) => {
                Ok(val)
            }
        }
    }
}

mod B {
    use super::A;

    pub enum Error {
        // no need to specify where the error originated, module and enum
        // serve this purpose
        Reason
    }

    pub fn work() -> Result<u8, Error> {
        Err(Error::Reason)
    }

    pub struct Concrete {}
    impl A::CommonBehaviour for Concrete {
        fn common() -> Result<A::CommonResult, A::CommonError> {
            Err(A::CommonError::Abc)
        }
    }
}

fn main() {

}
