/*
 * Onion architecture: Services can use domain and infrastructure.
 */

#[path = "../domain/mod.rs"]
mod domain;

#[path = "../drivers/mod.rs"]
mod drivers;

// Modules.
pub mod controllers;
pub mod validators;
