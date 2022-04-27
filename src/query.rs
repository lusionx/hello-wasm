use anyhow::Result;
use kuchiki::traits::*;
use kuchiki::NodeRef;

fn html_as_node(html: &str) -> Result<NodeRef> {
    let root = kuchiki::parse_html()
        .from_utf8()
        .read_from(&mut html.as_bytes())?;
    Ok(root)
}

pub fn query_attr(html: &str, selecter: &str, attr: &str) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let doc = html_as_node(html).unwrap();

    for css_match in doc
        .select(selecter)
        .expect(format!("Failed to parse CSS {}", selecter).as_str())
    {
        if let Some(as_element) = css_match.as_node().as_element() {
            if let Ok(elem_atts) = as_element.attributes.try_borrow() {
                if let Some(val) = elem_atts.get(attr) {
                    res.push(val.to_string());
                }
            }
        }
    }
    res
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attr_ok() {
        let vv = query_attr("<a href=\"404\">xxx</a>", "a", "href");
        let v = vv.first().unwrap();
        assert_eq!("404", v);
    }
}
