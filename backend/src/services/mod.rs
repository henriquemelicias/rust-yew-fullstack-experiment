/*
 * Onion architecture: Services can use domain and drivers.
 */

#[path = "../domain/mod.rs"]
mod domain;

#[path = "../drivers/mod.rs"]
mod drivers;

// Modules.
pub mod controllers;
pub mod validators;
