pub mod Cstam {

    use std::collections::HashSet;

    use regex::Regex;
    pub fn process_dir(fpath: String) {}
    pub fn process_file(fname: String) {
        if let Ok(doc) = std::fs::read_to_string(fname) {
            let mut start = 0;
            let l = doc.len();
            let bytes = doc.as_bytes();

            let re = Regex::new(
                r"(?m)^\s*(?:[A-Za-z_][A-Za-z0-9_\s\*]+)\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(",
            )
            .unwrap();
            let mut async_fns: Vec<String> = vec![];
            for i in re.captures_iter(&doc) {
                async_fns.push(i[1].to_string());
            }
            let mut is_async: HashSet<_> = async_fns.clone().iter().collect();
            let mut start = 0;
            while start < l {
                let mut pos = match doc[start..].find("async") {
                    Some(x) => x,
                    None => l,
                };
                if pos >= l {
                    break;
                }

                let rel_off = match doc[pos..].find("{") {
                    Some(x) => x,
                    None => l,
                };
                if rel_off >= l {
                    break;
                }
                let mut stack = 1;
                let mut k = pos + rel_off;
                while stack != 0 && k <= l {
                    match bytes[k] {
                        b'{' => stack += 1,
                        b'}' => stack -= 1,
                        _ => {}
                    }
                    k += 1;
                }
                let (fn_start, fn_end) = (pos + rel_off, k);
                let fn_body: String = bytes[fn_start..fn_end].iter().collect();
            }
        }
    }
}
