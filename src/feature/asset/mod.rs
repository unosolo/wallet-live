mod repository;

pub mod controller;
pub mod model;
pub mod service;

pub use self::repository::AssetRepository;

#[cfg(test)]
pub mod controller_tests;
