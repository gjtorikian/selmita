extern crate core;

use magnus::{define_module, Error, Module};

pub mod html;

#[magnus::init]
fn init() -> Result<(), Error> {
    let m_selmita = define_module("Selmita").expect("cannot define ::Selmita module");

    html::init(m_selmita);
    // sanitizer::init(m_selmita);
    // selector::Init_selmita_selector(m_selmita);
    // rewriter::Init_selmita_rewriter(m_selmita);

    Ok(())
}
