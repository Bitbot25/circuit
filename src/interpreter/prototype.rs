pub struct Prototype<'a> {
    properties: Vec<Prototype<'a>>,
    parent: &'a Prototype<'a>,
}