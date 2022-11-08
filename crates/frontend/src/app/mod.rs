/*
 * Onion architecture: App can use domain and infrastructure.
 */

#[path = "../domain/mod.rs"]
mod domain;

#[path = "../infrastructure/mod.rs"]
mod drivers;
