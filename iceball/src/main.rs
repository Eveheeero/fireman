use lopdf::{content::Content, Document, Object};

fn main() {
    let doc = lopdf::Document::load("test.pdf").unwrap();
    dbg!(print_pages(&doc, 289));
    dbg!(print_pages(&doc, 290));
}

fn print_pages(doc: &Document, page: u32) -> Vec<String> {
    let binding = doc.get_pages();
    let page = binding.get(&page).unwrap();
    let page = doc.get_object(*page).unwrap();
    let page_items = page.as_dict().unwrap();
    let page_items = doc
        .get_object(page_items.get(b"Contents").unwrap().as_reference().unwrap())
        .unwrap()
        .as_stream()
        .unwrap();
    let page_items = Content::decode(&page_items.decompressed_content().unwrap()).unwrap();
    let mut result = Vec::new();
    page_items.operations.iter().for_each(|op| {
        if !op.operator.eq_ignore_ascii_case("Tj") {
            return;
        }
        let line = op
            .operands
            .iter()
            .map(|operand| {
                extract_string(&doc, operand)
                    .map(|c| match c {
                        b'\n' => b' ',
                        c => c,
                    })
                    .map(|x| x as u16)
                    .collect::<Vec<u16>>()
            })
            .flatten()
            .collect::<Vec<u16>>();
        let line = String::from_utf16_lossy(&line);
        // 특수문자 제거
        let line = line
            .replace("\u{92}", "'")
            .replace("\\\u{93}", "\"")
            .replace("\\\u{94}", "\"")
            .replace("\u{97}", "-")
            .replace("\u{8a}", "-");
        result.push(line);
    });
    // ignore header, footer, page number
    result[3..].to_vec()
}

fn extract_string<'obj>(
    doc: &'obj Document,
    obj: &'obj Object,
) -> Box<dyn Iterator<Item = u8> + 'obj> {
    match obj {
        Object::String(string, _) => Box::new(string.iter().copied()),
        Object::Null => Box::new(std::iter::empty()),
        // Object::Boolean(o) => Box::new(o.to_string().into_bytes().into_iter()),
        Object::Boolean(_) => Box::new(std::iter::empty()),
        // Object::Integer(o) => Box::new(o.to_string().into_bytes().into_iter()),
        Object::Integer(_) => Box::new(std::iter::empty()),
        // Object::Real(o) => Box::new(o.to_string().into_bytes()),
        Object::Real(_) => Box::new(std::iter::empty()),
        // Object::Name(i) => Box::new(i.iter().copied()),
        Object::Name(_) => Box::new(std::iter::empty()),
        Object::Array(o) => Box::new(o.iter().map(|o| extract_string(doc, o)).flatten()),
        Object::Dictionary(o) => Box::new(
            o.iter()
                .map(|(k, v)| {
                    let k = k.iter().copied().chain(std::iter::once(b'\t'));
                    let v = extract_string(doc, v);
                    k.chain(v)
                })
                .flatten(),
        ),
        Object::Stream(o) => Box::new(
            o.dict
                .iter()
                .map(|(k, v)| {
                    let k = k.iter().copied().chain(std::iter::once(b'\t'));
                    let v = extract_string(doc, v);
                    k.chain(v)
                })
                .flatten(),
        ),
        Object::Reference(o) => extract_string(doc, doc.get_object(*o).unwrap()),
    }
}
