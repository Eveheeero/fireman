use lopdf::{content::Content, Document, Object};

fn main() {
    let doc = lopdf::Document::load("test.pdf").unwrap();
    let binding = doc.get_pages();
    let page = binding.get(&289).unwrap();
    // let _ = dbg!(doc.get_and_decode_page_content(*page));
    println!("{:?}", &page);
    let page = doc.get_object(*page).unwrap();
    let page_items = dbg!(page.as_dict().unwrap());
    page_items.iter().for_each(|(k, v)| {
        println!("{:?} {:?}", std::str::from_utf8(k).unwrap(), v);
    });
    let stream = doc
        .get_object(page_items.get(b"Contents").unwrap().as_reference().unwrap())
        .unwrap()
        .as_stream()
        .unwrap();
    let contents = Content::decode(&stream.decompressed_content().unwrap()).unwrap();
    contents.operations.iter().for_each(|op| {
        // println!("{:} -----", op.operator);
        let w = op
            .operands
            .iter()
            .map(|operand| {
                extract_string(&doc, operand)
                    .map(|c| match c {
                        b'\n' => b' ',
                        c => c,
                    })
                    .collect::<Vec<u8>>()
            })
            .flatten()
            .collect::<Vec<u8>>();
        unsafe { println!("{}", std::str::from_utf8_unchecked(&w)) };
    });
}

fn extract_string<'obj>(
    doc: &'obj Document,
    obj: &'obj Object,
) -> Box<dyn Iterator<Item = u8> + 'obj> {
    match obj {
        Object::String(string, _) => {
            Box::new(string.iter().copied().collect::<Vec<u8>>().into_iter())
        }
        Object::Null => Box::new(std::iter::empty()),
        // Object::Boolean(o) => Box::new(o.to_string().into_bytes().into_iter()),
        Object::Boolean(_) => Box::new(std::iter::empty()),
        // Object::Integer(o) => Box::new(o.to_string().into_bytes().into_iter()),
        Object::Integer(_) => Box::new(std::iter::empty()),
        // Object::Real(o) => Box::new(o.to_string().into_bytes()),
        Object::Real(_) => Box::new(std::iter::empty()),
        // Object::Name(i) => Box::new(i.iter().copied().collect::<Vec<u8>>().into_iter()),
        Object::Name(_) => Box::new(std::iter::empty()),
        Object::Array(o) => Box::new(
            o.iter()
                .map(|o| extract_string(doc, o))
                .flatten()
                .collect::<Vec<u8>>()
                .into_iter(),
        ),
        Object::Dictionary(o) => Box::new(
            o.iter()
                .map(|(k, v)| {
                    let k = k.iter().copied();
                    let v = extract_string(doc, v);
                    k.chain(v).collect::<Vec<u8>>().into_iter()
                })
                .flatten()
                .collect::<Vec<u8>>()
                .into_iter(),
        ),
        Object::Stream(o) => Box::new(
            o.dict
                .iter()
                .map(|(k, v)| {
                    let k = k.iter().copied();
                    let v = extract_string(doc, v);
                    k.chain(v).collect::<Vec<u8>>().into_iter()
                })
                .flatten()
                .collect::<Vec<u8>>()
                .into_iter(),
        ),
        Object::Reference(o) => extract_string(doc, doc.get_object(*o).unwrap()),
    }
}
