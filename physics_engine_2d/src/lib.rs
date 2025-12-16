pub mod entity;
pub mod particle;
pub mod shape;
pub mod vector2;
pub mod animate;
pub mod physics_module;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}

