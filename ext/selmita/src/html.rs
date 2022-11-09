use magnus::{
    class, function, method, scan_args, scan_args::scan_args, Error, Module, Object, RClass, RHash,
    RModule, Value,
};

#[derive(Clone, Debug)]
#[magnus::wrap(class = "Selmita::HTML")]
pub(crate) struct SelmitaHTML {}

impl SelmitaHTML {
    fn rewrite(&self) -> String {
        String::from("wow!")
    }
}

pub fn init(m_selmita: RModule) -> Result<(), Error> {
    let c_html = m_selmita
        .define_class("HTML", Default::default())
        .expect("cannot find class Selmita::HTML");

    // c_html.define_singleton_method("new", function!(RubyHTML::new, -1))?;
    c_html
        .define_private_method("selmita_rewrite", method!(SelmitaHTML::rewrite, 0))
        .expect("cannot define private method: selmita_rewrite");

    // c_html.define_method("html", method!(SelmitaHTML::html, 0))?;

    // element::Init_selmita_element(c_HTML);
    // end_tag::Init_selmita_end_tag(c_HTML);

    Ok(())
}
